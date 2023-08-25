Concuurent and/or Parallel Programming

Handling concurrent programming safely and efficiently is one of Rust’s major goals. 

 - Concurrent programming: different parts of a program execute independently.
 - Parallel programming: different parts of a program execute at the same time.

Rust knows both message passing concurrency and shared state concurrency. 

The receiver has two useful methods: recv and try_recv. 
 - recv (short for receive), which will block the main thread’s execution and wait until a value is sent down the channel. Once a value is sent, recv will return it in a Result<T, E>. When the transmitter closes, recv will return an error to signal that no more values will be coming.

 - try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. 
 
 Using try_recv is useful if this thread has other work to do while waiting for messages: you can write a loop that calls try_recv every so often, handles a message if one is available, and otherwise does other work for a little while until checking again.

 The transmitter has the method send. The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. 

 The compiler can prevent easy made mistakes because of this.

 ```
 use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
 ```
 Won't compile the ownership of val in the print has been moved to first the transmitter than to the receiver
 
