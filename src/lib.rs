//! # Scyllax (sɪl-æks)
//! A SQLx and Discord inspired query system for ScyllaDB
//!
//! ## Example
//! ### 1. Model definition
//! Before you can write any queries, you have to define a model.
//! ```rust,ignore
//! #[derive(Clone, Debug, FromRow, PartialEq, ValueList, Entity)]
//! pub struct PersonEntity {
//!     #[pk]
//!     pub id: uuid::Uuid,
//!     pub email: String,
//!     pub created_at: i64,
//! }
//! ```
//! ### 2. Select queries
//! With the [`select_query`] attribute, it's easy to define select queries.
//! ```rust,ignore
//! #[select_query(
//!     query = "select * from person where id = ? limit 1",
//!     entity_type = "PersonEntity"
//! )]
//! pub struct GetPersonById {
//!     pub id: Uuid,
//! }
//! ```
//! ### 3. Upsert queries
//! With the [`upsert_query`] attribute, it's easy to define upsert queries.
//! ```rust,ignore
//! #[upsert_query(table = "person", name = UpsertPerson)]
//! #[derive(Clone, Debug, FromRow, PartialEq, ValueList, Entity)]
//! pub struct PersonEntity {
//!     #[pk]
//!     pub id: uuid::Uuid,
//!     pub email: String,
//!     pub created_at: i64,
//! }
//! ```
pub use error::BuildUpsertQueryError;
pub use scylla::{
    prepared_statement::PreparedStatement, transport::errors::QueryError, QueryResult,
};

pub use crate::{error::ScyllaxError, executor::Executor};
pub use async_trait::async_trait;
pub use scylla::{frame::value::ValueList as ImplValueList, FromRow};
pub use scyllax_macros::*;

pub mod error;
pub mod executor;
pub mod maybe_unset;
pub mod prelude;
pub mod rows;
pub mod util;

/// The traits of the entity
pub trait EntityExt<T: ImplValueList + FromRow> {
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
    T: EntityExt<T> + ImplValueList + FromRow,
    R: Clone + std::fmt::Debug + Send + Sync,
    Q: Queries,
>
{
    /// Executes the query
    async fn execute(self, db: &Executor<Q>) -> Result<QueryResult, QueryError>;

    /// Parses the response from the database
    async fn parse_response(res: QueryResult) -> Result<R, ScyllaxError>;
}

/// The trait that's implemented on update/insert queryes
#[async_trait]
pub trait UpsertQuery<T: EntityExt<T> + ImplValueList + FromRow, Q: Queries> {
    /// Returns the query as a string
    fn create_serialized_values(
        &self,
    ) -> Result<scylla::frame::value::SerializedValues, BuildUpsertQueryError>;

    /// Executes the query
    async fn execute(self, db: &Executor<Q>) -> Result<QueryResult, ScyllaxError>;
}

/// The trait that's implemented on delete queries
// R is the return type of the query
// It can be either Option<T> or Vec<T>
#[async_trait]
pub trait DeleteQuery<T: EntityExt<T> + ImplValueList + FromRow, Q: Queries> {
    /// Executes the query
    async fn execute(self, db: &Executor<Q>) -> Result<QueryResult, QueryError>;
}

/// Applied to a superstruct of prepared statements.
pub trait GetPreparedStatement<T> {
    /// Returns the prepared statement
    fn get_prepared_statement(&self) -> &PreparedStatement;
}

/// A generic query that implements `query`.
pub trait GenericQuery<T: EntityExt<T> + ImplValueList + FromRow> {
    /// Returns the query as a string
    fn query() -> String;
}

/// A superstruct of all the queries
#[async_trait]
pub trait Queries
where
    Self: Sized,
{
    /// Creates a new instance
    async fn new(session: &scylla::Session) -> Result<Self, ScyllaxError>;

    #[doc = "Get a prepared statement."]
    fn get<T>(&self) -> &scylla::statement::prepared_statement::PreparedStatement
    where
        Self: GetPreparedStatement<T>,
    {
        <Self as GetPreparedStatement<T>>::get_prepared_statement(self)
    }
}
