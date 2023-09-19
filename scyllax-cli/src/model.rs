use scylla::frame::value::Timestamp;
use scyllax::prelude::*;

#[upsert_query(table = "migration", name = UpsertMigration)]
#[derive(scylla::FromRow, scylla::ValueList, Debug, Clone, PartialEq, Entity)]
pub struct MigrationEntity {
    #[pk]
    pub version: i64,
    #[pk]
    pub bucket: i32,
    pub description: String,
    pub installed_on: Timestamp,
    pub success: bool,
    pub checksum: Vec<u8>,
    pub execution_time: i64,
}

// get the latest version from the database
#[select_query(
    query = "select * from migration where bucket = 0 order by version desc limit 1",
    entity_type = "MigrationEntity"
)]
pub struct GetLatestVersion {}

/// Delete a migration by its version
#[delete_query(
    query = "delete from migration where bucket = 0 and version = ?",
    entity_type = "MigrationEntity"
)]
pub struct DeleteByVersion {
    pub version: i64,
}
