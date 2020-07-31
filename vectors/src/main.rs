fn main() {
    // 第一种声明方法
    let mut v: Vec<u32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.remove(1);
    v.push(5);
    for n in v.iter() {
        println!("Vec::new() {}", n); // 1 3 4 5
    }

    // 第二种声明方法
    let mut v2 = vec![5, 4, 3, 2, 1];
    v2.remove(1);
    v2.push(6);
    for n in v2.iter() {
        println!("vec! {}", n); // 1 3 4 5
    }
}
