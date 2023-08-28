advanced traits 

Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```
Item is the associated type here and can be used further in the definition -> Option<Self::Item>

Associated types allow us to implement one iterator for a collection, but prevent us from implementing multiple iterators like a generic version would do.

```
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```
when we would implement that trait in code we would have to give a type annotation for every call to next. \

Associated types also become part of the trait’s contract: implementors of the trait must provide a type to stand in for the associated type placeholder. Associated types often have a name that describes how the type will be used, and documenting the associated type in the API documentation is good practice.

Fully qualified syntax:

<Type as Trait>::function(receiver_if_method, next_arg, ...);
