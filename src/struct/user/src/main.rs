fn main() {
    let mut user1 = User {
        email: String::from("someone@mail.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("another@mail.com");
    println!(
        "user1 ({}, {}, {}, {})",
        user1.email, user1.username, user1.active, user1.sign_in_count
    );

    let user = User2 {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };

    build_user(String::from("one@mail.com"), String::from("username123"));
    build_user2(String::from("one@mail.com"), String::from("username123"));

    // update
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // `username` moved here
    };
    // no longer use `user1` after creating `user2`

    println!(
        "user2 ({}, {}, {}, {})",
        user2.email, user2.username, user2.active, user2.sign_in_count
    );

    // tuple structs
    let black = Color(1, 2, 3);
    let origin = Point(4, 5, 6);
    let Color(first, second, third) = black;
    println!("black {}, {}, {}", first, second, third);
    println!("origin {}, {}, {}", origin.0, origin.1, origin.2);
    println!("{black:?}");
    println!("{:?}", origin);

    // unit-like structs
    // always equal to every instance of any other type
    let subject = AlwaysEqual;
    println!("{:?}", subject);
    println!("{subject:?}");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// error[E0106]: missing lifetime specifier
// struct User2 {
//     active: bool,
//     username: &str,
//               ^ expected named lifetime parameter
//     email: &str,
//            ^ expected named lifetime parameter
//     sign_in_count: u64,
// }

// construct a new instance of the struct as the last expression
// in the function body to implicitly return that new instance.
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Using the Field Init Shorthand
fn build_user2(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
// like `()`
#[derive(Debug)]
struct AlwaysEqual;
