fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let value = value_in_cents(Coin::Nickel);
    println!("value in cents {}", value);

    println!("Hello, world!");
    let x = Either::Right(String::from("Hello world"));

    let value = match x {
        Either::Left(n) => n,

        Either::Right(s) => s.len(),
    };

    let opt: Option<String> = Some(String::from("Hello world"));

    // opt became &opt
    match &opt {
        Some(s) => println!("Some: {s}"),
        None => println!("None!"),
    };

    println!("{:?}", opt);

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}
enum IpAddrKind {
    V4,
    V6,
}
enum IpAddr {
    V4(u8, u8, u8, u8),

    V6(String),
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

#[derive(Debug)]

enum Either {
    Left(usize),

    Right(String),
}
