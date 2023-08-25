use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    //creating a seperate thread from the main thread with spawn
    let handle = thread::spawn(|| { //spawn takes a closure
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..6 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // join handle ensures that the snd thread if fully executed even if the main thread if already done
    // The two threads continue alternating, but the main thread waits because of the call to handle.join() 
    // and does not end until the spawned thread is finished.
    handle.join().unwrap();

    let v = vec![1, 2, 3];

    //By adding the move keyword before the closure, 
    //we force the closure to take ownership of the values it’s using rather than allowing Rust to infer that it should borrow the values.
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //drop(v); // can't use the v here as it has moved to the closure

    handle.join().unwrap();

    let mut n = 1;

    //the move copies n here as it is mutable, and does not as such take ownership
    let t = thread::spawn(move || {

        n = n + 1;

        thread::spawn(move || {

            n = n + 1;

        })

    });

    n = n + 1;

    t.join().unwrap().join().unwrap();

    println!("{n}");

    //message passing concurrency

    //mpsc stands for multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    //The spawned thread needs to own the transmitter to be able to send messages through the channel
    thread::spawn(move || {
        let val = String::from("George is a rhino!");
        //the transmitter has a send method
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);

}