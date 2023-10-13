//! the cli bin itself
use clap::Parser;
use console::style;
use scyllax_cli::Opt;
use tracing_subscriber::prelude::*;

/// runs the cli
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();
    // no special handling here
    if let Err(error) = scyllax_cli::run(Opt::parse()).await {
        println!("{} {}", style("error:").bold().red(), error);
        #[allow(clippy::exit)]
        std::process::exit(1);
    }
}
