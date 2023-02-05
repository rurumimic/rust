extern crate blog;
use blog::state::Post;
use blog::types::Post_v2;

fn main() {
    println!("OOP: State Pattern");
    state_pattern();

    println!("with Types");
    with_types();
}

fn state_pattern() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("{}", post.content());
}

fn with_types() {
    let mut post = Post_v2::new();

    post.add_text("I ate a salad for lunch today");
    let post = post.request_review(); // PendingReviewPost

    let post = post.approve(); // Post_v2
    assert_eq!("I ate a salad for lunch today", post.content());

    println!("{}", post.content());
}
