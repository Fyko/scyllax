use crate::{EntityExt, FromRow, ScyllaxError, SelectQuery, ValueList};
use scylla::{
    prepared_statement::PreparedStatement, query::Query, transport::errors::QueryError,
    CachingSession,
};

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
}
