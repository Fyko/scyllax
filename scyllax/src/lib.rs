pub use error::BuildUpsertQueryError;
pub use scylla::{
    prepared_statement::PreparedStatement, transport::errors::QueryError, QueryResult,
};

pub use crate::{error::ScyllaxError, executor::Executor};
pub use async_trait::async_trait;
pub use scylla::{frame::value::ValueList, FromRow};
pub use scyllax_macros::*;

pub mod error;
pub mod executor;
pub mod maybe_unset;
pub mod prelude;
pub mod rows;
pub mod util;

/// The traits of the entity
pub trait EntityExt<T: ValueList + FromRow> {
    /// Returns the keys of the entity as a vector of strings, keeping the order of the keys.
    fn keys() -> Vec<String>;

    /// Returns the primary keys
    fn pks() -> Vec<String>;
}

/// The trait that's implemented on select/read queries
// R is the return type of the query
// It can be either Option<T> or Vec<T>
#[async_trait]
pub trait SelectQuery<
    T: EntityExt<T> + ValueList + FromRow,
    R: Clone + std::fmt::Debug + Send + Sync,
>
{
    /// Returns the query as a string
    fn query() -> String;

    /// Prepares the query
    async fn prepare(db: &Executor) -> Result<PreparedStatement, QueryError>;

    /// Executes the query
    async fn execute(self, db: &Executor) -> Result<QueryResult, QueryError>;

    /// Parses the response from the database
    async fn parse_response(res: QueryResult) -> Result<R, ScyllaxError>;
}

/// The trait that's implemented on update/insert queryes
#[async_trait]
pub trait UpsertQuery<T: EntityExt<T> + ValueList + FromRow> {
    /// Returns the query as a string
    fn query(
        &self,
    ) -> Result<(String, scylla::frame::value::SerializedValues), BuildUpsertQueryError>;

    /// Executes the query
    async fn execute(self, db: &Executor) -> Result<QueryResult, ScyllaxError>;
}
