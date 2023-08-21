Rust incorporated several features from functional languages:

 - Pattern matching 
 - Enumeration (abstract data types)
 - Closures
 - Iterators

 Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. This is not completely the same as an actual closure is an instance of a function, a value, whose non-local variables have been bound either to values or to storage locations. In Python:

 `def f(x):
    def g(y):
        return x + y
    return g`  

or 

`def h(x):
    return lambda y: x + y`

a = f(6)

assert a(5) == 11

Both return a closure. Like the above Python functions, for Rust the appeal of a closure is to pass the function as an argument and get a function as a result type. Treating function as first class citizens. 

Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type. 

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: 
 - borrowing immutably
 - borrowing mutably
 - taking ownership

A closure body can do any of the following: 
 1. move a captured value out of the closure
 2. mutate the captured value
 3. neither move nor mutate the value
 4. capture nothing from the environment to begin with.

The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:

- FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
- FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
- Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.



