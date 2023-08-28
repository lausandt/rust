use rand::prelude::*;

fn main() {

    println!("{:?}", plus_one(Some(17)) );
    println!("{}", unwrap(Some(17)) );

    let mut rng = rand::thread_rng();
    let throw: i32 = rng.gen_range(2..13);
    
    good_bad(throw);

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    //we print top while there is some value on the stack
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    let (x, y, z) = (1, 2, 3);

    println!("y is {y}");

    let point = (3,5);

    print_coordinates(&point);

    //Creates a consuming iterator, 
    //that is, one that moves each value out of the vector (from start to end). The vector cannot be used after calling this.
    let mut v = vec![(1, 2), (3, 4)].into_iter();
    let mut sum = 0;

    while let Some(t) = v.next() {
        let (_,n) = t;
        sum += n
    }
    println!("{sum}");

    let x = None;
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let msg = Message::Move {x:160, y:255};

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    let x = (0, 1);

    match x {

        (_, y) if y == 0 => println!("A"),

        (0, _) => println!("B"),

        _ => println!("C")

    }

    let a = [(1,2)];
    let _ = a;

    


}

//two simple examples of pattern matching using match
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn unwrap(w:Option<i32>) -> i32 {
    match w {
        None => 0,
        Some(n) => n
    }
}

//pattern matching using multiple possibilities
fn good_bad(throw:i32) {
    match throw {
        9..=12 => println!("{} is a good throw!", throw ),
         
     _ => println!("{} is a bad throw, loser!", throw)
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

struct Point {
    x:i32,
    y:i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}