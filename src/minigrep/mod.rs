use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Unable to get query string.")
        };
        let file_name = match args.next() {
            Some(file_name) => file_name,
            None => return Err("Unable to get file name.")
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, file_name, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(config.file_name)?;
    let mut context = String::new();

    file.read_to_string(&mut context)?;

    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &context)
    } else {
        search_case_insensitive(&config.query, &context)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    context.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    context.lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}