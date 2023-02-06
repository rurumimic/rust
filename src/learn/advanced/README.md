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

---

## Traits

---

## Types

---

## Functions and Closures

---

## Macros
