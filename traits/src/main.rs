struct Person {
    name: String,
    age: u8,
}

// Person implements trait ToString
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("name: {}, age: {}", self.name, self.age);
    }
}

fn main() {
    let p = Person {
        name: String::from("Stone"),
        age: 35,
    };

    println!("Person {}", p.to_string());
}
