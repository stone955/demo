#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    addr: String,
}

#[derive(Debug)]
enum IPAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // C 语言定义方法
    {
        let ip_v4 = IpAddrKind::V4;
        let ip_v6 = IpAddrKind::V6;

        let home = IpAddr {
            kind: ip_v4,
            addr: String::from("127.0.0.1"),
        };

        println!("Home is {:?}", home);

        let loopback = IpAddr {
            kind: ip_v6,
            addr: String::from("::1"),
        };

        println!("Loopback is {:?}", loopback);
    }

    // Rust 推荐的方法
    {
        let ip_v4 = IPAddr::V4(192, 168, 1, 1);
        println!("IPV4 is {:#?}", ip_v4);

        let ip_v6 = IPAddr::V6(String::from("::1"));
        println!("IPV6 is {:#?}", ip_v6);
    }
}
