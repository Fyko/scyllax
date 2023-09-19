use std::ops::{Deref, Not};

use clap::{Args, Parser};
use clap_complete::Shell;

/// Parses options for the CLI.
#[derive(Parser, Debug)]
#[clap(version, about, author)]
pub struct Opt {
    /// The parsed command
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Parser, Debug)]
pub enum Command {
    #[clap(alias = "mig")]
    Migrate(MigrateOpt),

    /// Generate shell completions for the specified shell
    Completions { shell: Shell },
}

/// Group of commands for creating and running migrations.
#[derive(Parser, Debug)]
pub struct MigrateOpt {
    #[clap(subcommand)]
    pub command: MigrateCommand,
}

#[derive(Parser, Debug)]
pub enum MigrateCommand {
    /// Create a new migration with the given description.
    ///
    /// A version number will be automatically assigned to the migration.
    ///
    /// Example: scyllax-cli mig add create users table
    Add {
        description: String,

        #[clap(flatten)]
        source: Source,
    },

    /// Run all pending migrations.
    Run {
        #[clap(flatten)]
        source: Source,

        /// Run only the next pending migration
        #[clap(long)]
        next: bool,

        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },

    /// Revert the latest migration with a down file.
    Revert {
        #[clap(flatten)]
        source: Source,

        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },

    /// List all available migrations.
    Info {
        #[clap(flatten)]
        source: Source,

        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },
}

/// Argument for the migration scripts source.
#[derive(Args, Debug)]
pub struct Source {
    /// Path to folder containing migrations.
    #[clap(long, default_value = "migrations")]
    source: String,
}

impl Deref for Source {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

/// Argument for the database URL.
#[derive(Args, Debug)]
pub struct ConnectOpts {
    /// Location of the DB, by default will be read from the DATABASE_URL env var
    #[clap(long, short = 'D', env)]
    pub database_url: String,

    /// The keyspace to store migrations information.
    #[clap(long, short = 'K', default_value = "scyllax_migrations")]
    pub keyspace: String,

    /// The maximum time, in seconds, to try connecting to the database server before
    /// returning an error.
    #[clap(long, default_value = "10")]
    pub connect_timeout: u64,
}

/// Argument for ignoring applied migrations that were not resolved.
#[derive(Args, Copy, Clone, Debug)]
pub struct IgnoreMissing {
    /// Ignore applied migrations that are missing in the resolved migrations
    #[clap(long)]
    ignore_missing: bool,
}

impl Deref for IgnoreMissing {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        &self.ignore_missing
    }
}

impl Not for IgnoreMissing {
    type Output = bool;

    fn not(self) -> Self::Output {
        !self.ignore_missing
    }
}
