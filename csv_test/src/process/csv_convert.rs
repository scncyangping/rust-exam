use std::fs;

use crate::cli::cli_csv::{CsvOption, OutputFormat};

pub fn do_convert(csv_option: CsvOption) -> anyhow::Result<()> {
    // step1. read file from csv
    let mut reader = csv::Reader::from_path(csv_option.input)?;
    let headers = reader.headers()?.clone();
    let ret: Vec<serde_json::Value> = reader
        .records()
        .map(|result| {
            let record = result?;
            let r = headers
                .iter()
                .zip(record.iter())
                .collect::<serde_json::Value>();
            Ok(r)
        })
        .collect::<Result<_, csv::Error>>()?;

    let output = match csv_option.format {
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
    };

    fs::write(csv_option.output, output)?;

    anyhow::Ok(())
}
