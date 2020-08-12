struct Person {
    name: String,
    age: u8,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(&self) -> u32 {
        self.width * self.height
    }

    fn is_bigger(&self, rect: &Rectangle) -> bool {
        self.get_area() > rect.get_area()
    }

    fn from(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }
}

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

    // an example program using structs
    {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        println!("Rectangle: {:?}", rect);

        println!("Rectangle: {:#?}", rect);
        let area = calcuate_area(&rect);

        println!("Rectangle area is {}", area);
    }

    // method syntax
    {
        let rect = Rectangle {
            width: 30,
            height: 50,
        };

        println!("Rectangle area is {}", rect.get_area());

        let bigger_rect = Rectangle {
            width: 30,
            height: 50,
        };

        let smaller_rect = Rectangle {
            width: 20,
            height: 50,
        };

        println!(
            "bigger_rect is bigger than smaller_rect? {}",
            bigger_rect.is_bigger(&smaller_rect)
        );
    }

    // constructor function of a struct
    {
        let rect = Rectangle::from(40, 50);
        println!("Rectangle: {:#?}", rect);
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

fn calcuate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
