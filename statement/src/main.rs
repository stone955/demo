fn main() {
    // if... else if... else...
    let n = 50;
    if n == 30 {
        println!("The number is equals 30.");
    } else if n < 30 {
        println!("The number is less than 30.");
    } else {
        println!("The number is more than 30.");
    }

    // match(switch)

    let age = 61;

    match age {
        0..=18 => println!("The age {} is juvenile!", age),
        19..=40 => println!("The age {} is youth!", age),
        41..=60 => println!("The age {} is middle-aged!", age),
        _ => println!("The age {} is oldder!", age),
    }
}
