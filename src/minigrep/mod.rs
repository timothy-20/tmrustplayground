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
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments")
        }

        let query = args[1].clone();
        let file_name = args[2].clone();
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
    let mut results: Vec<&str> = Vec::new();

    for line in context.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, context: &'a str) -> Vec<&'a str> {
    let lowercase_query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in context.lines() {
        if line.to_lowercase().contains(&lowercase_query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    const CONTEXT: &str = "\
Rust:
safe, fas, productive.
Pick three.
Duct tape.";

    #[test]
    fn case_sensitive() {
        let query = "duct";

        assert_eq!(
            vec!["safe, fas, productive."],
            search_case_sensitive(query, CONTEXT)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "DuCt";

        assert_eq!(
            vec!["safe, fas, productive.", "Duct tape."],
            search_case_insensitive(query, CONTEXT)
        );
    }
}