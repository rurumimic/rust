fn main() {
    // a string literal
    let literal = "Hello, world!";
    println!("{}", literal);

    {
        // String type in heap
        let mut s1 = String::from("Hello"); // requests the memory it needs
        s1.push_str(", world!");
        println!("{}", s1);
        let s2 = s1; // move value
        println!("{}", s2);
    } // Rust calls drop automatically at the closing curly bracket.
}
