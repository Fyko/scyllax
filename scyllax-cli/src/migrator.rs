use std::{
    borrow::Cow,
    fs::ReadDir,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use console::style;
use scylla::{frame::value::Timestamp, query::Query};
use scyllax::prelude::Executor;
use sha2::{Digest, Sha384};

use crate::model::{DeleteByVersion, MigrationQueries, UpsertMigration};

/// A marker struct provided to the [`Migrator`] to indicate that it should run up migrations.
pub struct UpMigration;
trait UpMigrationExt {}
impl UpMigrationExt for UpMigration {}

/// A marker struct provided to the [`Migrator`] to indicate that it should run down migrations.
pub struct DownMigration;
trait DownMigrationExt {}
impl DownMigrationExt for DownMigration {}

#[async_trait]
pub trait MigrationMode {
    /// A function ran in [`MigrationFolderIterator`] to determin if the folder should be yielded.
    ///
    /// For up migrations, this is if the version is greater than the current version.
    ///
    /// For down migrations, this is if the version is equal to the current version.
    fn should_yield_folder(version: i64, current_version: i64) -> bool;

    /// The name of the file to look for in the migration folder.
    fn name() -> &'static str;

    /// The default contents of the file to create.
    fn file_content() -> &'static str;

    /// The initial log message to print when starting the migration.
    fn initial_log(migration: &MigrationFolder<Self>)
    where
        Self: Sized;

    /// The log message to print when a statement is executed.
    fn per_statement_log(index: usize, count: usize, time: u128)
    where
        Self: Sized;

    /// The log message to print when the migration is completed.
    fn completed_log(migration: &MigrationFolder<Self>, time: u128)
    where
        Self: Sized;

    /// The final steps to run after the migration has been completed, such as inserting a row into
    /// the migration table.
    async fn final_steps(
        migrator: &Migrator<Self>,
        migration: &MigrationFolder<Self>,
        contents: &str,
    ) -> Result<()>
    where
        Self: Sized;
}

#[async_trait]
impl MigrationMode for UpMigration {
    fn name() -> &'static str {
        "up.cql"
    }

    fn file_content() -> &'static str {
        "-- CREATE TABLE IF NOT EXISTS test (id int PRIMARY KEY);"
    }

    fn should_yield_folder(version: i64, current_version: i64) -> bool
    where
        Self: Sized,
    {
        version > current_version
    }

    fn initial_log(migration: &MigrationFolder<Self>) {
        println!(
            "Applying migration {}...",
            style(migration.to_string()).cyan(),
        );
    }

    fn per_statement_log(index: usize, count: usize, time: u128) {
        println!(
            "Applied statement {} ({}) ({})",
            style(index).green(),
            style(format!("of {count}")).dim(),
            style(format!("{}ms", time)).dim()
        );
    }

    fn completed_log(migration: &MigrationFolder<Self>, time: u128) {
        println!(
            "Applied migration {} ({})",
            style(migration.to_string()).cyan(),
            style(format!("{}ms", time)).dim()
        );
    }

    async fn final_steps(
        migrator: &Migrator<Self>,
        migration: &MigrationFolder<Self>,
        contents: &str,
    ) -> Result<()> {
        let checksum: Vec<u8> = Vec::from(Sha384::digest(contents.as_bytes()).as_slice());

        let upsert_row = UpsertMigration {
            bucket: 0,
            version: migration.version,
            description: migration.description.clone().into(),
            installed_on: Timestamp(chrono::Duration::seconds(
                time::OffsetDateTime::now_utc().unix_timestamp(),
            ))
            .into(),
            success: true.into(),
            checksum: checksum.into(),
            execution_time: 0.into(),
        };

        migrator.executor.execute_write(upsert_row).await?;

        Ok(())
    }
}

#[async_trait]
impl MigrationMode for DownMigration {
    fn name() -> &'static str {
        "down.cql"
    }

    fn file_content() -> &'static str {
        "-- DROP TABLE test;"
    }

    fn should_yield_folder(version: i64, current_version: i64) -> bool
    where
        Self: Sized,
    {
        version == current_version
    }

    fn initial_log(migration: &MigrationFolder<Self>) {
        println!(
            "Reverting migration {}...",
            style(migration.to_string()).cyan(),
        );
    }

    fn per_statement_log(index: usize, count: usize, time: u128) {
        println!(
            "Reverted statement {} ({}) ({})",
            style(index).green(),
            style(format!("of {count}")).dim(),
            style(format!("{}ms", time)).dim()
        );
    }

    fn completed_log(migration: &MigrationFolder<Self>, time: u128) {
        println!(
            "Reverted migration {} ({})",
            style(migration.to_string()).cyan(),
            style(format!("{}ms", time)).dim()
        );
    }

    async fn final_steps(
        migrator: &Migrator<Self>,
        migration: &MigrationFolder<Self>,
        _contents: &str,
    ) -> Result<()> {
        migrator
            .executor
            .execute_write(DeleteByVersion {
                version: migration.version,
            })
            .await?;

        Ok(())
    }
}

