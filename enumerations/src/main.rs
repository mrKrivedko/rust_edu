use std::net::Ipv4Addr;

enum IpAddKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddKind_v2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,  // пустой элемент без ассоциированных данных
    Move {x: i32, y: i32},  // имеет именованные поля, как и структура
    Write(String),
    ChangeColor(u32, u32,u32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

struct IpAddr {
    kind:IpAddKind,
    address: String,
}

fn main() {
    let ipv4 = IpAddKind::V4;
    let ipv6 = IpAddKind::V6;

    let home = IpAddr{
        kind: ipv4,  // IpAddKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr{
        kind: ipv6,  // IpAddKind::V6,
        address: String::from("::1"),
    };
    
    {
        let home = IpAddKind_v2::V4(String::from("0.0.0.0"));
        let loopback = IpAddKind_v2::V6(String::from("0:0:0:0:0:1"));
        println!("home addr: {:#?}, loopback: {:#?}", home, loopback);
    }
    println!("home addr: {}, loopback: {}", home.address, loopback.address);

    {
        let message = Message::Write(String::from("message"));
        message.call();
    }
}

fn route(ip_kind: IpAddKind) {}
