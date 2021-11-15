use std::env;
use std::error::Error;
use std::fs;

// a struct to hold the config
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // a function to instantiate the config struct
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // get the case sensitive environment variable
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// a function to call the search function with config
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// a function to do the search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // loop through each line in the file
    for line in contents.lines() {
        // does the line contain the query?
        if line.contains(query) {
            // if so, add it to the results
            results.push(line);
        }
    }

    results
}

// a function to do the search case insensitively
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // make query lowercase
    let query = query.to_lowercase();
    let mut results = Vec::new();

    // loop through each line in the file
    for line in contents.lines() {
        // does the line (lowercase) contain the query?
        if line.to_lowercase().contains(&query) {
            // if so, add it to the results
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    // a test to run the search function
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

    // a test to run the search function case insensitively
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
