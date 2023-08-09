Errors

Rust groups errors into two major categories: 
 - Recoverable: For a recoverable error, such as a file not found error, we want to report it to the user so they can retry.
 - Unrecoverable errors: Unrecoverable errors are always symptoms of bugs, for instance trying to access a location beyond the end of an array. 

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. Rust doesn’t have exceptions. Rust has:

 - type Result<T, E> for recoverable errors,
 - the panic! macro that stops execution when the program encounters an unrecoverable error.

Rust requires you to acknowledge the possibility of an error and take some action before your code will compile. This requirement makes your program more robust by ensuring that you’ll discover errors and handle them appropriately before you’ve deployed your code to production!

enum Result<T, E> {
    Ok(T),
    Err(E),
}

A result is either Ok(Type) or Err(error). Result can represent why an operation failed.

The ? operator can be used to propagate errors higher up to calling functions. You can only use the ? operator in a function that returns Result, Option, or another type that implements FromResidual. 

Guidelines for Error Handling

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:

 - The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
 - Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
 - There’s not a good way to encode this information in the types you use. 




