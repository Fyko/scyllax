use std::sync::Arc;

use crate::{
    error::ScyllaxError,
    executor::{Executor, GetCoalescingSender, GetPreparedStatement, ShardMessage},
    prelude::ReadQuery,
    queries::Query,
};
use async_trait::async_trait;
use scylla::{prepared_statement::PreparedStatement, Session};
use tokio::sync::mpsc::Sender;

/// A collection of prepared statements.
#[async_trait]
pub trait QueryCollection {
    /// Create a new collection of prepared statements.
    async fn new(session: &Session) -> Result<Self, ScyllaxError>
    where
        Self: Sized;

    /// Register all tasks with the executor.
    fn register_tasks(self, executor: Arc<Executor<Self>>) -> Self
    where
        Self: Sized;

    /// Gets a prepared statement from the collection.
    fn get_prepared<T: Query>(&self) -> &PreparedStatement
    where
        Self: GetPreparedStatement<T>,
    {
        <Self as GetPreparedStatement<T>>::get(self)
    }

    /// Gets a task from the collection.
    fn get_task<T: Query + ReadQuery>(&self) -> &Sender<ShardMessage<T>>
    where
        Self: GetCoalescingSender<T>,
    {
        <Self as GetCoalescingSender<T>>::get(self)
    }
}

/// Prepares a query
#[tracing::instrument(skip(session))]
pub async fn prepare_query(
    session: &Session,
    query: String,
    query_type: &str,
) -> Result<PreparedStatement, ScyllaxError> {
    tracing::info!("preparing query");

    let res = session.prepare(query).await;

    match res {
        Ok(prepared) => Ok(prepared),
        Err(err) => {
            tracing::error!("failed to prepare query: {:#?}", err);

            return Err(err.into());
        }
    }
}
