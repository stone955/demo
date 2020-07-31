use std::fs::File;
use std::io::prelude::*;

fn main() {
    // 从 info.txt 读
    let mut input_file = File::open("input.txt").expect("Can not open file!");

    let mut contents = String::new();

    input_file
        .read_to_string(&mut contents)
        .expect("Can not read the file!");

    println!("Info contents:\n\n{}", contents);

    // 将读取的内容写到 output.txt
    let mut output_file = File::create("output.txt").expect("Can not create file!");
    output_file
        .write_all(&mut contents.as_bytes())
        .expect("Can not write file!");
}
