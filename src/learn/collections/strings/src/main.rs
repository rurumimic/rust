#![allow(unused)]
fn main() {
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();
    // same as
    let _s = String::from("initial contents");

    // Append
    let mut _s = String::from("foo");
    _s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2); // take a string slice
    println!("s1 is {}", s1); // s1 is foobar
    println!("s2 is {}", s2); // s2 is bar

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s); // lol

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 moved here
    println!("s2 is {}", s2); // s2 is world!
    println!("s3 is {}", s3); // s3 is Hello, world!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("Здравствуйте[0..4] is {}", s); // Зд

    for c in "Зд".chars() {
        println!("{}", c); // З, д
    }

    for b in "Зд".bytes() {
        println!("{}", b); // 208, 151, 208, 180
    }
}
