use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    command: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[clap(about = "run commands")]
    Csv(CsvOpts),
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(anyhow::Error::msg("unknown output format")),
        }
    }
}

impl From<OutputFormat> for &'static str {
    fn from(fmt: OutputFormat) -> Self {
        match fmt {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short = 'i', long, default_value = "input.csv", value_parser=file_exist_check)]
    input: String,
    #[arg(long = "of", value_parser=output_format, default_value = "json")]
    output_format: OutputFormat,
    #[arg(short = 'o', long)]
    output: Option<String>,
    #[arg(short = 'd', long, default_value_t = 'd')]
    delimiter: char,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Person {
    name: String,
    age: u8,
    #[serde(rename = "Home Address")]
    address: String,
}

pub fn output_format(input: &str) -> Result<OutputFormat, &'static str> {
    input.parse().map_err(|_| "input is not a valid format")
}

pub fn file_exist_check(input: &str) -> Result<String, &'static str> {
    let path = Path::new(input);
    if !path.exists() {
        return Err("file does not exist");
    }
    Ok(input.into())
}
fn main() -> anyhow::Result<()> {
    let cmd = Cmd::parse();
    match cmd.command {
        SubCommand::Csv(opt) => {
            let output = if let Some(ou) = opt.output {
                ou
            } else {
                format!("output.{}", opt.output_format)
            };
            let mut reader = csv::Reader::from_path(opt.input)?;
            let headers = reader.headers()?.clone();
            let data = match opt.output_format {
                OutputFormat::Json | OutputFormat::Yaml => {
                    let mut results = vec![];
                    for result in reader.records() {
                        results.push(headers.iter().zip(result?.iter()).collect::<Value>());
                    }
                    if matches!(opt.output_format, OutputFormat::Json) {
                        serde_json::to_string_pretty(&results)?
                    } else {
                        serde_yaml::to_string(&results)?
                    }
                }
                OutputFormat::Toml => {
                    // toml 格式数据,需要有明确的字段
                    // 并且对于数组来说,需要给一个具体的属性才行
                    let results = reader
                        .deserialize()
                        .filter_map(Result::ok)
                        .collect::<Vec<Person>>();
                    println!("{:?}", results);
                    let mut map = HashMap::<String, Vec<Person>>::new();
                    map.insert("Persons".to_string(), results);
                    toml::to_string(&map)?
                }
            };
            fs::write(output, data)?;
        }
    }
    anyhow::Ok(())
}
