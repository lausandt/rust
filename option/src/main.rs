fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap());
    println!("{}", unwrap(none));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),  
    }

    let dice_roll_tuple: u8 = 9;
    match dice_roll_tuple {
        // The tuple without any values has a special name, unit.
        // This value and its corresponding type are both written () and represent an empty value or an empty return type. 
        // Expressions implicitly return the unit value if they don’t return any other value.
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),  // the underscore is the wildcard like in Haskell a value you might want to pattern match one but not use any further
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(7u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> { // a function that lets you add up optional value
    match x {
        None => None,
        Some(i) => Some(i + 1),  // the i binds to the value contained in Some
    }
}

fn unwrap(x: Option<i32>) -> i32 { // instead of the method unwrap the function unwrap
    match x {
        None => 0,
        Some(i) => i
    }
}

fn add_fancy_hat() { println!("What a fancy hat!")}
fn remove_fancy_hat() {println!("Remove that fancy hat!")}
fn move_player(num_spaces: u8) {}


