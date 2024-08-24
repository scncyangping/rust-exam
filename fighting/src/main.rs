use anyhow::{Ok, Result};
use clap::Parser;
use cmd::Cli;
use tracing::*;
mod cmd;

async fn _main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        cmd::Commands::Run => {
            print!("debug:{},config:{:?}", &cli.debug, &cli.config.as_path());
        }
        cmd::Commands::Test { data_path, test2 } => {
            println!("test: {:?}, {:?}", data_path, test2);
        }
        cmd::Commands::Test1(d) => {
            //println!("test1: {:?}", d.command);
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
