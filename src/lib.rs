use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pattern: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Not enough arguments"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { pattern, filename, case_sensitive })
    }
}

fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|x| x.contains(pattern))
        .collect()
}

fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results;
    if config.case_sensitive {
        results = search(&config.pattern, &contents);
    }
    else {
        results = search_case_insensitive(&config.pattern, &contents);
    }

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let pattern = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(pattern, content)
        );
    }

    #[test]
    fn test_case_insensitive_search() {
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
