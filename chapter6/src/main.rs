#![allow(unused)]

mod matching;

fn main() {
    let e = IpAddrKind::V6;
    match e {
        IpAddrKind::V4 => {
            println!("Its v4");
        },
        IpAddrKind::V6 => {
            println!("Its V6")
        },
    }

    let home = IpAddrV1{ 
        kind: IpAddrKind::V4, 
        address: String::from("127.0.0.1") 
    };

    let loopback = IpAddrV1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", loopback);
    println!("{:?}", home);


    let home = IpAddrV2::V4(String::from("127.0.0.01"));
    let loopback = IpAddrV2::V6(String::from("::1"));
    println!("{:?}", home);
    println!("{:?}", loopback);

    let home = IpAddrV3::V4(127, 0, 0, 1);
    let loopback = IpAddrV3::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback);

    let localhost = std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
    let l = std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 1));

    println!("{:}", localhost);
    println!("{:}", l);

    let message = Message::Move { x: 3, y: 3 };
    let color = Message::ChangeColor(32, 43, 22);
    println!("{:?}", message);
    message.send();
    color.send();

    let quarter = matching::Coin::Quarter(matching::UsState::Pennsylvania);
    let penny = matching::Coin::Penny;

    println!("The quarter is worth {} cent(s)", matching::value_in_cents(&quarter));
    println!("The penny is worth {} cent(s)", matching::value_in_cents(&penny));

    let five = Some(5);
    let six = matching::plus_one(five);
    let none = matching::plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    let dice_roll = 9;
    match dice_roll {
        3 => println!("You get a fancy hat"),
        7 => println!("Remove the fancy hat"),
        other => println!("Move {other} spaces")
    }

    match dice_roll {
        3 => println!("You get a fancy hat"),
        7 => println!("Remove the fancy hat"),
        _ => println!("Reroll!")
    }

    match dice_roll {
        3 => println!("You get a fancy hat"),
        7 => println!("Remove the fancy hat"),
        _ => ()
    }

    let config_max = Some(3);
    if let Some(max) = config_max {
        println!("max is {max}");
    }
    
    let mut count = 0;
    let coin = matching::Coin::Nickel;

    if let matching::Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("{count}");

}


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}


#[derive(Debug)]
struct IpAddrV1 {
    kind: IpAddrKind,
    address: String
}


#[derive(Debug)]
enum IpAddrV2 {
    V4(String),
    V6(String)
}


#[derive(Debug)]
enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String)
}


#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn send(&self) {
        match self {
            Message::Quit => println!("Sending quite message"),
            Message::Move { x, y } => println!("Moving to {} {}", x, y),
            Message::Write(s) => println!("Writing '{s}'"),
            Message::ChangeColor(r, g, b) => println!("Chaning color to {},{},{}", r, g, b),
        }
    }
}

