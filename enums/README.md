Enums allow you to define a type by enumerating its possible variants. 

Rust knows an enum called Option, which expresses that a value can be either something or nothing, it is thus akin to the Maybe monad in Haskell, although Maybe covers its use better, as in there will be maybe a value with this type. 

enum Option<T> {
    None,
    Some(T),
}

Pattern matching in the match expression makes it easy to run different code for different values of an enum. 

The if let construct is another convenient and concise idiom available to handle enums in code.