use std::error::Error;
use std::{env, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query,
            file_path,
            ignore_case: false,
        }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /* let mut result = Vec::new();
        for line in contents.lines() {
            if line.contains(query) {
                result.push(line);
            }
        }
        result
    */
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
        let mut result = Vec::new();
        for line in contents.lines() {
            if line.to_lowercase().contains(&query.to_lowercase()) {
                result.push(line);
            }
        }
        result
    */
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_sensitive() {
        let query = "three.";
        let contents = "\
            Rust:
safe, fast, productive. 
Pick three.";
        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive(query, contents)
        );
    }
    #[test]
    fn case_insensitive() {
        let query = "Three";
        let contents = "\
            Rust:
safe, fast, productive. 
Pick three.";
        assert_eq!(
            vec!["Pick three."],
            search_case_insensitive(query, contents)
        );
    }
}
