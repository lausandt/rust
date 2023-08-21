use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue, ShirtColor::Green],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let one = add_one(5);
    println!("the first add one is {}", one);

    let two = |num:i32| -> i32 { num + 1};
    println!("the second add one is an annotated closure {}", two(5));

    let three = |num| { num + 1 };
    println!("the third add one is a closure with optional syntax {}", three(5));

    let four = |num| num + 1;
    println!("the fourth add one is a closure with minimal syntax {}", four(5));

    // the compiler infers the type, but once used you can't change types
    let example_closure = |x| x;

    let s = example_closure(String::from("George is a rhino!"));
    println!("{}",s);

    //
    let n = example_closure(String::from("5"));
    println!("{}",n);

    let f = |_| (); // sometimes called the "toilet closure"

    let s = String::from("Hello");

    println!("{:?}",f(s));

    //immutable borrowing by a closure
    let list = vec![1,2,3];

    println!("Before defining closure: {:?}", list);

    let only_borrows = ||  println!("a closure that borrows an immutable {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    //mutable borrowing by a closure
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    //println!("{:?}", list); this will not compile as the list is borrowed but the closure isn't called

    borrows_mutably(); //the mutable borrow ends here

    println!("After calling closure: {:?}", list);

    //taking ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //move converts any variables captured by reference or mutable reference to variables captured by value
    // move forces the closure to take ownership of list and therefore preventing it from being dropped 
    // if the main thread finishes before this thread.
    thread::spawn(move || println!("From thread: {:?}", list)).join().unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:?}", list);

    list.sort_by_key(|r| r.height);
    println!("{:?}", list);

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:?} the number of sort operations was {}", list, num_sort_operations);

    //It needs to be explicit that the closure captures the lifetime of s_ref otherwise our function won't compile.
    fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a{
        move || s_ref.to_string()
    }

    let s_own = String::from("George is a rhino!");

    let cloner = make_a_cloner(&s_own);

    //drop(s_own);

    println!("{}", cloner());

    //We can remove the <'a> generic so long as we keep an indicator that the returned closure depends on some lifetime, like this:
    fn make_a_cloner_simple(s_ref: &str) -> impl Fn() -> String + '_ {
        move || s_ref.to_string()
    }

    let mut s = String::from("George");

    let mut add_suffix = |s: &mut String| s.push_str(" is a rhino!");

    println!("{s}");

    add_suffix(&mut s); 


}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
    Green,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // || self.most_stocked() is the closure expression 
                                                               // any argument can be passed between the pipes ||  
                                                               // The closure captures an immutable reference to the self Inventory instance
                                                               // and passes it with the code we specify to the unwrap_or_else method
    }

    
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        let mut num_green: i32 = 10;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Green => num_green +=1
            }
        }
        if num_red > num_blue {
            if num_red > num_green {
                ShirtColor::Red
            }
            else {
                ShirtColor::Green
            }
            
        } else {
            if num_blue > num_green {
                ShirtColor::Blue
            }
            else {
                ShirtColor::Green
            }
            
        }
    }

}

fn add_one(num:i32)->i32{
    num + 1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

pub struct Analyzer<F> {

    postprocess: F

}


impl<F:Fn(i32) -> i32> Analyzer<F> {

    fn process(&self, n: i32) -> i32 {3}

    pub fn pipeline(&self, n: i32) -> i32 {

        let n = self.process(n);

        (self.postprocess)(n)

    }

}