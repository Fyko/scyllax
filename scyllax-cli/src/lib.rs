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
                cluster,
                source,
                description,
            } => migrate::add(&source, &description, cluster).await?,
            MigrateCommand::Init { connect_opts } => migrate::init(connect_opts).await?,
            MigrateCommand::Run {
                source,
                next,
                all,
                cluster,
                connect_opts,
            } => migrate::run(&source, next, all, cluster, connect_opts).await?,
            MigrateCommand::Revert {
                cluster,
                source,
                connect_opts,
            } => migrate::revert(&source, cluster, connect_opts).await?,
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
