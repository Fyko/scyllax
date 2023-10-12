//! playground
// Well.
// We have a wrapping struct
// RawDatabase<QueriesStruct>
// So you deal with a RawDatabase<T>
// And T is the queries you can execute.
// Then you can trait bound on T
// Which gives you compile time check to see that your query is in the struct because that implies it has a Get<Q> trait impl on T.
use crate::{error::ScyllaxError, queries};
use async_trait::async_trait;
use scylla::{
    frame::value::{SerializeValuesError, SerializedValues},
    prepared_statement::PreparedStatement,
    Session, SessionBuilder, QueryResult, FromRow,
};
use std::{collections::HashMap, sync::Arc};
#[allow(unused, dead_code, unused_variables)]
use std::{future::Future, pin::Pin};
use tokio::sync::mpsc::{Sender, Receiver};
use tokio::sync::oneshot;

type Result<T> = std::result::Result<T, ScyllaxError>;
type SerializedValuesResult = std::result::Result<SerializedValues, SerializeValuesError>;

macro_rules! match_row {
    ($res:ident, $type:ty) => {
        match $res.single_row_typed::<$type>() {
            Ok(data) => Ok(Some(data)),
            Err(err) => {
                use scylla::transport::query_result::SingleRowTypedError;
                match err {
                    // tried to parse into type, but there are no rows
                    SingleRowTypedError::BadNumberOfRows(_) => Ok(None),
                    _ => {
                        tracing::error!("err: {:?}", err);
                        Err(ScyllaxError::SingleRowTyped(err))
                    }
                }
            }
        }
    };
}

/// the entity
#[derive(Clone, Debug, FromRow)]
struct UserEntity {
    id: i32,
}

/// generic query implement. this implements on all queries.
trait Query {
    fn query() -> String;

    fn bind(&self) -> SerializedValuesResult;
}

/// implements on read queries, which return an output.
#[async_trait]
pub trait ReadQuery {
    type Output: Clone + std::fmt::Debug + Send + Sync;

    /// Returns the shard key for the query
    fn shard_key(&self) -> String {
        // TODO: impl me
        String::new()
    }

    /// Parses the response from the database
    async fn parse_response(rows: QueryResult) -> Result<Self::Output>;
}

/// empty query implementation for all write queries. this is just a marker trait.
/// so you cant pass a write query into a read query function.
trait WriteQuery {}

trait GetPreparedStatement<T: Query> {
    fn get(&self) -> &PreparedStatement;
}
pub trait GetCoalescingSender<T: Query + ReadQuery> {
    fn get(&self) -> &Sender<ShardMessage<T>>;
}

pub type ShardMessage<Q> = (Q, oneshot::Sender<Result<<Q as ReadQuery>::Output>>);
type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A collection of prepared statements.
#[async_trait]
trait QueryCollection {
    async fn new(session: &Session) -> Result<Self>
    where
        Self: Sized;

    fn register_tasks(self, executor: Arc<Executor<Self>>) -> Self
    where
        Self: Sized;

    fn get_prepared<T: Query>(&self) -> &PreparedStatement
    where
        Self: GetPreparedStatement<T>,
    {
        <Self as GetPreparedStatement<T>>::get(self)
    }

    fn get_task<T: Query + ReadQuery>(&self) -> &Sender<ShardMessage<T>>
    where
        Self: GetCoalescingSender<T>,
    {
        <Self as GetCoalescingSender<T>>::get(self)
    }
}

struct UserByIdQuery {
    id: i32,
}
impl Query for UserByIdQuery {
    fn query() -> String {
        "select * from users where id = :id".to_string()
    }

    fn bind(&self) -> SerializedValuesResult {
        let mut values = SerializedValues::new();
        values.add_named_value("id", &self.id)?;

        Ok(values)
    }
}
#[async_trait]
impl ReadQuery for UserByIdQuery {
    type Output = Option<UserEntity>;

    async fn parse_response(rows: QueryResult) -> Result<Self::Output> {
        match_row!(rows, UserEntity)
    }
}
impl GetPreparedStatement<UserByIdQuery> for UserQueries {
    fn get(&self) -> &PreparedStatement {
        &self.user_by_id_query
    }
}
impl GetCoalescingSender<UserByIdQuery> for UserQueries {
    fn get(&self) -> &Sender<ShardMessage<UserByIdQuery>> {
        &self.user_by_id_task.as_ref().unwrap()
    }
}

struct UserByEmailQuery {
    email: String,
}
impl Query for UserByEmailQuery {
    fn query() -> String {
        "select * from users_by_email where email = :email".to_string()
    }

    fn bind(&self) -> SerializedValuesResult {
        let mut values = SerializedValues::with_capacity(1);
        values.add_named_value("email", &self.email);

        Ok(values)
    }
}
#[async_trait]
impl ReadQuery for UserByEmailQuery {
    type Output = Option<UserEntity>;

    async fn parse_response(rows: QueryResult) -> Result<Self::Output> {
        match_row!(rows, UserEntity)
    }
}
impl GetPreparedStatement<UserByEmailQuery> for UserQueries {
    fn get(&self) -> &PreparedStatement {
        &self.user_by_email_query
    }
}
impl GetCoalescingSender<UserByEmailQuery> for UserQueries {
    fn get(&self) -> &Sender<ShardMessage<UserByEmailQuery>> {
        &self.user_by_email_task.as_ref().unwrap()
    }
}

#[derive(Debug, Clone)]
#[allow(nonstandard_style, non_snake_case)]
struct UserQueries {
    user_by_id_query: PreparedStatement,
    user_by_email_query: PreparedStatement,
    user_by_id_task: Option<Sender<ShardMessage<UserByIdQuery>>>,
    user_by_email_task: Option<Sender<ShardMessage<UserByEmailQuery>>>,
}

#[async_trait]
impl QueryCollection for UserQueries {
    async fn new(session: &Session) -> Result<Self> {
        Ok(Self {
            user_by_id_query: session.prepare(UserByIdQuery::query()).await?,
            user_by_email_query: session.prepare(UserByEmailQuery::query()).await?,
            user_by_id_task: None,
            user_by_email_task: None,
        })
    }

    fn register_tasks(mut self, executor: Arc<Executor<Self>>) -> Self {
        self.user_by_id_task = {
            let (tx, rx) = tokio::sync::mpsc::channel(100);
            let ex = executor.clone();
            tokio::spawn(async move {
                ex.read_task::<UserByIdQuery>(rx).await;
            });
            Some(tx)
        };
        
        self
    }
}

#[derive(Debug, Clone)]
struct Executor<T> {
    session: Arc<Session>,
    queries: T,
}

impl<T: QueryCollection + Clone> Executor<T> {
    async fn new(session: Arc<Session>) -> Result<Self> {
        let queries = T::new(&session).await?;
        let executor = Arc::new(Self { session: session.clone(), queries });

        let queries = executor.queries.clone().register_tasks(executor);
        let executor = Self { session, queries };

        Ok(executor)
    }

    pub async fn execute_read<Q>(&self, query: Q) -> Result<Q::Output>
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
        let mut requests: HashMap<String, Vec<oneshot::Sender<Result<Q::Output>>>> = HashMap::new();

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

    pub async fn execute_write<Q>(&self, query: &Q) -> Result<QueryResult>
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

async fn test() -> Result<()> {
    let session = Arc::new(SessionBuilder::new().build().await.unwrap());

    let queries = Executor::<UserQueries>::new(session).await.unwrap();

    let user = queries
        .execute_read(UserByEmailQuery {
            email: "foo@bar.com".to_string(),
        })
        .await?;

    Ok(())
}
