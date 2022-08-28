fn main() {
    // a string literal
    let literal = "Hello, world!";
    println!("{}", literal);

    // String type in heap
    let mut s = String::from("Hello"); // requests the memory it needs
    s.push_str(", world!");
    println!("{}", s);
}
