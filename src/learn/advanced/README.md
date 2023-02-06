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

---

## Types

---

## Functions and Closures

---

## Macros
