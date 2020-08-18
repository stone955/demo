use std::fmt::Display;

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }
    fn get_y(&self) -> &T {
        &self.y
    }
    // fn get_distance_from_origin(&self) -> f32 {
    //     (self.x.powi(2) + self.x.powi(2)).sqrt()
    // }
}

trait Summary {
    // fn summarize(&self) -> String;

    // default implements
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {

    // Removing Duplication by Extracting a Function
    println!("--------------------Removing Duplication by Extracting a Function--------------------");
    {
        let numbers = vec![2, 4, 8, 1, 5, 7, 6, 3, 9, 0];

        let largest = get_largest_i32(&numbers);

        println!("The largest number is {}", largest);
    }

    {
        let numbers = vec![20, 40, 80, 10, 50, 70, 60, 30, 90, 0];

        let largest = get_largest_i32(&numbers);

        println!("The largest number is {}", largest);
    }

    // Generic Data Types
    println!("--------------------Generic Data Types--------------------");

    {
        let chars = vec!['y', 'm', 'a', 'q'];

        let largest = get_largest_char(&chars);

        println!("The largest char is {}", largest);
    }

    {
        let numbers = vec![20, 40, 80, 10, 50, 70, 60, 30, 90, 0];

        let largest = get_largest(&numbers);

        println!("The largest number is {}", largest);

        let chars = vec!['y', 'm', 'a', 'q'];

        let largest = get_largest(&chars);

        println!("The largest char is {}", largest);
    }

    // Generic In Structs Definitions
    println!("--------------------Generic In Structs Definitions--------------------");
    {
        let p = Point {
            x: 5,
            y: 5,
        };

        println!("The point's x: {}, y: {}", p.x, p.y);
    }

    // Generic In Method Definitions
    println!("--------------------Generic In Method Definitions--------------------");
    {
        let p = Point {
            x: 50,
            y: 50,
        };

        println!("The point's x: {}, y: {}", p.get_x(), p.get_y());
    }

    // Traits: Defining Shared Behavior
    println!("--------------------Traits: Defining Shared Behavior--------------------");
    {
        let tweet = Tweet {
            username: String::from("stone"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("oh!oh!oh!"),
            content: String::from("of course, as you probably already know, people"),
            author: String::from("stone"),
            location: String::from("china"),
        };

        println!("1 new article: {}", article.summarize());

        // traits as parameters
        notify(&tweet);
        notify(&article);
    }
}

fn get_largest_i32(numbers: &[i32]) -> i32 {
    let mut largest = numbers[0];

    for &number in numbers {
        if number > largest {
            largest = number;
        }
    }

    largest
}

fn get_largest_char(chars: &[char]) -> char {
    let mut largest = chars[0];

    for &c in chars {
        if c > largest {
            largest = c;
        }
    }

    largest
}

fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// traits as parameters
// fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// trait bound syntax
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
fn notify_and_display<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// traits as return value
// but wouldn't work
fn returns_summarizer(is_tweet: bool) -> impl Summary {
    if is_tweet {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    } else {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("stone"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    }
}