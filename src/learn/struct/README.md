# Structure

- book: [struct](https://doc.rust-lang.org/book/ch05-00-structs.html)

## Start

### User

```bash
cargo new user
```

- [user/src/main.rs](user/src/main.rs)

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

### Rectangles

```bash
cargo new rectangles
```

- [rectangles/src/main.rs](rectangles/src/main.rs)

#### Implementation

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // like a constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", rect1.area());
}
```

#### Debug

- std::[dbg](https://doc.rust-lang.org/std/macro.dbg.html)

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

```bash
cargo run

[src/main.rs:23] 30 * scale = 60
[src/main.rs:26] &rect2 = Rectangle {
    width: 60,
    height: 50,
}
```

