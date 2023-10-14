//! The `scyllax` [`Executor`] processes queries.
use crate::{
    collection::QueryCollection,
    error::ScyllaxError,
    prelude::WriteQuery,
    queries::{Query, ReadQuery},
};
use scylla::{prepared_statement::PreparedStatement, QueryResult, Session, SessionBuilder};
use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
    sync::Arc,
};
use tokio::sync::{mpsc::error::TrySendError, oneshot};
use tokio::{
    sync::mpsc::{Receiver, Sender},
    task::JoinSet,
};

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

pub trait GetCoalescingSender<T: ReadQuery> {
    fn get(&self) -> &Sender<ShardMessage<T>>;
}

/// A [`Session`] and a collection of prepared statements.
///
/// The [`Executor`] is responsible for executing queries.
///
/// It coalesces read queries, adding waiters to a pending request. Write queries are executed immediately.
#[derive(Debug, Clone)]
pub struct Executor<T> {
    pub session: Arc<Session>,
    queries: T,
}

/// A message sent to the [`Executor::read_task`] task.
pub type ShardMessage<Q> = (Q, oneshot::Sender<ReadQueryResult<Q>>);

/// The local HashMap of requests being coalesced in a read task.
type TaskRequestMap<Q> = HashMap<u64, Vec<oneshot::Sender<ReadQueryResult<Q>>>>;

/// The result of a read query.
type ReadQueryResult<Q> = Result<<Q as ReadQuery>::Output, ScyllaxError>;

/// A message sent to the [`Executor::read_query_runner`] task.
pub struct QueryRunnerMessage<Q: ReadQuery> {
    pub hash: u64,
    pub query: Q,
    pub response_transmitter: oneshot::Sender<ReadQueryResult<Q>>,
}

impl<T: QueryCollection + Clone> Executor<T> {
    /// Creates a new [`Executor`] from a [`Session`] and a [`QueryCollection`].
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

    /// Executes a read query and returns the result.
    pub async fn execute_read<Q>(&self, query: Q) -> Result<Q::Output, ScyllaxError>
    where
        Q: ReadQuery,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let to_coalesce = Q::coalesce();
        if !to_coalesce {
            return self.perform_read_query(query).await;
        }

        let (response_tx, response_rx) = oneshot::channel();
        let task = self.queries.get_task::<Q>();

        match task.send((query, response_tx)).await {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("error sending query to task: {:#?}", e);
                return Err(ScyllaxError::NoRowsFound);
            }
        }

        match response_rx.await {
            Ok(result) => result,
            Err(e) => Err(ScyllaxError::ReceiverError(e)),
        }
    }

    /// Calculates a hash for a query.
    fn calculate_hash<Q: Hash>(t: &Q) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    /// ## internal
    /// the read task is responsible for coalescing requests
    pub async fn read_task<Q>(
        &self,
        mut request_receiver: Receiver<ShardMessage<Q>>,
        query_runner: Sender<QueryRunnerMessage<Q>>,
    ) where
        Q: ReadQuery,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let mut join_set: JoinSet<_> = JoinSet::new();
        let query_runner = Arc::new(query_runner);

        let mut requests: TaskRequestMap<Q> = HashMap::new();
        loop {
            tokio::select! {
                Some((query, tx)) = request_receiver.recv() => {
                    tracing::debug!("recieved a query: {:#?}", query);
                    let query_type = std::any::type_name::<Q>();
                    let hash = Self::calculate_hash(&query);

                    if let Some(senders) = requests.get_mut(&hash) {
                        tracing::info!(key = hash, query = query_type, "coalescing a query");
                        senders.push(tx);
                    } else {
                        requests.insert(hash, vec![tx]);
                        let (response_transmitter, response_receiver) = oneshot::channel();

                        let query_runner = query_runner.clone();
                        tokio::spawn(async move {
                            match query_runner.send(
                                    QueryRunnerMessage {
                                        hash,
                                        query,
                                        response_transmitter
                                    }
                                ).await {
                                Ok(_) => (),
                                Err(e) => {
                                    let err = TrySendError::from(e);
                                    tracing::error!(
                                        hash = hash,
                                        "error sending query to query runner: {:?}",
                                        err
                                    );
                                    // todo: propagate error to requestor
                                },
                            };
                        });

                        join_set.spawn(async move {
                            let res = match response_receiver.await {
                                Ok(result) => result,
                                Err(e) => Err(ScyllaxError::ReceiverError(e)),
                            };
                            tracing::debug!(hash = hash, "joinset handle returned: {:#?}", res);

                            (hash, res)
                        });
                    }
                },
                Some(join_handle) = join_set.join_next() => {
                    tracing::debug!("join set recieved a result!");
                    if let Ok((hash, result)) = join_handle {
                        if let Some(senders) = requests.remove(&hash) {
                            for sender in senders {
                                let _ = sender.send(result.clone());
                            }
                        }
                    }
                },
                else => {}
            }
        }
    }

    /// ## internal
    ///
    /// This function is repsonsible for receiving query requests, executing them, and sending the result back to the requestor.
    ///
    /// It is spawned by the branch of [`Executor::read_task`] that is responsible for coalescing requests.
    pub async fn read_query_runner<Q>(&self, mut query_receiver: Receiver<QueryRunnerMessage<Q>>)
    where
        Q: Query + ReadQuery + Hash + Send + Sync,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        while let Some(QueryRunnerMessage {
            query,
            response_transmitter,
            hash,
        }) = query_receiver.recv().await
        {
            tracing::debug!("running query for hash: {hash}");
            let result = self.perform_read_query(query).await;

            // todo(fyko): arc this
            let _ = response_transmitter.send(result.clone());
        }
    }

    /// ## internal
    ///
    /// Executes a read query and returns the result.
    pub(self) async fn perform_read_query<Q>(&self, query: Q) -> Result<Q::Output, ScyllaxError>
    where
        Q: Query + ReadQuery + Hash + Send + Sync,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let statement = self.queries.get_prepared::<Q>();
        let variables = query.bind().unwrap();
        let response = match self.session.execute(statement, variables).await {
            Ok(response) => {
                tracing::debug!(
                    "query executed successfully: {:?} rows",
                    response.rows_num()
                );
                response
            }
            Err(e) => {
                tracing::error!("error executing query: {:#?}", e);
                return Err(e.into());
            }
        };

        Q::parse_response(response).await
    }

    /// Executes a write query and returns the [`scylla::QueryResult`].
    pub async fn execute_write<Q>(&self, query: Q) -> Result<QueryResult, ScyllaxError>
    where
        Q: WriteQuery,
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
