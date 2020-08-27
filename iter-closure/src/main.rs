/*
- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- How to use these two features to improve the I/O project in Chapter 12
- The performance of these tow features

- Closures: Anonymous Functions that Can Capture Their Environment

 */

use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity * 2
}

fn generate_workout(intensity: u32, random_number: u32) {
    // Calls simulated_expensive_calculation twice
    // {
    //     if intensity < 25 {
    //         println!("Today, do {} push!", simulated_expensive_calculation(intensity));
    //         println!("Next, do {} sit!", simulated_expensive_calculation(intensity));
    //     } else {
    //         if random_number == 3 {
    //             println!("Take a break today! Remember to stay hydrated!");
    //         } else {
    //             println!("Today, run for {} minutes!", simulated_expensive_calculation(intensity));
    //         }
    //     }
    // }

    // Calls simulated_expensive_calculation only once
    // {
    //     let expensive_result = simulated_expensive_calculation(intensity);
    //     if intensity < 25 {
    //         println!("Today, do {} push!", expensive_result);
    //         println!("Next, do {} sit!", expensive_result);
    //     } else {
    //         if random_number == 3 {
    //             println!("Take a break today! Remember to stay hydrated!");
    //         } else {
    //             println!("Today, run for {} minutes!", expensive_result);
    //         }
    //     }
    // }

    // Using closures to store code, but still calls expensive_closure twice
    {
        let expensive_closure = |num: u32| -> u32 {
            println!("calculating very slowly...");
            thread::sleep(Duration::from_secs(2));
            num * 3
        };

        if intensity < 25 {
            println!("Today, do {} push!", expensive_closure(intensity));
            println!("Next, do {} sit!", expensive_closure(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", expensive_closure(intensity));
            }
        }
    }
}

fn main() {

    // Creating an Abstraction of Behavior with Closures
    {
        let simulated_user_specified_value = 10;
        let simulated_random_number = 7;

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }
}


