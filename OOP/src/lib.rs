/*The state pattern implemented in RUST
 The state pattern encapsulates varying behaviour for the same object depending on the state of that object
 This can be a cleaner way for an object to change its behavior at runtime
 there is no need to resort to conditional statements and thus improve maintainability of the software
 One disadvantage of using an enum is every place that checks the value of the enum will need a match expression
 or similar to handle every possible variant.
 This could get more repetitive than this trait object solution.
 However the state pattern is not loose coupling adding another state would require to make changes to the other states */

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), // the start state is the Draft
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, txt: &str) {
        self.content.push_str(txt)

    }

    /*as_ref method on the Option because we want a reference to the value inside the Option 
    rather than ownership of the value*/
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
        
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }

    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }


}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {
    
}

impl State for Draft {
    //when a Draft state makes a request for review the state is changed
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    //when a PendingReview state makes a request for review the state is not changed
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //when a PendingReview state makes an approve the state is changed
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    //the state isn't changed
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //dito
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    //the content is actually returned the other states implement the standard content the empty string
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

}

// a solution that doesn't follow the pattern exacly but encodes the changes into types
pub struct PostType {
    content: String,
}

pub struct DraftPost {
    content: String,
}

impl PostType {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
}

impl PendingReviewPost {
    pub fn approve(self) -> PostType {
        PostType {
            content: self.content,
        }
    }
}