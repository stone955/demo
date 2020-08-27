/*
- Closures, a function-like construct you can store in a variable
- Iterators, a way of processing a series of elements
- How to use these two features to improve the I/O project in Chapter 12
- The performance of these tow features

- Closures: Anonymous Functions that Can Capture Their Environment

 */

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

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
    // {
    //     let expensive_closure = |num: u32| -> u32 {
    //         println!("calculating very slowly...");
    //         thread::sleep(Duration::from_secs(2));
    //         num * 3
    //     };
    //
    //     if intensity < 25 {
    //         println!("Today, do {} push!", expensive_closure(intensity));
    //         println!("Next, do {} sit!", expensive_closure(intensity));
    //     } else {
    //         if random_number == 3 {
    //             println!("Take a break today! Remember to stay hydrated!");
    //         } else {
    //             println!("Today, run for {} minutes!", expensive_closure(intensity));
    //         }
    //     }
    // }

    // Storing Closures Using Generic Parameters and the Fn Traits
    // {
    //     let mut cache_holder = CacheHolder::new(|num: u32| -> u32{
    //         println!("calculating very slowly...");
    //         thread::sleep(Duration::from_secs(2));
    //         num * 5
    //     });
    //     if intensity < 25 {
    //         println!("Today, do {} push!", cache_holder.get_value(intensity));
    //         println!("Next, do {} sit!", cache_holder.get_value(intensity));
    //     } else {
    //         if random_number == 3 {
    //             println!("Take a break today! Remember to stay hydrated!");
    //         } else {
    //             println!("Today, run for {} minutes!", cache_holder.get_value(intensity));
    //         }
    //     }
    // }

    {
        let mut hashmap_cache_holder = HashMapCacheHolder::new(|num: u32| -> u32{
            println!("calculating very slowly...");
            thread::sleep(Duration::from_secs(2));
            num * 5
        });
        if intensity < 25 {
            println!("Today, do {} push!", hashmap_cache_holder.get_value(intensity));
            println!("Next, do {} sit!", hashmap_cache_holder.get_value(intensity));
        } else {
            if random_number == 3 {
                println!("Take a break today! Remember to stay hydrated!");
            } else {
                println!("Today, run for {} minutes!", hashmap_cache_holder.get_value(intensity));
            }
        }
    }
}

// CacheHolder<T> struct that holds a closure and an optional result value
struct CacheHolder<T>
    where T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

//
impl<T> CacheHolder<T>
    where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> CacheHolder<T> {
        CacheHolder {
            calculation,
            value: None,
        }
    }

    fn get_value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// HashMapCacheHolder<T> struct that holds a closure and an optional result value
struct HashMapCacheHolder<T>
    where T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashMap<u32, u32>,
}

//
impl<T> HashMapCacheHolder<T>
    where T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> HashMapCacheHolder<T> {
        HashMapCacheHolder {
            calculation,
            values: HashMap::new(),
        }
    }

    fn get_value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
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

    // Comparing Performance: Loops vs. Iterators
}

#[test]
fn call_with_different_values() {
    let mut hashmap_cache_holder = HashMapCacheHolder::new(|x: u32| -> u32{
        x
    });

    let _ = hashmap_cache_holder.get_value(1);
    let v2 = hashmap_cache_holder.get_value(2);

    assert_eq!(v2, 2);
}


