use super::model::UpsertPost;
use scyllax::prelude::*;
use uuid::Uuid;

create_query_collection!(PosQueries, [GetPostById], [DeletePostById, UpsertPost]);

/// Get a [`super::model::PostEntity`] by its [`uuid::Uuid`]
#[derive(Debug, Clone, PartialEq, ValueList, ReadQuery)]
#[read_query(
    query = "select * from post where id = :id limit 1",
    return_type = "super::model::PostEntity"
)]
pub struct GetPostById {
    /// The [`uuid::Uuid`] of the [`super::model::PostEntity`] to get
    #[read_query(coalesce_shard_key)]
    pub id: Uuid,
}

/// Get a [`super::model::PostEntity`] by its [`uuid::Uuid`]
#[write_query(query = "delete from post where id = :id")]
pub struct DeletePostById {
    /// The [`uuid::Uuid`] of the [`super::model::PostEntity`] to get
    pub id: Uuid,
}
