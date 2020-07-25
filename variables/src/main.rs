// Direction 定义方向枚举
enum Direction {
    East,
    West,
    North,
    South,
}

const MAX_NUMBER: u8 = 20;

fn main() {
    // 定义不可变变量
    println!("----------不可变变量----------");
    let x = 45; // i32 带符号的32位整数
    println!("The value of x is {}", x);

    // 定义可变变量
    println!("----------可变变量----------");
    let mut y = 60;
    println!("The value of y is {}", y);

    y = 100;
    println!("The value of y is {}", y);

    // 定义变量时指定类型
    println!("----------可变变量指定类型----------");
    let z: i64 = 64; // i64 带符号的64位整数
    println!("The value of z is {}", z);
    let f: f64 = 123.456;
    println!("The value of f is {}", f);

    // 使用枚举
    println!("----------使用枚举----------");
    let directions = [
        Direction::East,
        Direction::North,
        Direction::South,
        Direction::West,
    ];

    for d in directions.iter() {
        match d {
            Direction::East => {
                println!("The direction is east");
            }
            Direction::West => {
                println!("The direction is west");
            }
            Direction::North => {
                println!("The direction is north");
            }
            Direction::South => {
                println!("The direction is south");
            }
        }
    }

    // 使用常量
    println!("----------使用常量----------");
    for i in 0..MAX_NUMBER {
        println!("The number is {}", i);
    }

    // 使用元组
    println!("----------使用元组----------");
    let tup = ("apple", 3.14, true, ("banana", 9.8, false));
    println!("tuple 0 is {}", tup.0);
    println!("tuple 1 is {}", tup.1);
    println!("tuple 2 is {}", tup.2);
    println!("tuple 3.0 is {}", (tup.3).0);
    println!("tuple 3.1 is {}", (tup.3).1);
    println!("tuple 3.2 is {}", (tup.3).2);

    let (a, b, _, d) = tup;
    println!("a is {}", a);
    println!("b is {}", b);
    println!("d.0 is {}", d.0);
}
