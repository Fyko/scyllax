//! The `scyllax` [`Executor`] processes queries.
use crate::{
    collection::QueryCollection,
    error::ScyllaxError,
    prelude::WriteQuery,
    queries::{Query, ReadQuery},
};
use scylla::{prepared_statement::PreparedStatement, QueryResult, Session, SessionBuilder};

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

#[derive(Debug)]
pub struct Executor<T> {
    pub session: Session,
    queries: T,
}

impl<T: QueryCollection> Executor<T> {
    pub async fn new(session: Session) -> Result<Self, ScyllaxError> {
        let queries = T::new(&session).await?;

        Ok(Self { session, queries })
    }

    pub async fn execute_read<Q>(&self, query: &Q) -> Result<Q::Output, ScyllaxError>
    where
        Q: Query + ReadQuery,
        T: GetPreparedStatement<Q>,
    {
        let statement = self.queries.get_prepared::<Q>();
        let variables = query.bind()?;

        let result = self.session.execute(statement, variables).await?;

        Q::parse_response(result).await
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
