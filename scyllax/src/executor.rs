//! The `scyllax` [`Executor`] processes queries.
use std::{collections::HashMap, sync::mpsc::RecvError};
use crate::{
    collection::QueryCollection,
    error::ScyllaxError,
    prelude::WriteQuery,
    queries::{Query, ReadQuery},
};
use scylla::{prepared_statement::PreparedStatement, QueryResult, Session, SessionBuilder};
use tokio::sync::mpsc::{Sender, Receiver};
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

#[derive(Debug)]
pub struct Executor<T> {
    pub session: Session,
    queries: T,
}

pub type ShardMessage<Q> = (Q, oneshot::Sender<Result<<Q as ReadQuery>::Output, ScyllaxError>>);

impl<T: QueryCollection> Executor<T> {
    pub async fn new(session: Session) -> Result<Self, ScyllaxError> {
        let queries = T::new(&session).await?;
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

        task.send((query, response_tx)).await.unwrap();

        match response_rx.await {
            Ok(result) => result,
            Err(e) => Err(ScyllaxError::ReceiverError(e)),
        }
    }

    async fn read_task<Q>(&self, mut rx: Receiver<ShardMessage<Q>>)
    where
        Q: Query + ReadQuery,
        T: GetPreparedStatement<Q> + GetCoalescingSender<Q>,
    {
        let mut requests: HashMap<String, Vec<oneshot::Sender<Result<Q::Output, ScyllaxError>>>> = HashMap::new();

        while let Some((query, tx)) = rx.recv().await {
            let key = query.shard_key();

            if let Some(senders) = requests.get_mut(&key) {
                senders.push(tx);
            } else {
                let mut senders = Vec::new();
                senders.push(tx);
                requests.insert(key.clone(), senders);

                // Execute the query here and send the result back
                // let result = self.execute_read(&query).await;
                let statement = self.queries.get_prepared::<Q>();
                // FIXME: better error handling
                let variables = query.bind().unwrap();
                // FIXME: better error handling
                let result = self.session.execute(statement, variables).await.unwrap();
                let parsed = Q::parse_response(result).await;

                if let Some(senders) = requests.remove(&key) {
                    for tx in senders {
                        let _ = tx.send(parsed.clone());
                    }
                }
            }
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
