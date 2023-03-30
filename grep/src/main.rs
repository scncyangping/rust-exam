use std::{env, process};

use grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    grep::run(config.query, config.filename, config.need_upper_case).unwrap_or_else(|err| {
        println!("Run error: {}", err);
        process::exit(1);
    });
}
