/**
*
Managing Growing Projects with Packages, Crates,and Modules

Packages and Crates

A crate is binary or library.
A package is one or more crates that provide a set of functionality.
A package contains a Cargo.toml file that describes how to build those crates.
A package must contain zero or one library crates, and no more. It can contains as many binary crates as you 'd like,
but it must contain at least one crate.

If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary,
both with the same name as the package.

A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.


Defining Modules to Control Scope and Privacy

We will discuss use, pub, as keyword, external packages, and the glob operator.

Modules let us organize code within a crate into groups for readability and easy reuse.
Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal
implementation detail and not available for outside use (private).

*
*/
use restaurant::house::back_of_house;

use rand::Rng;

fn main() {
    // full path
    restaurant::house::front_of_house::add_to_waitlist();
    restaurant::house::front_of_house::seat_at_table();
    restaurant::house::front_of_house::take_order();
    restaurant::house::front_of_house::serve_order();
    restaurant::house::front_of_house::take_payment();

    // use shot path
    let mut breakfast = back_of_house::Breakfast::summer("Rye");
    breakfast.toast = String::from("Ha");

    println!("Breakfast: {:?}", breakfast);

    // extern crate
    let rand_number = rand::thread_rng().gen_range(0, 100);
    println!("Rand number {}", rand_number);
}
