//! Scyllax CLI
use anyhow::Result;
use opt::{Command, MigrateCommand};

mod migrate;
mod migrator;
mod model;
mod opt;

pub use opt::Opt;

/// runs the cli
pub async fn run(opt: Opt) -> Result<()> {
    match opt.command {
        Command::Migrate(migrate) => match migrate.command {
            MigrateCommand::Add {
                source,
                description,
            } => migrate::add(&source, &description).await?,
            MigrateCommand::Init { connect_opts } => migrate::init(connect_opts).await?,
            MigrateCommand::Run {
                source,
                next,
                connect_opts,
            } => migrate::run(&source, connect_opts, next).await?,
            MigrateCommand::Revert {
                source,
                connect_opts,
            } => migrate::revert(&source, connect_opts).await?,
            MigrateCommand::Info { .. } => {
                println!("TODO: implement info command");
            }
        },
        Command::Completions { shell } => {
            use std::io;

            use clap::CommandFactory;
            use clap_complete::generate;

            generate(shell, &mut Command::command(), "scyllax", &mut io::stdout());
        }
    };

    Ok(())
}
