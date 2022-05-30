enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6,
}

enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32, i32),
}

impl Message {
    fn some_func() {
        println!("Let's get Rusty!")
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddr{
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1")
    };

    Message::some_func()
}

fn route(ip_kind: IpAddrKind) {} 