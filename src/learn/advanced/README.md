# Advanced

- book: [Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html)

## Unsafe

- book: [Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

- Rust has a second language hidden inside it
- *Unsafe Rust* doesn’t enforce these memory safety guarantees

### Unsafe Superpowers

- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of unions

### Dereferencing a Raw Pointer

- raw pointer
  - `*const T` immutable: pointer can't be directly assined to after being dereferenced
  - `*mut T` mutable

raw pointers:

- are allowed to ignore the borrowing rules
- aren't guaranteed to point to valid memory
- are allowed to be null
- don't implement any automatic cleanup

```rs
let address = 0x012345usize;
let r = address as *const i32;
```

[unsafe_rust/src/main.rs](unsafe_rust/src/main.rs)

```rs
let mut num = 5;

let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
  println!("r1 is: {}", *r1);
  println!("r2 is: {}", *r2);
}
```

createing a pointer does no harm.

when to use a raw pointer:

- interface with C code
- building up safe abstractions
  - the borrow checker doesn't understand

### Calling an Unsafe Function or Method

```rs
unsafe fn dangerous() {}

unsafe {
  dangerous();
}
```

### Creating a Safe Abstraction over Unsafe Code

```rs
let mut v = vec![1, 2, 3, 4, 5, 6];

let r = &mut v[..];

let (a, b) = r.split_at_mut(3);
// or
let (a, b) = split_at_mut(r, 3);

assert_eq!(a, &mut [1, 2, 3]);
assert_eq!(b, &mut [4, 5, 6]);
```

```rs
use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

#### undefined behavior

```bash
[1]    54476 segmentation fault (core dumped)  cargo run
```

```rs
use std::slice;

let address = 0x01234usize;
let r = address as *mut i32;

let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
```

### Using extern Functions to Call External Code

```rs
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

```rs
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
```

### Accessing or Modifying a Mutable Static Variable

- In Rust, *global variables* are called *static* variables.
- If two threads are accessing the same mutable global variable, it can cause a data race.

```rs
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

```rs
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}
```

### Implementing an Unsafe Trait

```rs
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}
```

### Accessing Fields of a Union

- Unions are primarily used to interface with unions in C code.
- Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.

### When to Use Unsafe Code

- the compiler can’t help uphold memory safety
- When you have a reason to use unsafe code, you can do so
- explicit unsafe annotation makes it easier

---

## Traits

- book: [Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)

### Associated Types

```rs
pub trait Iterator {
    type Item; // placeholder

    fn next(&mut self) -> Option<Self::Item>;
}
```

implement trait: don’t need to annotate types because we can’t implement a trait on a type multiple times.

```rs
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
```

generics: must annotate the types in each implementation.

```rs
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

### Default Generic Type Parameters and Operator Overloading

```rs
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```

```rs
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

ref: [Newtype Pattern for Type Safety and Abstraction](#newtype-pattern-for-type-safety-and-abstraction)

### Fully Qualified Syntax

```rs
<Type as Trait>::function(receiver_if_method, next_arg, ...);
```

example:

```rs
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

```rs
fn main() {
    let person = Human;
    Pilot::fly(&person); // This is your captain speaking.
    Wizard::fly(&person); // Up!
    Human::fly(&person); // *waving arms furiously*
    // or
    // person.fly(); // *waving arms furiously*

    // A baby dog is called a Spot
    println!("A baby dog is called a {}", Dog::baby_name());
    // A baby dog is called a puppy
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

}
```

### Supertraits

```rs
use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

```rs
**********
*        *
* (1, 3) *
*        *
**********
```

```rs
struct Point {
    x: i32,
    y: i32,
}

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

### Newtype Pattern

Newtype is a term that originates from the Haskell programming language.

```rs
use std::fmt;

struct Wrapper(Vec<String>); // tuple struct

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", ")) // tuple.0
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

- it doesn’t have the methods of the value it’s holding
- have to implement all the methods of `Vec<T>` directly on `Wrapper`
- implementing the `Deref` trait on the `Wrapper` to return the inner type would be a solution

---

## Types

- book: [Advanced Types](https://doc.rust-lang.org/book/ch19-04-advanced-types.html)

### Newtype Pattern for Type Safety and Abstraction

ref: [Millimeters and Meters](#default-generic-type-parameters-and-operator-overloading)

- statically enforcing that values are never confused and indicating the units of a value
- abstract away some implementation details of a type
  - new type can expose a public API that is different from the API of the private inner type
- hide internal implementation
  - OOP: [Encapsulation that Hides Implementation Details](../oop/README.md#encapsulation)

### Type Synonyms with Type Aliases

- type alias
  - give an existing type another name
  - to reduce repetition
- Choosing a meaningful name for a type alias can help communicate your intent as well.

```rs
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

