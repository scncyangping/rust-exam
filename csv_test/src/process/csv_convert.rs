use std::collections::HashMap;
use std::fs;
use serde_json::Value;
use crate::cli::cli_csv::{CsvOpts, OutputFormat, Person};

pub fn do_convert(opt: CsvOpts) -> anyhow::Result<()> {
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
    anyhow::Ok(())
}
