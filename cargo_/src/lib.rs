//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.



/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = cargo_::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Adds the string "world" to `s`.
///
/// # Example
/// ```
/// let mut s = String::new();
/// cargo_::add_world(&mut s);
/// assert_eq!(s, "Hello world");
/// ```
pub fn add_world(s: &mut String) {

    s.push_str("world");

}


#[test]

fn test_add_world() {

    let mut s = String::new();

    add_world(&mut s);

    assert_eq!(s, "world");

}

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Purple
        
    }
}