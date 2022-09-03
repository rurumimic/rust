fn main() {
    let mut s1 = String::from("Hello, world!");
    let word1 = first_word(&s1); // 6
    s1.clear(); // s1 = ""
    println!("{} != {}", s1.len(), word1); // 0 != 6

    // let mut s2 = String::from("Hello, world!");
    // let word2 = first_word2(&s2);
    //                         --- immutable borrow occurs here
    // s2.clear(); // error!
    // ^^^^^^^^^^ mutable borrow occurs here
    // println!("{}, {}", s2, word2);
    //                        ----- immutable borrow later used here

    // String Literals Are Slices
    // `&str` is an immutable reference
    let s = "Hello, world!";
    println!("{}", s);

    let my_string = String::from("hello world");
    let word = first_word3(&my_string[0..6]); // hello
    let word = first_word3(&my_string[..]); // hello
    let word = first_word3(&my_string); // hello

    let my_string_literal = "hello world";
    let word = first_word3(&my_string_literal[0..6]); // hello
    let word = first_word3(&my_string_literal[..]); // hello
    let word = first_word3(my_string_literal); // hello

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // &[i32]
                          // storing a ref to the first element and a length
    assert_eq!(slice, &[2, 3]); // [2, 3] != [2, 3]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // String -> [bytes]

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes(); // String -> [bytes]

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes(); // String -> [bytes]

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
