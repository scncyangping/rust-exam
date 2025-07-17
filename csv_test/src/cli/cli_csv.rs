use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use serde::__private::de::IdentifierDeserializer;

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
    pub input: String,
    #[arg(long = "of", value_parser=output_format, default_value = "json")]
    pub output_format: OutputFormat,
    #[arg(short = 'o', long)]
    pub output: Option<String>,
    #[arg(short = 'd', long, default_value_t = 'd')]
    pub delimiter: char,
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

