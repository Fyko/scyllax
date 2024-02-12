use std::fmt::Debug;

use crate::error::ScyllaxError;
use async_trait::async_trait;
use scylla::{
    frame::value::{LegacySerializedValues, SerializeValuesError},
    serialize::row::SerializeRow,
    QueryResult,
};

pub type SerializedValuesResult = std::result::Result<LegacySerializedValues, SerializeValuesError>;

/// A generic query implement. This implements on all queries for type-safety.
pub trait Query
where
    Self: SerializeRow + Debug + Send + Sync + Sized,
{
    /// Returns the query as a string
    fn query() -> String;
}

/// The trait that's implemented on read queries, which return an output which demands a parser.
#[async_trait]
pub trait ReadQuery
where
    Self: Query + std::hash::Hash + Sized + 'static,
{
    type Output: Clone + Debug + Send + Sync;

    /// Parses the response from the database
    async fn parse_response(rows: QueryResult) -> Result<Self::Output, ScyllaxError>;

    /// Whether or not the query should be coalesced
    fn coalesce() -> bool {
        true
    }
}

/// Empty query implementation for all write queries. This is just a marker trait.
/// So you cant pass a write query into a read query function.
pub trait WriteQuery
where
    Self: Query,
{
}
