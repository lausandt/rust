fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let guess: i32 = "42".parse().expect("Not a number");
    println!("{guess}");

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let a = [3; 5];

    println!("{}",a[2]);

    george("George".to_string());
}

fn george(name: String) {
    println!("{name} is a rhino!!!")
}