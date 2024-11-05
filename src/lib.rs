use std::env::VarError;
use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        };
        let query = args[1].clone();
        let file_path = args[2].clone();
        let case_sensitive = match env::var("IGNORE_CASE") {
            Ok(flag) => match flag.as_str() {
                "1" => false,
                _ => true,
            },
            Err(_) => match args.get(3) {
                None => false,
                Some(arg) => match arg.as_str() {
                    "ig" | "igc" | "ignore" | "ignorecase" | "ignore_case" => true,
                    _ => false,
                },
            },
        };

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
}
