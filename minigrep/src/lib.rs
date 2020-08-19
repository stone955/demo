use std::error::Error;
use std::fs;

pub struct Parameter {
    pub query: String,
    pub filename: String,
}

impl Parameter {
    // Returning a Result from new Instead of Calling panic!
    pub fn new(args: &[String]) -> Result<Parameter, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Parameter {
            query,
            filename,
        })
    }
}

pub fn run(parameter: Parameter) -> Result<(), Box<dyn Error>> {
    let query = parameter.query;
    let contents = fs::read_to_string(parameter.filename)?;
    for line in search(&query, &contents) {
        println!("{},", line);
    }
    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}