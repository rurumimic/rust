# Ownership

- book: [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

## What is Ownership?

- book: [What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

```bash
cargo new ownership
```

write [main.rs](ownership/src/main.rs) and run:

```bash
cargo run
```

---

## References and Borrowing

- [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

```bash
cargo new references_borrowing
```

write [main.rs](references_borrowing/src/main.rs) and run:

```bash
cargo run
```

### Data Race

Rust prevents this problem by refusing to compile code with data races!

happens when these three behaviors occur:

- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

---

## The Slice Type

- [The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)

```bash
cargo new slice_type
```

write [main.rs](slice_type/src/main.rs) and run:

```bash
cargo run
```
