use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        // let query = args[1].clone();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get query string!")
        };
        // let filename = args[2].clone();
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Did not get filename!!")
        };
        let case_insensitive = !env::var("CASE_INSENSITIVE").is_err();

        let case_insensitive = case_insensitive
            || match args.next() {
                Some(option) => option == "-i",
                None => false,
            };
        Ok(Config {
            query,
            filename,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results: Vec<&str> = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results
    contents.lines().filter(|line| line.contains(query)).collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
