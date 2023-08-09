pub trait Summary {  // trait is a keyword
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
    
    fn summarize_default(&self)->String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//This is similar the Java's class A implements B 
impl Summary for NewsArticle {

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
            format!("{}", self.author)
        }
    
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

trait MakeNoise {

    fn make_noise(&self) {
  
      println!("(silence)");
  
    }
  
  }
  
  
  struct Dog {}
  
  struct Cat {}
  
  
  impl MakeNoise for Dog {
  
    fn make_noise(&self) {
  
      println!("bark");
  
    }
  
  }
  
  
  impl MakeNoise for Cat {}

  //Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. 
  // This parameter accepts any type that implements the specified trait. 
  pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
  }

  //implements the trait bound syntax, which
  pub fn notifyTB<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notifyDuo<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
}

pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("Rhino"),
        content: String::from(
            "of course, as you probably already know, people you need to start wearing purple",
        ),
        reply: false,
        retweet: false,
    }
}
