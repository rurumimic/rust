fn main() {
    // &: ampersand = reference
    // `&s1` refers to the value of `s1`
    // but does not own it
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // The length of 'hello' is 5.

    // cannot_change(&s1);

    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("{}", s2); // hello, world

    // Mutable References Restriction:
    // cannot borrow `s2` as mutable more than once at a time

    // OK
    // {
    //    let r1 = &mut s2;
    // } // r1 goes out of scope
    // let r2 = &mut s2;

    // ERROR
    // let r1 = &mut s2; // first mutable borrow occurs here
    // let r2 = &mut s2; // second mutable borrow occurs here
    // println!("{}, {}", r1, r2);
    //                    -- first borrow later used here
    // Prevent data races at compile time!

    // ERROR
    // cannot have a mutable reference
    // while we have an immutable one to the same value.
    //
    // let r1 = &s2; // no problem
    //          --- immutable borrow occurs here
    // let r2 = &s2; // no problem
    // let r3 = &mut s2; // BIG PROBLEM
    //          ^^^^^^^ mutable borrow occurs here
    // println!("{}, {}, and {}", r1, r2, r3);
    //                            -- immutable borrow later used here

    // let dangled = dangle();
    let no_dangled = no_dangle();
    println!("{}", no_dangled);
}

// borrowing
// parameter `s` is a reference
fn calculate_length(s: &String) -> usize {
    s.len() // s ptr -> s1 ptr -> "hello"
} // s goes out of scope
  // but it is not dropped
  // `s` doesn’t have ownership

// error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
// fn cannot_change(some_string: &String) {
//                               -------
//                               help: consider changing this to be a mutable reference: `&mut String`
//   some_string.push_str(", world");
//   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//   `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// error[E0106]: missing lifetime specifier
// fn dangle() -> &String {
//                ^ expected named lifetime parameter
//                consider using the `'static` lifetime
// fn dangle() -> &'static String {
//     let s = String::from("hello");
//     &s
// } // s goes out of scope, and is dropped

fn no_dangle() -> String {
    let s = String::from("hello");
    s // Ownership is moved out
} // and nothing is deallocated.
