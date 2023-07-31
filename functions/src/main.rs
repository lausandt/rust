fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");

    let number = 37;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut condition = true;
    println!("{condition}");
    condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    let mut n: i32 = 0;
    loop {
            if n > 2 {
                break;
            }
            println!("again!");
            n += 1
        }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    
    for number in (1..11).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let arg: i32 = 10;
    let x  = n_th_fib(arg);
    println!("The {arg}th fibonacci number is {x}")
}

fn plus_one(x: i32) -> i32 {
    x + 1 //return values don't get a semicolon
}

fn n_th_fib(n: i32) -> i32 {
    if n == 0  {
        0
    } else if n == 1 {
        1

    } else {
        n_th_fib(n-1) + n_th_fib(n-2)
    }
    

} 