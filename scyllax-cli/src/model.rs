use scylla::frame::value::Timestamp;
use scyllax::prelude::*;

create_query_collection!(
    MigrationQueries,
    [GetLatestVersion],
    [DeleteByVersion, UpsertMigration]
);

#[entity]
#[upsert_query(table = "migration", name = UpsertMigration)]
pub struct MigrationEntity {
    #[entity(primary_key)]
    pub version: i64,
    #[entity(primary_key)]
    pub bucket: i32,
    pub description: String,
    pub installed_on: Timestamp,
    pub success: bool,
    pub checksum: Vec<u8>,
    pub execution_time: i64,
}

// get the latest version from the database
#[derive(Debug, Clone, PartialEq, ValueList, ReadQuery)]
#[read_query(
    query_nocheck = "select * from migration where bucket = 0 order by version desc limit 1",
    return_type = "MigrationEntity"
)]
pub struct GetLatestVersion {}

/// Delete a migration by its version
#[write_query(query = "delete from migration where bucket = 0 and version = :version")]
pub struct DeleteByVersion {
    pub version: i64,
}
