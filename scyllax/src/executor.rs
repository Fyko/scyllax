//! The `scyllax` [`Executor`] processes queries.
use crate::{
    collection::QueryCollection,
    error::ScyllaxError,
    prelude::WriteQuery,
    queries::{Query, ReadQuery},
};
use scylla::{prepared_statement::PreparedStatement, QueryResult, Session, SessionBuilder};
use std::{collections::HashMap, sync::Arc};
use tokio::{sync::{mpsc::{Receiver, Sender, self}, MutexGuard, Mutex}, task::JoinSet};
use tokio::sync::oneshot;

/// Creates a new [`CachingSession`] and returns it
pub async fn create_session(
    known_nodes: impl IntoIterator<Item = impl AsRef<str>>,
    default_keyspace: Option<impl Into<String>>,
) -> anyhow::Result<Session> {
    let session = SessionBuilder::new()
        .known_nodes(known_nodes)
        .build()
        .await?;

    if let Some(ks) = default_keyspace {
        session.use_keyspace(ks, true).await?;
    }

    Ok(session)
}

pub trait GetPreparedStatement<T: Query> {
    fn get(&self) -> &PreparedStatement;
}

pub trait GetCoalescingSender<T: Query + ReadQuery> {
    fn get(&self) -> &Sender<ShardMessage<T>>;
}

#[derive(Debug, Clone)]
pub struct Executor<T> {
    pub session: Arc<Session>,
    queries: T,
}

pub type ShardMessage<Q> = (
    Q,
    oneshot::Sender<ReadQueryResult<Q>>,
);
type TaskRequestMap<Q> =
    HashMap<String, Vec<oneshot::Sender<ReadQueryResult<Q>>>>;

type ReadQueryResult<Q> = Arc<Result<<Q as ReadQuery>::Output, ScyllaxError>>;

pub struct QueryRunnerMessage<Q: ReadQuery> {
    pub key: String,
    pub query: Q,
    pub response_rx: oneshot::Sender<ReadQueryResult<Q>>,
}

impl<T: QueryCollection + Clone + Send + Sync + 'static> Executor<T> {
    pub async fn new(session: Arc<Session>) -> Result<Self, ScyllaxError> {
        let queries = T::new(&session).await?;
        let executor = Arc::new(Self {
            session: session.clone(),
            queries,
        });

        let queries = executor.queries.clone().register_tasks(executor);
        // let queries = Arc::new(queries);
        let executor = Self { session, queries };

        Ok(executor)
    }

    pub async fn execute_read<Q>(&self, query: Q) -> Result<Q::Output, ScyllaxError>
    where
        Q: Query + ReadQuery,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let (response_tx, response_rx) = oneshot::channel();
        let task = self.queries.get_task::<Q>();

        match task.send((query, response_tx)).await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("error sending query to task: {:#?}", e);
                return Err(ScyllaxError::NoRowsFound);
            },
        }

        match response_rx.await {
            Ok(result) => {
                let result = Arc::try_unwrap(result).unwrap();
                result
            },
            Err(e) => Err(ScyllaxError::ReceiverError(e)),
        }
    }

    /// the read task is responsible for coalescing requests
    pub async fn read_task<Q>(&self, mut request_receiver: Receiver<ShardMessage<Q>>, query_runner: Sender<QueryRunnerMessage<Q>>)
    where
        Q: Query + ReadQuery + Send + Sync + 'static,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let mut requests: TaskRequestMap<Q> = HashMap::new();
        let mut join_set: JoinSet<_> = JoinSet::new();

        loop {
            tokio::select! {
                Some((query, tx)) = request_receiver.recv() => {
                    tracing::debug!("read_task recieved a request!");
                    let key = query.shard_key();
                    if let Some(senders) = requests.get_mut(&key) {
                        tracing::debug!("key:{key} already has a request, adding to senders");
                        senders.push(tx);
                    } else {
                        tracing::debug!("key:{key} is new! creating a new request");
                        requests.insert(key.clone(), vec![tx]);
                        let (response_transmitter, response_receiver) = oneshot::channel();
                        // let _ = runner.send(QueryRunnerMessage { key: key.clone(), query, response_rx }).await;
                        let _ = query_runner.send(QueryRunnerMessage { key: key.clone(), query, response_rx: response_transmitter });

                        join_set.spawn(async {
                            let res = match response_receiver.await {
                                Ok(result) => result,
                                Err(e) => Arc::new(Err(ScyllaxError::ReceiverError(e))),
                            };
                            tracing::debug!("joinset handle returned: {:#?}", res);

                            (key, res)
                        });
                    }
                },
                Some(join_handle) = join_set.join_next() => {
                    tracing::debug!("join set recieved a result!");
                    if let Ok((key, result)) = join_handle {
                        if let Some(senders) = requests.remove(&key) {
                            for sender in senders {
                                let _ = sender.send(result.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    /// this function does the requests themselves
    // async fn perform_read_query<Q>(session: Arc<Session>, statement: &PreparedStatement, query: Q) -> Result<<Q as ReadQuery>::Output, ScyllaxError>
    pub async fn read_query_runner<Q>(&self, mut query_receiver: Receiver<QueryRunnerMessage<Q>>)
    where
        Q: Query + ReadQuery + Send + Sync,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        if let Some(QueryRunnerMessage { query, response_rx, key }) = query_receiver.recv().await {
            tracing::info!("running query for key: {key}");
            let statement = self.queries.get_prepared::<Q>();
            let variables = query.bind().unwrap();
            let response = match self.session.execute(statement, variables).await {
                Ok(response) => {
                    tracing::info!("query executed successfully: {:?} rows", response.rows_num());
                    response
                },
                Err(e) => {
                    tracing::error!("error executing query: {:#?}", e);
                    let _ = response_rx.send(Arc::new(Err(e.into())));
                    return;
                },
            };
            let parsed = Q::parse_response(response).await;
            let result = Arc::new(parsed);

            let _ = response_rx.send(result.clone());
        }
    }

    pub async fn execute_write<Q>(&self, query: &Q) -> Result<QueryResult, ScyllaxError>
    where
        Q: Query + WriteQuery,
        T: GetPreparedStatement<Q>,
    {
        let statement = self.queries.get_prepared::<Q>();
        let variables = query.bind()?;

        self.session
            .execute(statement, variables)
            .await
            .map_err(Into::into)
    }
}

impl<T: QueryCollection> std::fmt::Display for Executor<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.session)
    }
}
