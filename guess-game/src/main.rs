use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // 0. Read from user input
    /*
    println!("Guess the number!");

    println!("Please inpput your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("fatal to read line");

    println!("You guessed: {}", guess);
    */

    // 1. Generate a secret number
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0, 100);

    // 可以用二分查找猜数字，最多logN次就能猜到
    loop {
        println!("Please inpput your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("fatal to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
            }
            Ordering::Greater => {
                println!("Too big!");
            }
            Ordering::Equal => {
                println!("Yeah! You win!");
                break;
            }
        }
    }
}
