#![allow(unused)]
fn main() {
    let literal: &str = "I have a static lifetime.";
    let literal: &'static str = "I have a static lifetime.";
}

// fn dangling_references() {
//     // `x` does not live long enough
//     let r;

//     {
//         let x = 5;
//         r = &x;
//         //  ^^ borrowed value does not live long enough
//     } // `x` dropped here while still borrowed

//     println!("r: {}", r);
//     //                - borrow later used here
// }

// fn wont_compile() {
//    let string1 = String::from("abcd");
//    let result;
//
//    {
//        let string2 = String::from("xyz");
//
//        // let result = longest(string1.as_str(), string2);
//        result = longest(string1.as_str(), string2.as_str()); // borrowed value does not live long enough
//    } // `string2` dropped here while still borrowed
//
//    println!("The longest string is {}", result); // borrow later used her
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // error[E0106]: missing lifetime specifier
    // fn longest(x: &str, y: &str) -> &str {
    //               ----     ----     ^ expected named lifetime parameter

    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn ref_in_struct() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
} // valid: novel doesn't go out of scope until after the ImportantExcerpt goes out of scope.

// Lifetime Elision
// fn first_word(s: &str) -> &str {
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn chapter4() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
