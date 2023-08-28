use std::ops::Add;
use std::fmt;

mod inner {

    pub trait A {

        fn f(&self) -> usize { 0 }

    }

    pub trait B {

        fn f(&self) -> usize { 1 }

    }

    pub struct P;

    impl A for P {}

    impl B for P {}

}
// this is a newtype akin to Haskells newtype
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

//operator overloading
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your BoBo speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("I love my broomstick!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {

    let w = Wrapper(vec![String::from("George"), String::from("is a rhino!")]);
    println!("w = {}", w);

    //using the overloaded operator
    let new_point = Point { x: 1, y: 0 } + Point { x: 2, y: 3 };
    println!("{:?}", new_point);

    new_point.outline_print();

    let person = Human;
    //Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
    Pilot::fly(&person);
    Wizard::fly(&person);
    //could also be: Human::fly(&person)
    person.fly();
    //fully qualified syntax
    println!("A baby dog is not called a {} it is called a {}", Dog::baby_name(), <Dog as Animal>::baby_name());

    use inner::{P, B};    

    println!("{}", P.f());  
}
