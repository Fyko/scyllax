use crate::error::ScyllaxError;
use async_trait::async_trait;
use scylla::{
    frame::value::{SerializeValuesError, SerializedValues},
    QueryResult,
};

pub type SerializedValuesResult = std::result::Result<SerializedValues, SerializeValuesError>;

/// A generic query implement. This implements on all queries for type-safety.
pub trait Query {
    /// Returns the query as a string
    fn query() -> String;

    /// Turns the query into a [`SerializedValues`]
    fn bind(&self) -> SerializedValuesResult;
}

/// The trait that's implemented on read queries, which return an output which demands a parser.
#[async_trait]
pub trait ReadQuery {
    type Output: Clone + std::fmt::Debug + Send + Sync;

    /// Parses the response from the database
    async fn parse_response(rows: QueryResult) -> Result<Self::Output, ScyllaxError>;
}

/// Empty query implementation for all write queries. This is just a marker trait.
/// So you cant pass a write query into a read query function.
pub trait WriteQuery {}
