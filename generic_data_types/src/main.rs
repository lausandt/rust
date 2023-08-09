use generic_data_types::{Tweet, NewsArticle};
use generic_data_types::Summary;

fn main() {
        let list_one = vec![34, 50, 25, 100, 65];
       
        println!("The largest number is {}", largest(&list_one));

        let list_two = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        println!("The largest number is {}", largest(&list_two));

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
    
        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize()); 


}

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
