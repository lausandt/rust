fn main() {
    //a string slice
    let x = "George is a Rhino!";
    println!("{}",&x);

    //creating a new empty string
    let mut s = String::new();

    // String has the collection methods that 
    s.push('G');
    s.push('e');

    println!("{}", s);

    s.pop();

    println!("{}", s);

    let data = "initial contents";

    let s = data.to_string();

    println!("{}", s);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    println!("{}", s);

    let mut george = String::from("George");

    george.push_str(" is a Rhino");
    
    println!("{}", george);

    let s1 = String::from("Hello, ");
    let s2 = String::from("George!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    println!("{}", s3);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    // the + operator calls the function: fn add(self, s: &str) -> String {}
    // t1 equals self, which isn't referenced in the function add is called four times, every time changing t1 
    let t4 = t1 + "-" + &t2 + "-" + &t3;

    println!("{}",t4);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    //format! is a macro that doesn't take ownership and returns a string, finally it does not repeat + 4 times
    let t5 = format!("{t1}-{t2}-{t3}");

    println!("{}",t5);

    let hello = "Здравствуйте";
    let answer = &hello.chars().nth(0);

    println!("{}", answer.unwrap());

    for c in hello.chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }


}
