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
