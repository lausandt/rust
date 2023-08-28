Pattern matching

Like Haskell Rust has good pattern matching capabilities. Rust can pattern match on some combination of the following:

    Literals
    Destructured arrays, enums, structs, or tuples
    Variables
    Wildcards
    Placeholders

Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. Here are some examples:

 - In the expression if let Some(x) = a_value, then Some(x) is refutable. If the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.
 - In the expression if let &[x, ..] = a_slice, then &[x, ..] is refutable. If the value in the a_slice variable has zero elements, the &[x, ..] pattern will not match.
