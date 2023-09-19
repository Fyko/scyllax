//! the cli bin itself
use clap::Parser;
use console::style;
use scyllax_cli::Opt;

/// runs the cli
#[tokio::main]
async fn main() {
    // no special handling here
    if let Err(error) = scyllax_cli::run(Opt::parse()).await {
        println!("{} {}", style("error:").bold().red(), error);
        #[allow(clippy::exit)]
        std::process::exit(1);
    }
}
