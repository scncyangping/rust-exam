use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub need_upper_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Invalid");
        }
        let query = args[1].clone();

        let filename = args[2].clone();

        // 判断环境变量是否设置了这个值,设置了就是true
        let need_upper_case = env::var("ISUPPER").is_err();
        Ok(Config {
            query: query,
            filename: filename,
            need_upper_case: need_upper_case,
        })
    }
}

pub fn run(query: String, filename: String, upper: bool) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let result = if upper {
        search(query.as_str(), &contents)
    } else {
        search_upper(query.as_str(), &contents)
    };

    for ele in result {
        println!("{}", ele);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_upper<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_uppercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_uppercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search() {
        let query = "mod tests";
        let contents = "
    mod tests {
#[test]
fn test_search() {
    let contents = 
    ";
        assert_eq!(vec!["mod tests {"], search(query, contents))
    }
}
