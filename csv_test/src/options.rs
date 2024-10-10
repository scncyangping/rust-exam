use std::fs;

use clap::{Parser, Subcommand};

use crate::process::Player;

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
    Csv(CsvOption),
}

#[derive(Debug, Parser)]
pub struct CsvOption {
    #[arg(short, long,value_parser=varify_input_file)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value_t = ',')]
    delemiter: char,
    #[arg(long, default_value_t = true)]
    header: bool,
}

fn varify_input_file(filename: &str) -> Result<String, String> {
    println!("filename: {}", filename);
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("file is not exist".into())
    }
}

pub fn do_match() -> anyhow::Result<()> {
    let options = Options::parse();
    match options.command {
        Some(c) => match c {
            Commands::Csv(csv_option) => {
                // step1. read file from csv
                let mut reader = csv::Reader::from_path(csv_option.input)?;
                let mut data_slice = Vec::with_capacity(128);
                for ele in reader.deserialize() {
                    let player: Player = ele?;
                    data_slice.push(player);
                }
                let json_string = serde_json::to_string_pretty(&data_slice)?;
                fs::write(csv_option.output, json_string)?;
                anyhow::Ok(())
            }
        },
        None => todo!(),
    }
}
