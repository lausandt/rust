
//The return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y
//<'a> is the lifetime parameter
// longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str wouldn't compile 
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

//the compiler adds life times to &self and announcement, the return type has the same life time as &self 
impl<'a> ImportantExcerpt<'a> {
    pub fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

//function that returns the first word of a string
pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

use std::fmt::Display;

// generic type T, which can be filled in by any type that implements the Display trait as specified by the where clause.
pub fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct Foo<'a> {

    bar: &'a i32
  
  }
  
  
  // Foo changed to &Foo
  struct TestResult {
    /// Student's scores on a test
    scores: Vec<usize>,

    /// A possible value to curve all scores
    curve: Option<usize>
}
impl TestResult {  
    pub fn get_curve(&self) -> &Option<usize> { 
        &self.curve 
    }

    /// If there is a curve, then increments all 
    /// scores by the curve
    pub fn apply_curve(&mut self) {
        if let Some(curve) = self.get_curve() {
            for score in self.scores.clone().iter_mut() {
                *score += *curve;
            }
        }
    }
    
}



  
  