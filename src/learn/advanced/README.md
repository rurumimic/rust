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

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

---

## Types

---

## Functions and Closures

---

## Macros
