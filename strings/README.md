Collections::String

Strings in Rust are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text. The operations on String that every collection type has, such as creating, updating, and reading. 

Ways in which String is different from the other collections, namely how indexing into a String is complicated by the differences between how people and computers interpret String data.

Rust has only one string type in the core language, which is the string slice str, that is usually seen in its borrowed form &str. String slices, are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program’s binary and are therefore string slices.

The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. Both String and string slices are UTF-8 encoded.

This is different from most programming languages, in Java a string is an object of the String class, all strings are. In Python are also instances of a string class. Haskell strings are of a list of char