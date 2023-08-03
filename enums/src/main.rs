fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrEnum::V4(String::from("127.0.0.1"));
         
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("{:?}", localhost);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    let mut absent_number: Option<i32> = None;  // akin to Maybe Int
    absent_number = Some(55);

    println!("{}", some_number.unwrap() + 7) 


}

#[derive(Debug)]
enum IpAddrKind {
    V4,  
    V6,  
}

#[derive(Debug)]
enum IpAddrEnum {
    V4(String), //The name of each enum variant that we define also becomes a function that constructs an instance of the enum.
    V6(String)  ////That is, IpAddr::V6() is a function call that takes a String argument and returns an instance of the IpAddr type
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {                  // it would require for different structs to achieve the same functionality as this struct
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Struct
    Write(String),              // Message::Write -> function call
    ChangeColor(i32, i32, i32), // Message::ChangeColor -> function call 
}

impl Message {
    fn call(&self) {
        // stub
    }
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