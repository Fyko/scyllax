//! Re-exports of the most commonly used types and traits.
pub use crate::{
    error::BuildUpsertQueryError, executor::Executor, maybe_unset::MaybeUnset, select_query,
    upsert_query, util::v1_uuid, DeleteQuery, Entity, EntityExt, FromRow, ImplValueList,
    ScyllaxError, SelectQuery, UpsertQuery,
};

pub use scylla::frame::value::SerializeValuesError;
pub use scylla::frame::value::SerializedValues;
pub use scylla::ValueList;
