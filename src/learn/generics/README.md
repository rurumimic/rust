# Generics

- book: [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)

## Start

### Generic Types

- [generics/src/main.rs](generics/src/main.rs)

#### Restrict type parameter `T`

```rs
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
```

#### struct

```rs
struct Point<T, U> {
    x: T,
    y: U,
}

let both_integer = Point { x: 5, y: 10 };
let both_float = Point { x: 1.0, y: 4.0 };
let integer_and_float = Point { x: 5, y: 4.0 };
```

#### enum

```rs
enum Option<T> {
    Some(T),
    None,
}
```

```rs
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### Performance

**Monomorphization**: process of turning generic code into specific code by filling in the concrete types that are used when compiled

```rs
let integer = Some(5);
let float = Some(5.0);
```

to:

```rs
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

### Trait

- [traits/src/lib.rs](traits/src/lib.rs)
- [traits/src/main.rs](traits/src/main.rs)
- [traits/src/pair.rs](traits/src/pair.rs)

```rs
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
```

#### Trait Bounds

```rs
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_multiple_trait(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item);
}
```

#### Trait Where Clauses

```rs
fn some_function_where<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
```

#### Return type

```rs
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

### Lifetimes

- [lifetimes/src/main.rs](lifetimes/src/main.rs)

#### Dangling References

```rs
fn main() {
    // `x` does not live long enough
    let r;

    {
        let x = 5;
        r = &x;
        //  ^^ borrowed value does not live long enough
    } // `x` dropped here while still borrowed

    println!("r: {}", r);
    //                - borrow later used here
}
```

#### Lifetime Annotation Syntax

```rs
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```

```rs
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
}
```

#### Lifetime Elision

1. the compiler assigns a lifetime parameter to each parameter that’s a reference.
   - `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
2. if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
   - `fn foo<'a>(x: &'a i32) -> &'a i32`
3. if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a *method*, the lifetime of `self` is assigned to all output lifetime parameters.

##### example

```rs
// Good
fn first_word(s: &str) -> &str {}

// Good
fn first_word<'a>(s: &'a str) -> &'a str {}
```

```rs
// Error: 1st rule
fn longest(x: &str, y: &str) -> &str {}

// Error: 2nd, 3rd rule
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
```

#### Lifetime Annotations in Struct Definitions

```rs
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

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

#### Static Lifetime

- the affected reference can live for the entire duration of the program
- all string literals have the `'static` lifetime

```rs
let literal: &str = "I have a static lifetime.";
let literal: &'static str = "I have a static lifetime.";
```

- can create a dangling reference or a mismatch of the available lifetimes

### Generic Type Parameters, Trait Bounds, and Lifetimes Together

- [together/src/main.rs](together/src/main.rs)

```rs
use std::fmt::Display;

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "Today is someone's birthday!",
    );
    println!("The longest string is {}", result);
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Result:

```bash
Announcement! Today is someone's birthday!
The longest string is abcd
```
