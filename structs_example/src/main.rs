#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// a rust struct becomes akin to an object in Java or Python when it starts implementing methods
// You can implement the methods in several or one block, the one block should be used in general
impl Rectangle { // method
    fn area(&self) -> u32 { // the use of &self ensures that the method does not take ownership
    self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let middle = Rectangle {
        width: 30,
        height: 50,
    };
    let small = Rectangle {
        width: 10,
        height: 40,
    };
    let large = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {} cm, the circumference is {} cm.",
        rect1.area(), circumference(&rect1)
    );

    //println!("rect1 is {:?}", rect1); // {:?} tells the macro to use the debug mode
    //println!("rect1 is {:#?}", rect1); // {:?} tells the macro to use the pretty print mode
    //dbg ! (&rect1);
    println!("can the middle area contain the small area? {}", middle.can_hold(&small));
    println!("can the middle area contain the large area? {}", middle.can_hold(&large));


}



fn circumference(rectangle: &Rectangle) -> u32 { //& is a reference
    2*rectangle.width + 2*rectangle.height
}