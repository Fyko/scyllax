//! Re-exports of the most commonly used types and traits.
pub use crate::{
    collection::{prepare_query, QueryCollection},
    entity::EntityExt,
    error::{BuildUpsertQueryError, ScyllaxError},
    executor::{create_session, Executor, GetCoalescingSender, GetPreparedStatement},
    maybe_unset::MaybeUnset,
    queries::{Query, ReadQuery, SerializedValuesResult, WriteQuery},
    util::v1_uuid,
};
pub use async_trait::async_trait;
pub use scylla_reexports::*;
pub use scyllax_macros::*;

pub mod scylla_reexports {
    //! Re-exports of the most commonly used types and traits from the `scylla` crate.
    pub use scylla::{
        frame::value, statement::prepared_statement::PreparedStatement,
        transport::errors::QueryError, FromRow, QueryResult, Session, ValueList,
    };
}
