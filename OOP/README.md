Rust OOP

Rust does not implement a form of inheritance, but it does provide means to perform OO programming such as.

 - Structs -> Objects
 - Enums -> enumerations
 - pub -> encapsulation 
 - Traits -> Interfaces
 
Rust uses dynamic dispatch with traits, for the compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call. 

**dyn**
dyn is a prefix of a trait object's type.

The dyn keyword is used to highlight that calls to methods on the associated Trait are dynamically dispatched. To use the trait this way, it must be 'object safe'.

Unlike generic parameters or impl Trait, the compiler does not know the concrete type that is being passed. That is, the type has been erased. As such, a dyn Trait reference contains two pointers. One pointer goes to the data (e.g., an instance of a struct). Another pointer goes to a map of method call names to function pointers (known as a virtual method table or vtable).

At run-time, when a method needs to be called on the dyn Trait, the vtable is consulted to get the function pointer and then that function pointer is called.