/// A migration folder, which contains the version and description of the migration.
#[derive(Debug, Clone, PartialEq)]
pub struct MigrationFolder<K: MigrationMode> {
    /// The version (timestamp) of the migration
    pub version: i64,
    /// The description of the migration
    pub description: String,

    _marker: std::marker::PhantomData<K>,
}

impl<K: MigrationMode> MigrationFolder<K> {
    /// Execute a migration, up or down.
    pub async fn execute(&self, migrator: &Migrator<K>) -> Result<()> {
        K::initial_log(self);

        let mut path = std::path::PathBuf::new();
        path.push(&*migrator.path);
        path.push(&self.to_string());

        let mut kind_path = path.clone();
        kind_path.push(K::name());

        let contents = std::fs::read_to_string(kind_path).context("Unable to read up file")?;
        let statements = contents.split("\n\n").collect::<Vec<_>>();
        let session = &migrator.executor.session;

        let start = Instant::now();
        let count = statements.len();
        for (i, statement) in statements.into_iter().enumerate() {
            let query = Query::new(statement);

            let start = std::time::Instant::now();
            session.query(query, ()).await?;
            let end = std::time::Instant::elapsed(&start);

            K::per_statement_log(i, count, end.as_millis());
        }

        let end = std::time::Instant::elapsed(&start);
        K::completed_log(self, end.as_millis());
        K::final_steps(migrator, self, &contents).await?;

        Ok(())
    }
}

impl<K: MigrationMode> TryFrom<Cow<'_, str>> for MigrationFolder<K> {
    type Error = anyhow::Error;

    fn try_from(value: Cow<'_, str>) -> Result<Self, Self::Error> {
        let mut parts = value.split('_');
        let version = parts.next().unwrap().parse::<i64>()?;
        let description = parts.collect::<Vec<&str>>().join(" ");

        Ok(Self {
            version,
            description,
            _marker: std::marker::PhantomData,
        })
    }
}

impl<K: MigrationMode> ToString for MigrationFolder<K> {
    /// Convert the migration folder to a string, which is the version and description joined by an
    /// underscore.
    fn to_string(&self) -> String {
        format!("{}_{}", self.version, self.description.replace(' ', "_"))
    }
}

pub struct Migrator<K: MigrationMode> {
    /// The current version of the database
    pub current_version: i64,
    /// The path to the migrations directory
    pub path: PathBuf,
    /// The executor to use to run the migrations
    pub executor: Arc<Executor<MigrationQueries>>,

    pub _marker: std::marker::PhantomData<K>,
}

impl<K: MigrationMode> Migrator<K> {
    pub fn new(
        executor: Arc<Executor<MigrationQueries>>,
        path: &str,
        current_version: i64,
    ) -> Self {
        let path = Path::new(path).to_owned();
        Self {
            current_version,
            path,
            executor,
            _marker: std::marker::PhantomData,
        }
    }

    /// Creates an iterator over the migration folders.
    pub fn iter(&self) -> MigrationFolderIterator<K> {
        MigrationFolderIterator::new(self.path.read_dir().unwrap(), self.current_version)
    }
}

// create an iterator over the migration folders
impl<K: MigrationMode> IntoIterator for Migrator<K> {
    type Item = anyhow::Result<MigrationFolder<K>>;
    type IntoIter = MigrationFolderIterator<K>;

    fn into_iter(self) -> Self::IntoIter {
        MigrationFolderIterator::new(self.path.read_dir().unwrap(), self.current_version)
    }
}

pub struct MigrationFolderIterator<K: MigrationMode> {
    /// The directory to read from
    read_dir: ReadDir,
    /// The current version of the database
    current_version: i64,
    _marker: std::marker::PhantomData<K>,
}

impl<K: MigrationMode> MigrationFolderIterator<K> {
    /// Create a new iterator over the migration folders.
    pub fn new(read_dir: ReadDir, current_version: i64) -> Self {
        Self {
            read_dir,
            current_version,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<K: MigrationMode> Iterator for MigrationFolderIterator<K> {
    type Item = anyhow::Result<MigrationFolder<K>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let entry = self.read_dir.next()?;
            let file = match entry {
                Ok(file) => file,
                Err(err) => return Some(Err(err.into())),
            };

            let file_name = file.file_name();
            let file_name = file_name.to_string_lossy();

            match MigrationFolder::<K>::try_from(file_name.clone()) {
                Ok(migration_folder) => {
                    if K::should_yield_folder(migration_folder.version, self.current_version) {
                        return Some(Ok(migration_folder));
                    }
                }
                Err(err) => return Some(Err(err)),
            }
        }
    }
}
