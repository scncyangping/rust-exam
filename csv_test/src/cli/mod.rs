pub mod b64;
pub mod cli_csv;
pub mod gen_pass;
use crate::process::{b64::{b64_decode, b64_encode}, csv_convert::do_convert, gen_pass::gen_pass_with_length};
use b64::Base64OpCmd;
use clap::{Parser, Subcommand};
use cli_csv::CsvOpts;
use gen_pass::GenPass;

// template csv -i input.csv -o out.json -d ','
#[derive(Debug, Parser)]
#[command(version, about, long_about = "long about")]
pub struct Options {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "csv", about = "csv command des")]
    Csv(CsvOpts),
    #[command(name = "genpass")]
    GeneratePassword(GenPass),
    #[clap(name = "b64", subcommand)]
    Base64(Base64OpCmd),
}

pub fn do_match() -> anyhow::Result<()> {
    let options = Options::parse();
    match options.command {
        Some(c) => match c {
            Commands::Csv(csv_option) => do_convert(csv_option),
            Commands::GeneratePassword(gen_pass) => gen_pass_with_length(gen_pass.length),
            Commands::Base64(base64_op_cmd) => match base64_op_cmd {
                Base64OpCmd::Encode(base_cmd) => b64_encode(&base_cmd.input),
                Base64OpCmd::Decode(base_cmd) => b64_decode(&base_cmd.input),
            },
        },
        None => todo!(),
    }
}
