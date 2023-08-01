fn main() {
    let s1 = String::from("George is a rhino!");

    let (mut s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    //let s1 = String::from("George is a rhino!"); won't compile

    let len = calc_len(&s2);  //the &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
                                     //because it does not own it, the value it points to will not be dropped 
                                     //when the reference stops being used

    println!("{}", len);

    change(&mut s2);

    println!("{}", s2);

    println!("{}", first_word(&s2));

    let rhino = "Purple is the only proper colour";

    println!("Rhino says: {}",rhino);

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn calculate_length(s: String) -> (String, usize) { // a function taking ownership and returning ownership
    let length = s.len(); 

    (s, length)
}

fn calc_len(s: &String)-> usize {
    s.len()
}  //Here, s goes out of scope. But because it does not have ownership of what
   // it refers to, it is not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(" Croc: I want a blue heron");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
