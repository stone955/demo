use std::collections::HashMap;

fn main() {
    let mut marks = HashMap::new();

    marks.insert("languages", vec!["Java", "Golang", "Rust", "Javascript"]);
    marks.insert(
        "frameworks",
        vec!["Mybatis", "Spring", "Spring boot", "Spring cloud"],
    );
    marks.insert(
        "apps",
        vec!["Wechat", "QQ TIM", "Zhibo8", "Taobao", "Maimai"],
    );
    marks.insert("devtools", vec!["VSCode", "SQLyog", "WPS", "Typora"]);

    // Loop through HashMap
    for (key, value) in &marks {
        println!("Key {}, Value {:?}", key, value);
    }

    match marks.get("languages") {
        Some(mark) => {
            println!("Development languages {:?}", mark);
        }
        None => {
            println!("Not found languages");
        }
    }
}
