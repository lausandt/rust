use generic_data_types::{Tweet, NewsArticle, Summary, Pair, notify, notifyTB, notifyDuo, returns_summarizable, displayable};


fn main() {
        let list_one = vec![34, 50, 25, 100, 65];
       
        println!("The largest number is {}", largest(&list_one));

        let list_two = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        println!("The largest number is {}", largest(&list_two));

        let tweet = Tweet {
            username: String::from("Croc"),
            content: String::from(
                "I am peckish! I need a snacky!",
            ),
            reply: false,
            retweet: false,
        };
    
        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("I am a wooly rhino!"),
            location: String::from("Amsterdam, The Netherlands"),
            author: String::from("George"),
            content: String::from(
                "George declares that he is a wooly rhino and not as some content a wooly heron."
            ),
        };

        let article2 = NewsArticle {
            headline: String::from("Croc is peckish!"),
            location: String::from("Amsterdam, The Netherlands"),
            author: String::from("Croc"),
            content: String::from(
                "I am peckish! I need a snacky!, Preferably a Cote Du Boeuf otherwise a blue heron will do fine",
            ),
        };

        println!("New article available! {}", article.summarize()); 

        println!("{:?}",notify(&tweet));

        println!("{:?}",notifyTB(&article));

        println!("{:?}",notifyDuo(&article, &article2)); // cannot mix concrete types, unlike Haskell.

        println!("{:?}",returns_summarizable().summarize());

        println!("{}", returns_summarizable().summarize());

        let pair = Pair {
            x:5,
            y:7,
          };

        println!("{:?}",pair.cmp_display());

        let p = pair.new(16,8);

        println!("{:?}",p.cmp_display());

        let s = String::from("hello");

        let mut s2 = displayable(s);
      
        //s2.push_str(" world");
      
        println!("{s2}");


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
