fn main() {
    {
        // 1.loop
        println!("----------loop----------");
        let mut n = 0;
        loop {
            n += 1;
            // break
            if n > 10 {
                break;
            }
            println!("The value of n is {}", n);
        }

        // loop with return value
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter > 10 {
                break counter * 2;
            }
        };
        println!("The value of counter is {}", result);
    }

    {
        // 2.while-loop
        println!("----------while----------");
        let mut m = 0;
        while m < 10 {
            println!("The value of m is {}", m);
            m += 1;
        }
    }

    {
        // 3.for ... in ...
        println!("----------for...in...----------");
        let numbers = 0..10;
        for i in numbers {
            println!("The number is {}", i);
        }
        let animals = vec!["Rabbit", "Dog", "Cat", "Bird"];
        for a in animals.iter() {
            println!("The animal is {}", a);
        }
        for (i, a) in animals.iter().enumerate() {
            println!("The index is {}, the animal is {}", i, a);
        }
    }
}
