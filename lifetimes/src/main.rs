
use lifetimes::{ImportantExcerpt,longest, first_word, longest_with_an_announcement};

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // using two different liftimes/scopes
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str()); // moving result out of this scope would mean the compiler would panic
        println!("The longest string is {}", result); // moving the print outside this scope would lead to "abcd"
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{:?}", i.announce_and_return_part("George"));

    println!("{}", first_word("George is a rhino!"));

    println!("{}",longest_with_an_announcement("abc", "defg", "George") );

    let v = vec![5, 4, 3, 2, 1];
    let n = &v[0];
    find_nth(&v, *n-1);
    println!("{}", n);

    let v = vec![5, 4, 3, 2, 1];
    find_nth(&v, 0);
    println!("{}", v[0]);
    
}

fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elems = elems.to_vec();
    elems.sort();
    let t = &elems[n];
    return t.clone();
}
  
