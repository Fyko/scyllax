//! playground
// Well.
// We have a wrapping struct
// RawDatabase<QueriesStruct>
// So you deal with a RawDatabase<T>
// And T is the queries you can execute.
// Then you can trait bound on T
// Which gives you compile time check to see that your query is in the struct because that implies it has a Get<Q> trait impl on T.
use crate::error::ScyllaxError;
use async_trait::async_trait;
use scylla::{
    frame::value::{SerializeValuesError, SerializedValues},
    prepared_statement::PreparedStatement,
    Session, SessionBuilder,
};
#[allow(unused, dead_code, unused_variables)]
use std::{future::Future, pin::Pin};

type Result<T> = std::result::Result<T, ScyllaxError>;
type SerializedValuesResult = std::result::Result<SerializedValues, SerializeValuesError>;

/// the entity
struct UserEntity;

/// generic query implement. this implements on all queries.
trait Query {
    fn query() -> String;

    fn bind(&self) -> SerializedValuesResult;
}

/// implements on read queries, which return an output.
trait ReadQuery {
    type Output;
}

/// empty query implementation for all write queries. this is just a marker trait.
/// so you cant pass a write query into a read query function.
trait WriteQuery {}

trait GetPreparedStatement<T: Query> {
    fn get(&self) -> &PreparedStatement;
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// A collection of prepared statements.
#[async_trait]
trait QueryCollection {
    async fn new(session: &Session) -> Result<Self>
    where
        Self: Sized;

    fn get_prepared<T: Query>(&self) -> &PreparedStatement
    where
        Self: GetPreparedStatement<T>,
    {
        <Self as GetPreparedStatement<T>>::get(self)
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
impl ReadQuery for UserByIdQuery {
    type Output = UserEntity;
}
impl GetPreparedStatement<UserByIdQuery> for UserQueries {
    fn get(&self) -> &PreparedStatement {
        &self.user_by_id_query
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
impl ReadQuery for UserByEmailQuery {
    type Output = UserEntity;
}
impl GetPreparedStatement<UserByEmailQuery> for UserQueries {
    fn get(&self) -> &PreparedStatement {
        &self.user_by_email_query
    }
}

#[allow(nonstandard_style, non_snake_case)]
struct UserQueries {
    user_by_id_query: PreparedStatement,
    user_by_email_query: PreparedStatement,
}

#[async_trait]
impl QueryCollection for UserQueries {
    async fn new(session: &Session) -> Result<Self> {
        Ok(Self {
            user_by_id_query: session.prepare(UserByIdQuery::query()).await?,
            user_by_email_query: session.prepare(UserByEmailQuery::query()).await?,
        })
    }
}

struct Executor<T> {
    session: Session,
    queries: T,
}

impl<T: QueryCollection> Executor<T> {
    async fn new(session: Session) -> Result<Self> {
        let queries = T::new(&session).await?;

        Ok(Self { session, queries })
    }

    async fn execute_read<Q>(&self, query: &Q) -> Result<Q::Output>
    where
        Q: Query + ReadQuery,
        T: GetPreparedStatement<Q>,
    {
        let statement = self.queries.get_prepared::<Q>();
        let variables = query.bind()?;

        let result = self.session.execute(statement, variables).await?;

        todo!("execute the query")
    }
}

async fn test() -> Result<()> {
    let session = SessionBuilder::new().build().await.unwrap();

    let queries = Executor::<UserQueries>::new(session).await.unwrap();

    let user = queries
        .execute_read(&UserByEmailQuery {
            email: "foo@bar.com".to_string(),
        })
        .await?;

    Ok(())
}
