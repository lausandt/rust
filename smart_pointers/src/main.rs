use crate::List::{Cons, Nil};
use crate::RcList::{Cons as OtherCons, Nil as OtherNil};
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    //stores the integer 5 on the stack
    let a = 7;
    //stores the integer 5 as a box on the heap
    let b = Box::new(5);
    println!("{} is on the stack {} is on the heap", a, b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}",list);

    let mut n = 1;

    let b = Box::new(&mut n);

    **b += 1;

    println!("{}", n);

    let x = 5;
    let y = &x;

    // The variable x holds an i32 value 5. 
    // We set y equal to a reference to x. 
    // We can assert that x is equal to 5. 
    // However, if we want to make an assertion about the value in y, we have to use *y
    // to follow the reference to the value it’s pointing to (hence dereference) 
    // so the compiler can compare the actual value. 
    // Once we dereference y, we have access to the integer value y is pointing to that we can compare with 5
    assert_eq!(5, x);
    assert_eq!(5, *y); 

    let x = 5;
    let y = Box::new(x); //here we use a copy of x (=5) to create a new value on the heap

    assert_eq!(5, x);
    assert_eq!(5, *y); //Box<T> is a pointer we need to dereference that too

    let x = 5;
    let y = Box::new(&x); //here we use a reference of x (=5) to create a new value on the heap

    assert_eq!(5, x);
    assert_eq!(5, **y); //Now we need to dereference the box pointer which is another pointer and we need to dereference that too

    let m = MyBox::new(String::from("George"));
    hello(&m); //Rust turns &MyBox<String> into &String by calling deref, and does deref &String to &str recursively 

    let n = AccessLogger(-1);

    let x = *n + 1;
  
    let n2 = n;
  
    println!("{} {}", x, *n);

    let c = CustomSmartPointer {
        data: String::from("George"),
    };

    let d = CustomSmartPointer {
        data: String::from("is a rhino!"),
    };

    drop(c);
    println!("CustomSmartPointers created.");

    let e = Example(0);

    drop(e);

    let mut s = String::new();

    { s };

    let a = Rc::new(OtherCons(5, Rc::new(OtherCons(10, Rc::new(OtherNil)))));
    let b = OtherCons(3, Rc::clone(&a)); //Rc::clone increments the reference count instead of making a deep copy
    let c = OtherCons(4, Rc::clone(&a)); //When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.

    println!("list b = {:?}, list c = {:?}", b, c);

}

#[derive(Debug)]
enum List {
    Cons(i32,Box<List>), //Box<T> is a pointer, with a fixed size therefore Rust can use unlimited recursion 
    Nil,
}

/// custom version of Box<T>
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

#[derive(Clone, Copy)]

struct AccessLogger(i32);


impl Deref for AccessLogger {

    type Target = i32;

    fn deref(&self) -> &Self::Target {

        println!("deref");

        &self.0

    }

}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

struct Example(i32);

impl Drop for Example {

    fn drop(&mut self) {

        self.0 += 1;

        println!("drop {}", self.0);

        

    }

}

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}