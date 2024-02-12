use std::sync::Arc;

use scylla::{frame::value::CqlTimestamp, SerializeRow};
use scyllax::prelude::*;

create_query_collection!(
    MigrationQueries,
    [GetLatestVersion],
    [DeleteByVersion, UpsertMigration]
);

pub async fn create_migration_executor(
    scylla_nodes: String,
    keyspace: String,
) -> anyhow::Result<Arc<Executor<MigrationQueries>>> {
    let session = create_session(scylla_nodes.split(','), Some(keyspace)).await?;

    let executor = Executor::<MigrationQueries>::new(Arc::new(session)).await?;

    Ok(Arc::new(executor))
}

#[entity]
#[upsert_query(table = "migration", name = UpsertMigration)]
pub struct MigrationEntity {
    #[entity(primary_key)]
    pub version: i64,
    #[entity(primary_key)]
    pub bucket: i32,
    pub description: String,
    pub installed_on: CqlTimestamp,
    pub success: bool,
    pub checksum: Vec<u8>,
    pub execution_time: i64,
}

// get the latest version from the database
#[derive(Debug, Clone, PartialEq, SerializeRow, ReadQuery)]
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
