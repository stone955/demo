// cargo run {search string} {example-filename}.txt

use std::env;
use std::fs;

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
    let parameter = Parameter::new(&args);
    println!("Searching for {}, in file {}", parameter.query, parameter.filename);

    // read file
    let contents = fs::read_to_string(parameter.filename).expect("Failed to read the file");

    println!("Text: \n{}", contents);

    // This pattern is about separating concerns: main.rs handles running the program,
    // and lib.rs handles all the logic of the task at hand
}

struct Parameter {
    query: String,
    filename: String,
}

impl Parameter {
    // Creating a Constructor for Parameter
    fn new(args: &[String]) -> Parameter {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Parameter {
            query,
            filename,
        }
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
