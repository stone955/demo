fn main() {
    // let str = String::from("My name is stone. ");
    let mut str = String::from("My name is stone. ");
    // Length
    println!("Length {}", str.len());
    // Is Empty?
    println!("String is empty? {}", str.is_empty());
    // split_whitespace
    for token in str.split_whitespace() {
        println!("{}", token);
    }
    // contains
    println!(
        "Does the string contains 'stone'? {}",
        str.contains("stone")
    );
    // append
    str.push_str("Welcome to learn rust strings.");
    println!("{}", str);

    // string method
    // Replace
    let before_str = String::from("Java is great!");
    let after_str = before_str.replace("Java", "Golang");
    println!(
        "Before replace: {}. After replace: {}",
        before_str, after_str
    );

    // Lines
    
}
