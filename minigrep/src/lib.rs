use std::error::Error;
use std::fs;
use std::env;
use std::env::Args;

pub struct Parameter {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Parameter {
    // Returning a Result from new Instead of Calling panic!
    pub fn new(args: &[String]) -> Result<Parameter, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Parameter {
            query,
            filename,
            case_sensitive,
        })
    }

    // Removing a clone Using an Iterator
    pub fn build(mut args: Args) -> Result<Parameter, &'static str> {
        // skip the name of program
        args.next();

        let query = match args.next() {
            Some(query) => query,
            None => return Err("Query string not found"),
        };

        let filename = match args.next() {
            Some(filename) => filename,
            None => return Err("Filename string not found")
        };

        let case_sensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Parameter {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(parameter: Parameter) -> Result<(), Box<dyn Error>> {
    let query = parameter.query;
    let contents = fs::read_to_string(parameter.filename)?;

    let results = if parameter.case_sensitive {
        search(&query, &contents)
    } else {
        search_case_insensitive(&query, &contents)
    };

    for line in results {
        println!("{},", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // Making Code Clearer with Iterator Adaptors
    contents.lines().filter(|line| {
        line.contains(query)
    }).collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query.to_lowercase()) {
    //         results.push(line);
    //     }
    // }
    // results

    // Making Code Clearer with Iterator Adaptors
    contents.lines().filter(|line| {
        line.to_lowercase().contains(&query.to_lowercase())
    }).collect()
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
Pick three.";

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
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}