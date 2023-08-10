Lifetimes

this is a continuation of generic data types.

Lifetimes are another kind of generic. Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

 - &i32        -> a reference
 - &'a i32     -> a reference with an explicit lifetime
 - &'a mut i32 -> a mutable reference with an explicit lifetime

 Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions. Once they’re connected, Rust has enough information to allow memory-safe operations and disallow operations that would create dangling pointers or otherwise violate memory safety.

 Lifetime elision: The compiler infers the life time of references to parameters and return types. The compiler uses three rules:
 1. The compiler assigns a different lifetime parameter to each lifetime in each input type.
 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.
 3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.

 Static is a special form of a life time and suggest that the reference can last the entire run time of the program:

 let s: &'static str = "I have a static lifetime.";

 