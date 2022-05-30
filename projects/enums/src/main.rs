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

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("State quarter from {:#?}!", state);
            20
        },
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    //...
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddr{
        kind: IpAddrKind::V4(127, 0, 0, 1),
        address: String::from("127.0.0.1")
    };

    Message::some_func();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x = 5;
    let y = None;

    let sum = x + y.unwrap_or(0);

    println!("{}", sum);

    let penny = Coin::Penny;
    value_in_cents(penny);
    value_in_cents(Coin::Quarter((UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    print!("{:#?}", plus_one(five));

    if let Some(5) = five {
        println!("five!")
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        _ => None,
    }
}

fn route(ip_kind: IpAddrKind) {} 