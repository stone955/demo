extern crate rand;
use rand::Rng;

fn main() {
    let rand_number = rand::thread_rng().gen_range(0, 10);
    println!("Random number: {}", rand_number);

    let rand_bool = rand::thread_rng().gen_weighted_bool(2);
    println!("Random boolean: {}", rand_bool);
}
