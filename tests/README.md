Testing

Tests are Rust functions that verify that the non-test code is functioning in the expected manner. The bodies of test functions typically perform these three actions:

1. Set up any needed data or state.
2. Run the code you want to test.
3. Assert the results are what you expect.

This is about the features Rust provides specifically for writing tests that take these actions, which include the test attribute, a few macros, and the should_panic attribute.

cargo new "filename" --lib creates a test project

panicked at 'assertion failed: `(left == right)` in python would be `(expected,actual)`