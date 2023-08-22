All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks like this:

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>; //associated type for this trait

    // methods with default implementations elided
}

Rust uses the Option type. Type Option represents an optional value: every Option is either Some and contains a value, or None, and does not. It is the natural way for Rust to handle the iterator pattern, this compared having a hasNext method as in Java or an error as in Python. Option is of coursein use similar to the Maybe monad in Haskell.

