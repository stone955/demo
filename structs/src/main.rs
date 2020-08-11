struct Person {
    name: String,
    age: u8,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // p is immutable
    {
        let p = Person {
            name: String::from("stone"),
            age: 35,
        };

        println!("The person's name is {}, age is {}", p.name, p.age);
    }

    // p is mutable
    {
        let mut p = Person {
            name: String::from("stone"),
            age: 35,
        };

        p.age = 34;

        println!("The person's name is {}, age is {}", p.name, p.age);
    }

    // build function
    {
        let p = build_person(String::from("sidy"), 30);
        println!("The person's name is {}, age is {}", p.name, p.age);
    }

    // using struct update syntax
    {
        let p1 = Person {
            name: String::from("stone"),
            age: 35,
        };

        let p2 = Person {
            name: String::from("sidy"),
            ..p1
        };

        println!("The person's name is {}, age is {}", p2.name, p2.age);
    }

    // Using Tuple Structs without Named Fields to Create Different Types
    {
        let black = Color(0, 0, 0);

        println!("R: {}, E: {}, G: {}", black.0, black.1, black.2);

        let origin = Point(0, 0, 0);

        println!("X: {}, Y: {}, Z: {}", origin.0, origin.1, origin.2);
    }
}

fn build_person(name: String, age: u8) -> Person {
    Person {
        name: name,
        age: age,
    }

    // Using the Field Init Shorthand when Variables and Fields Have the Same Name
    // Person {
    //     name,
    //     age,
    // }
}
