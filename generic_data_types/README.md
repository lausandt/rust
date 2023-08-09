Generic Data Types.

Generics is a form of parametric polymorpy like the definition of first in Haskell: fst :: (a,b) -> a

Rust generics uses type parameters (like Java), which make it possible to design classes & methods that defer the specification of one or more types until the method/class is declared and instantiated by client code. Generic classes and methods combine reusability, type safety, and efficiency in a way that their non-generic counterparts cannot. Generics are most frequently used with collections and the methods that operate on them.

DRY:
 - Identify duplicate code.
 - Extract the duplicate code into the body of the function and specify the inputs and return values of that code in the function signature.
 - Update the two instances of duplicated code to call the function instead.

Dry with generics implement the same code for methods/classes that accept argument and return values of different types. 

Trait:
A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

Traits are similar to a Java interfaces, although with some differences.

    You can implement an external trait for a local type

    You can implement a local trait for an external type

    You can implement a local trait for a local type
