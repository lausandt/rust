The vector

The standard library includes a number of very useful data structures called collections. Collections type containers can hold values of multiple types. The data is stored on the heap and not on the stack as is the case for the array and tuple type. For collections subtypes the amount of data does not need to be known at compile time. Which means these types can grow and shrink at run time.


Each collection type has different capabilities and costs. The three most used collection subtypes that are used in Rust are:

    A vector allows you to store a variable number of values next to each other.
    A string is a collection of characters.
    A hash map allows you to associate a value with a particular key. It’s a particular implementation of the more general data structure called a map (in an imperative language).

Vectors can only store values that are the same type. We can use an enum, the variants of an enum are defined under the same enum type. So when we need one type to represent elements of different types, we can define and use an enum!