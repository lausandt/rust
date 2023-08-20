Rust incorporated several features from functional languages:

 - Pattern matching 
 - Enumeration (abstract data types)
 - Closures
 - Iterators

 Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. This is not completely the same as an actual closure is an instance of a function, a value, whose non-local variables have been bound either to values or to storage locations. In Python:

 def f(x):
    def g(y):
        return x + y
    return g  

or 

def h(x):
    return lambda y: x + y 

a = f(6)

assert a(5) == 11

Both return a closure. Like the above Python functions, for Rust the appeal of a closure is to pass the function as an argument and get a function as a result type. Treating function as first class citizens. 


