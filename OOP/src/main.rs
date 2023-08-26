use OOP::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("George is a wooly Rhino");
    println!("{}", post.content());

    post.request_review();
    println!("{}", post.content());

    post.approve();
    println!("{}",post.content());
}