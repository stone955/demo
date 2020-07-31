struct Person {
    name: String,
    age: u8,
}

trait Movable {
    // run
    fn run(&self);
    // walk
    fn walk(&self);
}

// Person implements trait ToString
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("name: {}, age: {}", self.name, self.age);
    }
}

// Person implements trait Movable
impl Movable for Person {
    fn run(&self) {
        if self.age < 40 {
            println!("I can run fastly!");
        } else {
            println!("I can run slowly!");
        }
    }

    fn walk(&self) {
        if self.age < 50 {
            println!("I can walk fastly!");
        } else {
            println!("I can walk slowly!");
        }
    }
}

fn main() {
    let p = Person {
        name: String::from("Stone"),
        age: 35,
    };

    println!("Person {}", p.to_string());

    // 接口实现
    let younger = Person {
        name: String::from("Younger"),
        age: 35,
    };

    younger.run();
    younger.walk();

    let oldder = Person {
        name: String::from("Older"),
        age: 50,
    };
    oldder.run();
    oldder.walk();
}
