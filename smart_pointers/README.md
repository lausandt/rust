smart pointers

Smart pointers are data structures that act like a pointer but also have additional metadata and capabilities. The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist in other languages as well.

String and Vec<T> are smart pointers, they both own memory and allow you to manipulate that memory. String, for example, stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8.

Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers. The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope. 


 - Box<T> for allocating values on the heap. 
 - Rc<T>, a reference counting type that enables multiple ownership
 - Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time

Box<T> 

The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.

The use of the Box<T> allows for a recursive type in Rust

The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references. When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation. 

a pointer has 8 bytes on a 64-bit architecture. 

Rust does deref coercion when it finds types and trait implementations in three cases:

 - From &T to &U when T: Deref<Target=U>
 - From &mut T to &mut U when T: DerefMut<Target=U>
 - From &mut T to &U when T: Deref<Target=U>

Immutable references will never coerce to mutable references. Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).

The Rust type Rc<T>, which is an abbreviation for reference counting enables multiple ownership of values by variables. The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

Use the Rc<T> type if you want to allocate some data on the heap for multiple parts of the program to read and you can’t determine at compile time which part will finish using the data last. after all if we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

Rc<T> is only for use in single-threaded scenarios!
