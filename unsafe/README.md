Unsafe programming

 Uses the keyword unsafe and for instance allow you to use raw pointers. As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively. The asterisk isn’t the dereference operator; it’s part of the type name. In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:

 1. Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
 2. Aren’t guaranteed to point to valid memory
 3. Are allowed to be null
 4. Don’t implement any automatic cleanup

By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.

  

