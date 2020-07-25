fn main() {
    // 定义函数
    println!("----------function----------");
    // 定义变量x
    let x: u32 = 10;
    print_number_to(x);

    println!("----------statement block----------");
    {
        let y: u32 = 20;
        print_number_to(y);
    }
    // 会报错
    // println!("x is {}, y is {}", x, y);
}

fn print_number_to(num: u32) {
    for n in 0..num {
        if is_even(n) {
            println!("number {} is even", n);
        } else {
            println!("number {} is odd", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}
