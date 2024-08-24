// #[derive(clap::Parser)]
// #[clap(author, version, about, long_about = None)]
// #[clap(propagate_version = true)]
// pub struct Cli {
//     #[clap(subcommand)]
//     command: Commands,

//     #[clap(long, short, default_value = "/etc/warpgate.yaml", action=ArgAction::Set)]
//     config: PathBuf,

//     #[clap(long, short, action=ArgAction::Count)]
//     debug: u8,
// }

use std::path::PathBuf;

use clap::ArgAction;
#[derive(clap::Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(long,short,action=ArgAction::Count)]
    pub debug: u8,
    #[clap(long, short, default_value = "/etc/warpgate.yaml", action=ArgAction::Set)]
    pub config: PathBuf,
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    Run,
    Test {
        #[clap(long, required = false)]
        data_path: Option<String>,
        #[clap(long, required = false)]
        test2: Option<String>,
    },
    Test1(Test1Args),
}

#[derive(clap::Args)]
pub struct Test1Args {
    #[clap(subcommand)]
    pub command: Option<Test1Command>,
    #[clap(long, short)]
    pub comond2: String,
}

#[derive(clap::Subcommand)]
pub enum Test1Command {
    A,
    B,
}
