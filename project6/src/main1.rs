// 6.1 Defining an Enum

use std::option::Option;

#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,                       // 연관된 데이터가 전혀 없다
    Move { x: i32, y: i32 },    // 익명 구조체 포함
    Write(String),              // 하나의 String 포함
    ChangeColor(i32, i32, i32), // 세개의 i32 튜플을 포함
}

impl Message {
    fn call(&self) {}
}
fn main1() {
    let ipFour = IpAddressKind::V4;
    let ipSix = IpAddressKind::V6;

    route(IpAddressKind::V4);
    route(IpAddressKind::V6);

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("우리집"),
    };

    let loopback: IpAddress = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("loopback"),
    };

    println!("Home: {:#?}", home);

    let homeAddr = IpAddr::V4(String::from("127.0.0.1"));
    println!("Home Address: {:#?}", homeAddr);

    let some_number: Option<i32> = Some(5);
    let some_string: Option<&str> = Some("a string");
    let absent_number: Option<i32> = None;

    println!("some_number : {:?}", some_number);
    println!("some_string : {:?}", some_string);
    println!("absent_number : {:?}", absent_number);

    // https://doc.rust-lang.org/std/option/enum.Option.html
    if some_number.is_some() == true {
        let sum = some_number.unwrap() + 5;
        println!("Sum: {}", sum);
    }
}

fn route(ip_type: IpAddressKind) {}
