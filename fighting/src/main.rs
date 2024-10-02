use anyhow::{Ok, Result};
use clap::Parser;
use cmd::Cli;
use config::load_config;
use logging::init_logging;
use tracing::*;
pub mod cmd;
pub mod config;
pub mod logging;
pub mod protocol;
pub mod util;

async fn _main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(load_config(&cli.config, false).ok().as_ref(), &cli).await;
    let version = env!("CARGO_PKG_VERSION");
    info!(%version, "Fighting");

    match &cli.command {
        cmd::Commands::Run => {
            print!("debug:{},config:{:?}", &cli.debug, &cli.config.as_path());
        }
        cmd::Commands::Test { data_path, test2 } => {
            println!("test: {:?}, {:?}", data_path, test2);
        }
        cmd::Commands::Test1(d) => {
            println!("test1")
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(error) = _main().await {
        error!(?error, "Fatal error");
        std::process::exit(1);
    }
}
