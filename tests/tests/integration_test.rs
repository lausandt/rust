use tests::add_two;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, add_two(2));
}

#[test]
fn george() {
    assert_eq!("George is a rhino!", common::setup().to_string());
}