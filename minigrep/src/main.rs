// cargo run {search string} {example-filename}.txt

use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    // reading the argument values
    let args: Vec<String> = env::args().collect();

    println!("The Args: {:?}", args);

    // [Output] The Args: ["target\\debug\\minigrep.exe", "searchstring", "example-filename.txt"]

    // let (query, filename) = parse_arguments(&args);
    // println!("Searching for {}, in file {}", query, filename);


    // Grouping Configuration Values
    // let parameter = parse_parameter(&args);
    // println!("Searching for {}, in file {}", parameter.query, parameter.filename);

    // Creating a Constructor for Config
    // let parameter = Parameter::new(&args);
    // println!("Searching for {}, in file {}", parameter.query, parameter.filename);

    // Returning a Result from new Instead of Calling panic!
    let parameter = Parameter::new(&args).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}, in file {}", parameter.query, parameter.filename);

    // read file
    // let contents = fs::read_to_string(parameter.filename).expect("Failed to read the file");
    // println!("Text: \n{}", contents);

    // Extracting Logic from main
    // Returning Errors from the run Function
    if let Err(err) = run(parameter) {
        println!("Failed to run: {}", err);
        process::exit(1);
    }

    // This pattern is about separating concerns: main.rs handles running the program,
    // and lib.rs handles all the logic of the task at hand
}

struct Parameter {
    query: String,
    filename: String,
}

impl Parameter {
    // Creating a Constructor for Parameter
    // fn new(args: &[String]) -> Parameter {
    //     if args.len() < 3 {
    //         panic!("not enough arguments");
    //     }
    //     let query = args[1].clone();
    //     let filename = args[2].clone();
    //     Parameter {
    //         query,
    //         filename,
    //     }
    // }

    // Returning a Result from new Instead of Calling panic!
    fn new(args: &[String]) -> Result<Parameter, &'static str> {
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

// fn parse_arguments(args: &[String]) -> (&str, &str) {
//     // saving the argument values in variables
//     let query = &args[1];
//     let filename = &args[2];
//     (query, filename)
// }

// fn parse_parameter(args: &[String]) -> Parameter {
//     // saving the argument values in variables
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Parameter {
//         query,
//         filename,
//     }
// }

// Box<dyn Error> means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
// The dyn keyword is short for “dynamic.”
fn run(parameter: Parameter) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(parameter.filename)?;
    println!("Text: \n{}", contents);
    Ok(())
}
