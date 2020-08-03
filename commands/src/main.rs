use std::env;
use std::io;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    for arg in args.iter() {
        println!("{}", arg);
    }

    // 获取输入
    let mut input = String::new();

    println!("Hi mate! Please input message:");

    // 传指针
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("Read success! You input: {}", input.to_uppercase());
        }
        Err(e) => {
            println!("God! Read error: {}", e);
        }
    }
}
