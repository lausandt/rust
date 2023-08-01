A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

Rust uses the ampersand (&) as reference.

The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *. 

Rust calls the action of creating a reference borrowing. 

You can make reference mutable by adding mut beforev the ampersand, however mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. 

The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

1. Two or more pointers access the same data at the same time.
2. At least one of the pointers is being used to write to the data.
3. There’s no mechanism being used to synchronize access to the data.

The Rules of References

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