```rs
type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type() -> Thunk {
    // --snip--
}
```

```rs
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}
```

### Never Type that Never Returns

- never type
  - empty type has no values

diverging functions:

```rs
fn bar() -> ! {
    // --snip--
}
```

`continue` has a `!` value:

```rs
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

`panic!` has the type `!`:

```rs
impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
```

`loop` has the type `!`:

```rs
print!("forever ");

loop {
    print!("and ever ");
}
```

However, this wouldn’t be true if we included a `break`.

### Dynamically Sized Types and the `Sized` Trait

let us write code using values whose size we can know only at runtime:

- DST
- Dynamically Sized Types
- Unsized Types

```rs
let s1: str = "Hello there!";
let s2: str = "How's it going?";
```

```rs
fn generic<T>(t: T) {
    // --snip--
}
// same as:
fn generic<T: Sized>(t: T) {
    // --snip--
}
// special syntax to relax this restriction:
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
```

---

## Functions and Closures

- book: [Advanced Functions and Closures](https://doc.rust-lang.org/book/ch19-05-advanced-functions-and-closures.html)

### Function Pointers

- Function pointers implement all three of the closure traits
  - `Fn`, `FnMut`, and `FnOnce`
- only accept `fn` and not closures: interfacing with external code that doesn’t have closures
  - C langauge

```rs
fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    apply_twice(add_one, 3); // 5 = (3 + 1) + 1
}
```

closure vs function pointer:

```rs
let list_of_numbers = vec![1, 2, 3];

let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(|i| i.to_string()).collect();

let list_of_strings: Vec<String> =
    list_of_numbers.iter().map(ToString::to_string).collect();
```

enum function pointer:

```rs
 enum Status {
    Value(u32),
    Stop,
}

let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
```

### Returning Closures

- Closures are represented by traits
  - which means you can’t return closures directly
  - don’t have a concrete type that is returnable

```rs
// Err
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}

// OK
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```

---

## Macros

- book: [Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- [The Little Book of Rust Macros](https://danielkeep.github.io/tlborm/book/index.html)

### Macros vs Functions

- function signature
  - must declare the number and type of parameters
  - can define anywhere and call anywhere
- macro
  - can take a variable number of parameters
  - expanded before the compiler interprets the meaning of the code
  - must define macros or bring them into scope before you call them in a file

write Rust code that writes Rust code

### Declarative Macros: with `macro_rules!` for General Metaprogramming

```rs
let v: Vec<u32> = vec![1, 2, 3];
```

slightly simplified definition of the `vec!` macro:

```rs
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

If the pattern matches, the associated block of code will be emitted.

### Procedural Macros: for Generating Code from Attributes

```rs
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```

### How to Write a Custom `derive` Macro

- [Cargo.toml](macro/Cargo.toml)
- hello
  - [main.rs](macro/hello/src/main.rs)
- hello_macro
  - [lib.rs](macro/hello_macro/src/lib.rs)
  - hello_macro_derive
    - [Cargo.toml](macro/hello_macro/hello_macro_derive/Cargo.toml)
    - [lib.rs](macro/hello_macro/hello_macro_derive/src/lib.rs)

```bash
macro/
├── Cargo.toml
├── hello/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── hello_macro/
    ├── Cargo.toml
    ├── hello_macro_derive/
    │   ├── Cargo.toml
    │   └── src/
    │       └── lib.rs
    └── src/
        └── lib.rs
```

#### hello_macro_derive/Cargo.toml

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

- `syn`: parses Rust code from a string into a data structure that we can perform operations on
- `quote`: turns `syn` data structures back into Rust code

#### Run Hello

```bash
cargo run

Hello, Macro! My name is Pancakes!
```

1. `hello/src/main.rs` → `#[derive(HelloMacro)] struct Pancakes`
2. `hello_macro_derive/src/lib.rs` → `#[proc_macro_derive(HelloMacro)] pub fn hello_macro_derive`
   1. `Pancakes`: `parse(input)` → `ast: DeriveInput` → `&ast.ident`
   2. `quote! { Rust Code }` → `into()` → `TokenStream`
3. `hello/src/main.rs` → `Pancakes::hello_macro()` → `trait HelloMacro::hello_macro()` → `println!("Hello... Pancakes!")`

### Attribute-like macros

- allow to create new attributes
- `derive`: only structs and enums

```rs
#[route(GET, "/")]
fn index() {}
```

```rs
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}
```

- `attr`: `GET, "/"`
- `item`: `fn index() {}`

### Function-like macros

- take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do

```rs
let sql = sql!(SELECT * FROM posts WHERE id=1);
```

```rs
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}
```
