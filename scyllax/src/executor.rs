use crate::{error::ScyllaxError, EntityExt, FromRow, SelectQuery, UpsertQuery, ValueList};
use scylla::{
    prepared_statement::PreparedStatement, query::Query, transport::errors::QueryError,
    CachingSession, QueryResult, SessionBuilder,
};

/// Creates a new [`CachingSession`] and returns it
pub async fn create_session(
    known_nodes: impl IntoIterator<Item = impl AsRef<str>>,
    default_keyspace: Option<impl Into<String>>,
) -> anyhow::Result<CachingSession> {
    let session = CachingSession::from(
        SessionBuilder::new()
            .known_nodes(known_nodes)
            .build()
            .await?,
        1_000,
    );

    if let Some(ks) = default_keyspace {
        session.get_session().use_keyspace(ks, true).await?;
    }

    Ok(session)
}

/// A structure that executes queries
pub struct Executor {
    pub session: CachingSession,
}

impl Executor {
    /// Creates a new [`Executor`] with a provided [`scylla::CachingSession`].
    pub fn with_session(session: CachingSession) -> Executor {
        Self { session }
    }

    /// Prepares a query
    pub async fn prepare_query(&self, query: String) -> Result<PreparedStatement, QueryError> {
        self.session
            .add_prepared_statement(&Query::new(query))
            .await
    }

    /// Executes a [`SelectQuery`] and returns the result
    pub async fn execute_select<
        T: EntityExt<T> + FromRow + ValueList,
        R: Clone + std::fmt::Debug + Send + Sync,
        E: SelectQuery<T, R>,
    >(
        &self,
        query: E,
    ) -> Result<R, ScyllaxError> {
        let res = query.execute(self).await?;
        E::parse_response(res).await
    }

    /// Executes a [`UpsertQuery`] and returns the result
    pub async fn execute_upsert<T: EntityExt<T> + FromRow + ValueList, E: UpsertQuery<T>>(
        &self,
        query: E,
    ) -> Result<QueryResult, ScyllaxError> {
        let res = query.execute(self).await?;

        Ok(res)
    }
}
