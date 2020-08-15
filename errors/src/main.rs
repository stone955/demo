/*
Rust groups errors into two major categories: recoverable and unrecoverable errors.
For a recoverable error, such as a file not found error, it's reasonable to report
the problem to the user and retry the operation.
For a unrecoverable error, such as bugs, like trying to access a location beyond
the end of an array.

Rust doesn't have exception. Instead, it has the type Result<T,E> for recoverable errors
and the panic! macro that stops execution when the program encounters an unrecoverable error.


Unrecoverable Errors with panic!


Recoverable Errors with Result

Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
enum Result<T, E> {
    Ok(T),
    Err(E),
}
Because Result has these generic type parameters, we can use the Result type and the functions that
the standard library has defined on it in many different situations where the successful value and
error value we want to return may differ.


 */

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;

#[allow(unused_variables)]
fn main() {
    // unrecoverable error panic!
    // {
    //     panic!("crash and burn");
    // }

    // recoverable error with Result<T, E>
    {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => {
                file
            }

            // matching on different errors
            // Err is io::Error
            Err(e) => match e.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Failed to create the file. Error {:?}", e)
                },
                _ => {
                    panic!("Failed to open the file. Error {:?}", e)
                }
            }
        };
    }

    // another
    {
        let f = File::open("world.txt").unwrap_or_else(|e| {
            if e.kind() == ErrorKind::NotFound {
                File::create("world.txt").unwrap_or_else(|e| {
                    panic!("Failed to create the file. Error {:?}", e);
                })
            } else {
                panic!("Failed to open the file. Error {:?}", e);
            }
        });
    }

    // Shortcuts for Panic on Error: unwrap and expect
    {
        let f = File::open("hello.txt").unwrap();

        let f = File::open("world.txt").expect("Failed to open the file");
    }

    // Propagating Errors
    {
        fn read_username_from_file() -> Result<String, io::Error> {
            let f = File::open("hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e)
            };

            let mut s = String::new();

            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e)
            }
        }

        match read_username_from_file() {
            Ok(s) => println!("Username: {}", s),
            Err(e) => panic!("Failed to read username. Error {:?}", e)
        }
    }

    // Shortcut for Propagating Errors. The ? operator
    // The ? operator can only be used in a function that return
    // `Result` or `Option` (or another type that implements `std::ops::Try`)
    {
        fn read_language_from_file() -> Result<String, io::Error> {
            // let mut f = File::open("world.txt")?;
            // let mut s = String::new();
            // f.read_to_string(&mut s)?;
            // Ok(s)

            let mut s = String::new();
            File::open("world.txt")?.read_to_string(&mut s)?;
            Ok(s)
        }

        match read_language_from_file() {
            Ok(s) => println!("Language: {}", s),
            Err(e) => panic!("Failed to read language. Error {:?}", e)
        }
    }
}
