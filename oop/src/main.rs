// 定义一个矩形
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 计算面积
    fn get_area(&self) -> u32 {
        return self.height * self.width;
    }

    // 打印输出
    fn print_to_string(&self) {
        println!("Rectangle: {} * {}", self.width, self.height);
    }

    // 是否是正方形
    fn is_square(&self) -> bool {
        return self.width == self.height;
    }
}

fn main() {
    let rect = Rectangle { width: 10, height: 5 };

    rect.print_to_string();

    println!("Rectangle's area: {}", rect.get_area());

    println!("Rectangle is square? {}", rect.is_square());
}
