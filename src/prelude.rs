//! Re-exports of the most commonly used types and traits.
pub use crate::{
    delete_query, entity, error::BuildUpsertQueryError, executor::Executor, json_data,
    maybe_unset::MaybeUnset, prepare_queries, select_query, upsert_query, util::v1_uuid,
    DeleteQuery, Entity, EntityExt, FromRow, GenericQuery, GetPreparedStatement, ImplValueList,
    JsonData, Queries, ScyllaxError, SelectQuery, UpsertQuery,
};

pub use scylla::frame::value::SerializeValuesError;
pub use scylla::frame::value::SerializedValues;
pub use scylla::ValueList;
