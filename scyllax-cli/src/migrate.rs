use anyhow::Context;
use console::style;
use scylla::{frame::value::Timestamp, query::Query, statement::SerialConsistency};
use scyllax::executor::{create_session, Executor};
use sha2::{Digest, Sha384};
use std::fs::{self, File};
use time::format_description;

use crate::{
    migrator::MigrationFolder,
    model::{DeleteByVersion, GetLatestVersion, MigrationQueries, UpsertMigration},
    opt::ConnectOpts,
};

enum MigrationType {
    Up,
    Down,
}

impl MigrationType {
    fn name(&self) -> &'static str {
        match self {
            MigrationType::Up => "up.cql",
            MigrationType::Down => "down.cql",
        }
    }

    fn file_content(&self) -> &'static str {
        match self {
            MigrationType::Up => "-- CREATE TABLE IF NOT EXISTS test (id int PRIMARY KEY);",
            MigrationType::Down => "-- DROP TABLE test;",
        }
    }
}

fn create_file(migration_source: &str, migration_type: MigrationType) -> anyhow::Result<()> {
    use std::path::PathBuf;

    let mut path = PathBuf::new();
    path.push(migration_source);
    path.push(migration_type.name());

    println!("Creating {}", style(migration_type.name()).cyan());

    let mut file = File::create(&path).context("Failed to create migration file")?;

    std::io::Write::write_all(&mut file, migration_type.file_content().as_bytes())?;

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

    create_file(&path.to_string_lossy(), MigrationType::Up)?;
    create_file(&path.to_string_lossy(), MigrationType::Down)?;

    println!("Migration created successfully");

    Ok(())
}

/// Runs all pending migrations.
pub async fn run(
    migration_source: &str,
    connect_opts: ConnectOpts,
    only_next: bool,
) -> anyhow::Result<()> {
    let files = fs::read_dir(migration_source).context("Unable to read migrations directory")?;

    // if target_version is specified, then we need to check if we are already at the target version
    let session = create_session([connect_opts.database_url], Some(connect_opts.keyspace)).await?;
    let executor = Executor::<MigrationQueries>::new(session).await?;

    let current_version = executor
        .execute_read(&GetLatestVersion {})
        .await?
        .map_or(-1, |v| v.version);

    let mut migration_folders = Vec::new();
    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_string_lossy();

        let migration_folder = MigrationFolder::try_from(file_name.clone())?;

        if migration_folder.version <= current_version {
            continue;
        }

        migration_folders.push(migration_folder);
    }
    migration_folders.sort_by(|a, b| a.version.cmp(&b.version));

    if migration_folders.is_empty() {
        println!("No pending migrations");
        return Ok(());
    }

    // now, for every migration folder, we need to run the up file
    for migration_folder in migration_folders {
        let mut path = std::path::PathBuf::new();
        path.push(migration_source);
        path.push(&migration_folder.to_string());

        let mut up_path = path.clone();
        up_path.push(MigrationType::Up.name());

        let contents = fs::read_to_string(&up_path).context("Unable to read up file")?;

        // the file contents can hold multiple statements separated by semicolons and newlines
        // we need to split them where there's a while line that's just a linebreak (\n)
        // and then execute each statement separately
        let statements = contents.split("\n\n").collect::<Vec<_>>();
        let session = &executor.session;

        println!(
            "Applying migration {}...",
            style(&migration_folder.to_string()).cyan(),
        );

        let start = std::time::Instant::now();
        let count = statements.len();
        for (i, statement) in statements.into_iter().enumerate() {
            let mut prepared_query = Query::new(statement);
            prepared_query.set_tracing(true);
            prepared_query.set_serial_consistency(Some(SerialConsistency::LocalSerial));

            let start = std::time::Instant::now();
            session.query(prepared_query.clone(), ()).await?;
            let end = std::time::Instant::elapsed(&start);

            println!(
                "Applied statement {} ({}) ({})",
                style(i).green(),
                style(format!("of {count}")).dim(),
                style(format!("{}ms", end.as_millis())).dim()
            );
        }
        let end = std::time::Instant::elapsed(&start);
        println!(
            "Applied migration {} ({})",
            style(&migration_folder.to_string()).cyan(),
            style(format!("{}ms", end.as_millis())).dim()
        );

        let checksum: Vec<u8> = Vec::from(Sha384::digest(contents.as_bytes()).as_slice());

        let upsert_row = UpsertMigration {
            bucket: 0,
            version: migration_folder.version,
            description: migration_folder.description.into(),
            installed_on: Timestamp(chrono::Duration::seconds(
                time::OffsetDateTime::now_utc().unix_timestamp(),
            ))
            .into(),
            success: true.into(),
            checksum: checksum.into(),
            execution_time: 0.into(),
        };
        executor.execute_write(&upsert_row).await?;

        // if we are running only the next migration, then we need to stop here
        if only_next {
            break;
        }
    }

    Ok(())
}

/// Reverts the last migration.
pub async fn revert(migration_source: &str, connect_opts: ConnectOpts) -> anyhow::Result<()> {
    let mut files =
        fs::read_dir(migration_source).context("Unable to read migrations directory")?;

    // if target_version is specified, then we need to check if we are already at the target version
    let session = create_session([connect_opts.database_url], Some(connect_opts.keyspace)).await?;
    let executor = Executor::<MigrationQueries>::new(session).await?;

    let current_version = if let Some(v) = executor.execute_read(&GetLatestVersion {}).await? {
        v.version
    } else {
        println!("No migrations to revert");
        return Ok(());
    };

    // only revert the last migration
    let migration = if let Some(f) = files
        .find(|f| {
            let file = f.as_ref().unwrap();
            let file_name = file.file_name();
            let file_name = file_name.to_string_lossy();

            let migration_folder = MigrationFolder::try_from(file_name.clone()).unwrap();

            migration_folder.version == current_version
        })
        .transpose()?
    {
        f
    } else {
        println!("Couldn't find migration ({}) to revert.", current_version);
        return Ok(());
    };
    let migration = MigrationFolder::try_from(migration.file_name().to_string_lossy())?;

    // now, for every migration folder, we need to run the up file
    let mut path = std::path::PathBuf::new();
    path.push(migration_source);
    path.push(&migration.to_string());

    let mut up_path = path.clone();
    up_path.push(MigrationType::Up.name());

    let contents = fs::read_to_string(&up_path).context("Unable to read up file")?;
    let statements = contents.split("\n\n").collect::<Vec<_>>();
    let session = &executor.session;

    println!(
        "Reverting migration {}...",
        style(&migration.to_string()).cyan(),
    );

    let start = std::time::Instant::now();
    let count = statements.len();
    for (i, statement) in statements.into_iter().enumerate() {
        let mut prepared_query = Query::new(statement);
        prepared_query.set_tracing(true);
        prepared_query.set_serial_consistency(Some(SerialConsistency::LocalSerial));

        let start = std::time::Instant::now();
        session.query(prepared_query.clone(), ()).await?;
        let end = std::time::Instant::elapsed(&start);

        println!(
            "Reverted statement {} ({}) ({})",
            style(i).green(),
            style(format!("of {count}")).dim(),
            style(format!("{}ms", end.as_millis())).dim()
        );
    }
    let end = std::time::Instant::elapsed(&start);

    println!(
        "Reverted migration {} ({})",
        style(&migration.to_string()).cyan(),
        style(format!("{}ms", end.as_millis())).dim()
    );

    executor
        .execute_write(&DeleteByVersion {
            version: migration.version,
        })
        .await?;

    Ok(())
}
