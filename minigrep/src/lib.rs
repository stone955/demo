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
    let contents = fs::read_to_string(parameter.filename)?;
    println!("Text: \n{}", contents);
    Ok(())
}