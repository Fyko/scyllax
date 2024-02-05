use anyhow::{Context, Result};
use console::style;
use scylla::query::Query;
use scyllax::executor::create_session;
use std::{
    borrow::Cow,
    fs::{self, read_dir, File},
};
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

pub async fn add(migration_source: &str, description: &str, cluster: Option<String>) -> anyhow::Result<()> {
    let mode = MigrationSource::from_path(migration_source)?;
    tracing::debug!("mode: {mode:?}");
    if mode == MigrationSource::Clustered && cluster.is_none() {
        println!("You must specify a cluster name when using cluster migrations ({migration_source}/**/**/{{up,down}}.cql)");
        return Ok(());
    }

    let migration_source = if let Some(cluster) = cluster {
        format!("{}/{}", migration_source, cluster)
    } else {
        migration_source.to_string()
    };

    fs::create_dir_all(&migration_source).context("Unable to create migrations directory")?;

    let format = format_description::parse("[year][month][day][hour][minute][second]")?;
    let ts = time::OffsetDateTime::now_utc().format(&format)?;

    let folder = format!("{}_{}", ts, description.replace(' ', "_"));
    eprintln!("Creating {}", style(format!("{migration_source}/{folder}")).cyan());
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

#[derive(Debug, Eq, PartialEq)]
enum MigrationSource {
    Flat,
    Clustered,
}

impl MigrationSource {
    fn from_path(migration_source: &str) -> Result<MigrationSource> {
        // check if the migration_source has two levels of folders
        // 1 level ($migration_source/**/up.cql)
        // 2 levels ($migration_source/**/**/up.cql)
        let Some(Ok(mut first_folder)) = fs::read_dir(migration_source)?.next() else {
            return Ok(MigrationSource::Flat);
        };

        for mode in [MigrationSource::Flat, MigrationSource::Clustered] {
            let mut members = fs::read_dir(first_folder.path())?;
            // if any of the members end with .cql, then we have a flat layout
            if members.any(|m| m.unwrap().path().to_string_lossy().ends_with(".cql")) {
                return Ok(mode);
            }

            // i wonder if there's a better way to do this without reopening the iterator
            if let Some(next) = fs::read_dir(first_folder.path())?.next() {
                first_folder = next.unwrap();
            } else {
                panic!("{migration_source} is empty. Consider adding some stuff to it so the migration mode can be determined.");
                // this might happen if the folder is empty
                // return Ok(MigrationSource::Flat);
            }
        }

        panic!("Bad migration layout")
    }

    fn sources(&self, migration_source: &str) -> Result<Vec<String>> {
        if self == &MigrationSource::Clustered {
            Ok(vec![migration_source.to_string()])
        } else
        /*if mode == MigrationSource::Flat*/
        {
            let files = fs::read_dir(migration_source)?
                .filter_map(|f| f.ok())
                .filter(|f| f.path().is_dir())
                .map(|f| f.path().to_string_lossy().to_string())
                .collect::<Vec<_>>();
    
            Ok(files)
        }
    }
}

/// Runs all pending migrations.
#[tracing::instrument]
pub async fn run(
    migration_source: &str,
    only_next: bool,
    all: bool,
    cluster: Option<String>,
    connect_opts: ConnectOpts,
) -> anyhow::Result<()> {
    let mode = MigrationSource::from_path(migration_source)?;
    tracing::debug!("mode: {mode:?}");

    if mode == MigrationSource::Clustered && cluster.is_none() {
        println!("You must specify a cluster name when using cluster migrations ({migration_source}/**/**/{{up,down}}.cql)");
        println!("To apply all migrations from all clusters to this one, use the --all flag (usually development only)");
        return Ok(());
    }

    let sources = mode.sources(migration_source)?;

    let executor =
        create_migration_executor(connect_opts.scylla_nodes, connect_opts.keyspace).await?;

    let current_version = executor
        .execute_read(GetLatestVersion {})
        .await?
        .map_or(-1, |v| v.version);

    for source in sources {
        tracing::info!("Running migrations for {}", source);
        

        let migrator =
            Migrator::<UpMigration>::new(executor.clone(), &source, current_version);

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
            continue; // todo: remove me
            migration_folder.execute(&migrator).await?;

            // if we are running only the next migration,
            // then we need to stop here
            if only_next {
                break;
            }
        }
    }

    Ok(())
}

/// Reverts the last migration.
pub async fn revert(migration_source: &str, cluster: Option<String>, connect_opts: ConnectOpts) -> anyhow::Result<()> {
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

    let session = create_session(connect_opts.scylla_nodes.split(','), None::<&str>).await?;

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
