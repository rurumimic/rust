fn main() {
    println!("[Copy and Move]");
    copy_move();

    println!("\n[Ownership and Functions]");
    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function...
                        // Error: println!("{}", s);

    let x = 5;
    makes_copy(x);
    // Ok: println!("{}", x);

    println!("\n[Return values]");
    return_values();

    println!("\n[Return tuple]");
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
} // x = 5: goes out of scope and is dropped
  // s = String"hello": nothing happens.

fn copy_move() {
    println!("in Stack:");

    // a string literal
    let literal = "Hello, world!";
    println!("{}", literal);

    // Copy a value in stack
    let x = 5;
    let y = x;
    println!("x = {} -> y = {}", x, y);

    println!("\nin Heap:");
    {
        // String type in heap
        let mut s1 = String::from("Hello"); // requests the memory it needs
        s1.push_str(", world!");
        println!("s1 = {}", s1);

        // Move value
        let s2 = s1;
        println!("s2 = {} <- s1", s2);
        // Error: println!("{}", s1);

        // Clone
        let s3 = s2.clone();
        println!("s2 = {} -> s3 = {}", s2, s3);
    } // Rust calls drop automatically at the closing curly bracket.
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string: `drop` is called. backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer: nothing happens

fn return_values() {
    let s1 = gives_ownership(); // s1 <- fn
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s3 <- fn <- s2

    println!("s1 = {}, s2 = _, s3 = {}", s1, s3);
} // s3 "hello": drop
  // s2: nothing happens
  // s1 "yours": drop

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // give value to s1
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
