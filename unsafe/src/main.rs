use std::slice;

static GEORGE: &str = "and again George usurpes your code!";
static mut COUNTER: u32 = 0; //a mutable global value Having multiple threads access COUNTER would likely result in data races.

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32; //as *const i32 casts num to an immutable raw pointer
    let r2 = &mut num as *mut i32; //we can both create an immutable and mutable value of num, unline safe RUST

    unsafe {
        println!("r1 is: {}", *r1); // the star operator is the dereference operator here
        println!("r2 is: {}", *r2);
    }

    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3); //split_at_mut  function can't be 

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let (x,y) = split_at_mut(r, 3);

    println!("x is {:?}, y is {:?}", x, y);

    //External code used in RUST or RUST code used by another language
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    
    unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    #[no_mangle]
    pub extern "C" fn call_from_c() {
            println!("Just called a Rust function from C!");
    }

    println!("{}",GEORGE);

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    let mut v = Vec::with_capacity(4);

    for i in 0 .. 3 { 

        v.push(i); 

    }
    let n = &v[0] as *const i32;
    v.push(4);
    
    println!("{}", unsafe { *n });
    
}

unsafe fn dangerous() {
    println!("Croc is peckish! This is dangerous")
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}