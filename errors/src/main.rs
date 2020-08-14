/*
Rust groups errors into two major categories: recoverable and unrecoverable errors.
For a recoverable error, such as a file not found error, it's reasonable to report
the problem to the user and retry the operation.
For a unrecoverable error, such as bugs, like trying to access a location beyond
the end of an array.

Rust doesn't have exception. Instead, it has the type Result<T,E> for recoverable errors
and the panic! macro that stops execution when the program encounters an unrecoverable error.


Unrecoverable Errors with panic!



 */

fn main() {
    // unrecoverable error panic!
    // {
    //     panic!("crash and burn");
    // }
}
