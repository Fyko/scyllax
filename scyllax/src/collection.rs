use crate::{error::ScyllaxError, executor::GetPreparedStatement, queries::Query};
use async_trait::async_trait;
use scylla::{prepared_statement::PreparedStatement, Session};

/// A collection of prepared statements.
#[async_trait]
pub trait QueryCollection {
    async fn new(session: &Session) -> Result<Self, ScyllaxError>
    where
        Self: Sized;

    fn get_prepared<T: Query>(&self) -> &PreparedStatement
    where
        Self: GetPreparedStatement<T>,
    {
        <Self as GetPreparedStatement<T>>::get(self)
    }
}

#[tracing::instrument(skip(session))]
pub async fn prepare_query(
    session: &Session,
    query: String,
) -> Result<PreparedStatement, ScyllaxError> {
    tracing::info!("preparing");
    Ok(session.prepare(query).await?)
}
