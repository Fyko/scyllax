//! The `scyllax` [`Executor`] processes queries.

use crate::{
    error::ScyllaxError, DeleteQuery, EntityExt, FromRow, GenericQuery, ImplValueList, Queries,
    SelectQuery, UpsertQuery,
};
use scylla::{
    prepared_statement::PreparedStatement, transport::errors::QueryError, QueryResult, Session,
    SessionBuilder,
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

/// A structure that executes queries
pub struct Executor<Q>
where
    Q: Queries,
{
    /// The internal [`scylla::CachingSession`]
    pub session: Session,
    /// All the prepared statements
    pub queries: Q,
}

impl<Q: Queries> Executor<Q> {
    /// Creates a new [`Executor`] with a provided [`scylla::CachingSession`].
    pub fn with_session(session: Session, queries: Q) -> Executor<Q> {
        Self { session, queries }
    }

    /// Prepares a query
    pub async fn prepare_query(&self, query: String) -> Result<PreparedStatement, QueryError> {
        self.session.prepare(query).await
    }

    /// Executes a [`SelectQuery`] and returns the result
    pub async fn execute_select<
        T: EntityExt<T> + FromRow + ImplValueList,
        R: Clone + std::fmt::Debug + Send + Sync,
        E: GenericQuery<T> + SelectQuery<T, R, Q>,
    >(
        &self,
        query: E,
    ) -> Result<R, ScyllaxError> {
        let res = query.execute(self).await?;
        E::parse_response(res).await
    }

    /// Executes a [`DeleteQuery`] and returns the result
    pub async fn execute_delete<
        T: EntityExt<T> + FromRow + ImplValueList,
        E: GenericQuery<T> + DeleteQuery<T, Q>,
    >(
        &self,
        query: E,
    ) -> Result<QueryResult, ScyllaxError> {
        let res = query.execute(self).await?;

        Ok(res)
    }

    /// Executes a [`UpsertQuery`] and returns the result
    pub async fn execute_upsert<
        T: EntityExt<T> + FromRow + ImplValueList,
        E: GenericQuery<T> + UpsertQuery<T, Q>,
    >(
        &self,
        query: E,
    ) -> Result<QueryResult, ScyllaxError> {
        let res = query.execute(self).await?;

        Ok(res)
    }
}
