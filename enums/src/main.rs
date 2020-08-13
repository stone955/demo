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

fn main() {
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
}
