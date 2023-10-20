use anyhow::Context;
use console::style;
use scylla::query::Query;
use scyllax::executor::create_session;
use std::fs::{self, File};
use time::format_description;

use crate::{
    migrator::{DownMigration, MigrationFolder, MigrationMode, Migrator, UpMigration},
    model::{create_migration_executor, GetLatestVersion},
    opt::ConnectOpts,
};

fn create_file<K: MigrationMode>(migration_source: &str) -> anyhow::Result<()> {
    use std::path::PathBuf;

    let mut path = PathBuf::new();
    path.push(migration_source);
    path.push(K::name());

    println!("Creating {}", style(K::name()).cyan());

    let mut file = File::create(&path).context("Failed to create migration file")?;

    std::io::Write::write_all(&mut file, K::file_content().as_bytes())?;

    Ok(())
}

pub async fn add(migration_source: &str, description: &str) -> anyhow::Result<()> {
    fs::create_dir_all(migration_source).context("Unable to create migrations directory")?;

    let format = format_description::parse("[year][month][day][hour][minute][second]")?;
    let ts = time::OffsetDateTime::now_utc().format(&format)?;

    let folder = format!("{}_{}", ts, description.replace(' ', "_"));
    println!("Creating {}", style(&folder).cyan());
    // create folder
    let mut path = std::path::PathBuf::new();
    path.push(migration_source);
    path.push(&folder);
    fs::create_dir_all(&path).context("Unable to create migration directory")?;

    create_file::<UpMigration>(&path.to_string_lossy())?;
    create_file::<DownMigration>(&path.to_string_lossy())?;

    println!("Migration created successfully");

    Ok(())
}

/// Runs all pending migrations.
pub async fn run(
    migration_source: &str,
    connect_opts: ConnectOpts,
    only_next: bool,
) -> anyhow::Result<()> {
    let executor =
        create_migration_executor(connect_opts.scylla_nodes, connect_opts.keyspace).await?;

    let current_version = executor
        .execute_read(GetLatestVersion {})
        .await?
        .map_or(-1, |v| v.version);

    let migrator =
        Migrator::<UpMigration>::new(executor.clone(), migration_source, current_version);

    let mut migration_folders = migrator
        .iter()
        .scan((), |_, x| x.ok())
        .collect::<Vec<MigrationFolder<UpMigration>>>();
    migration_folders.sort_by(|a, b| a.version.cmp(&b.version));

    if migration_folders.is_empty() {
        println!("No pending migrations");
        return Ok(());
    }

    for migration_folder in migration_folders {
        migration_folder.execute(&migrator).await?;

        // if we are running only the next migration,
        // then we need to stop here
        if only_next {
            break;
        }
    }

    Ok(())
}

/// Reverts the last migration.
pub async fn revert(migration_source: &str, connect_opts: ConnectOpts) -> anyhow::Result<()> {
    let executor =
        create_migration_executor(connect_opts.scylla_nodes, connect_opts.keyspace).await?;

    let current_version = if let Some(v) = executor.execute_read(GetLatestVersion {}).await? {
        v.version
    } else {
        println!("No migrations to revert");
        return Ok(());
    };

    let migrator =
        Migrator::<DownMigration>::new(executor.clone(), migration_source, current_version);

    let migration = if let Some(Ok(m)) = migrator.iter().next() {
        m
    } else {
        println!("Couldn't find migration ({}) to revert.", current_version);
        return Ok(());
    };

    migration.execute(&migrator).await?;

    Ok(())
}

/// Creates the `scyllax_migrations` table.
pub async fn init(connect_opts: ConnectOpts) -> anyhow::Result<()> {
    let create_keyspace = r#"create keyspace if not exists scyllax_migrations with replication = { 'class': 'NetworkTopologyStrategy', 'datacenter1': 1 };"#;
    let create_table = r#"
create table if not exists scyllax_migrations.migration (
    bucket int,
	version bigint,
	description text,
	installed_on timestamp,
	success boolean,
	checksum blob,
	execution_time bigint,
	primary key (bucket, version)
);"#;

    let session = create_session(
        connect_opts.scylla_nodes.split(','),
        Some(connect_opts.keyspace),
    )
    .await?;

    for query in [create_keyspace, create_table] {
        let prepared_query = Query::new(query);
        session.query(prepared_query, ()).await?;
    }

    println!(
        "{}\n{}\n{}",
        style("scyllax_migrations keyspace and table created.").green(),
        style("It's recommended you manually create these tables in production."),
        style("See init.cql in folder for scyllax-cli in our GitHub repository.")
    );

    Ok(())
}
