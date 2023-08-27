

use OOP::{Post,PostType};

fn main() {
    let mut post = Post::new();

    post.add_text("George is a woolly Rhino");
    println!("{}", post.content());

    post.request_review();
    println!("{}", post.content());

    post.reject();
    println!("{}", post.content());

    post.request_review();
    println!("{}", post.content());

    post.approve();
    println!("{}",post.content());

    let mut post_type = PostType::new();

    post_type.add_text("George his type is a woolly rhino!");

    let post_type = post_type.request_review();

    let post_type = post_type.approve();

    println!("{}",post_type.content())
    
}



