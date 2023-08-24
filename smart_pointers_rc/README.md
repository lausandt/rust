Reference counters (Multiple ownership) Rc<T>

The Rust type Rc<T>, which is an abbreviation for reference counting enables multiple ownership of values by variables. The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

Use the Rc<T> type if you want to allocate some data on the heap for multiple parts of the program to read and you can’t determine at compile time which part will finish using the data last. after all if we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

Rc<T> is only for use in single-threaded scenarios!

Using Rc<T> allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of the owners still exist.

Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only. You cannot use mutable references as this might result in race conditions. 

RefCell<T>

Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data.The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

When to choose Box<T>, Rc<T>, or RefCell<T>:

 - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
 - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; 
 - RefCell<T> allows immutable or mutable borrows checked at runtime. Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

Mutating the value inside an immutable value is the interior mutability pattern. 
