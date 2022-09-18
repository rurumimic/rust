mod pair;
use pair::Pair;

use std::fmt::{Debug, Display};
use traits::{NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // 1 new tweet: horse_ebooks: of course, as you probably already know, people
    // 1 new tweet: (Read more from @horse_ebooks...)

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
    // New article available! (Read more from Iceburgh...)

    notify(&tweet);
    notify_generic(&article);
    // Breaking news! (Read more from @horse_ebooks...)
    // Breaking news! (Read more from Iceburgh...)

    notify_multiple_trait(&tweet);
    notify_multiple_trait_generic(&tweet);
    // Breaking news! Tweet from horse_ebooks

    // Implement with Trait Bounds
    let s = 3.to_string();
    println!("3 is {}", s);

    let p = Pair { x: 1, y: 3 };
    let p = Pair::new(1, 3);
    p.cmp_display();
    // The largest member is y = 3
}

// impl Trai syntax
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_multiple_trait(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item);
}

pub fn notify_multiple_trait_generic<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item);
}

// where clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}

fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    2
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
