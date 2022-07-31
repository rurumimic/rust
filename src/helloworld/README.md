# Hello World

- book
  - [Hello, World!](https://doc.rust-lang.org/book/ch01-02-hello-world.html)
  - [Hello, Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html)

## Cargo

### Create a new package

```bash
cargo new helloworld
     Created binary (application) `helloworld` package
```

```bash
helloworld/
├── Cargo.toml
└── src
    └── main.rs
```

### Cargo.toml

```toml
[package]
name = "helloworld"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### main.rs

```rs
fn main() {
    println!("Hello, world!");
}
```

---

## Build and Run

### Compile

```bash
cargo build
   Compiling helloworld v0.1.0 (…/src/helloworld)
    Finished dev [unoptimized + debuginfo] target(s) in 1.48s
```

#### Output

```bash
helloworld
├── Cargo.lock
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs
└── target/
    ├── CACHEDIR.TAG
    └── debug/
        ├── build/
        ├── deps/
        ├── examples/
        ├── helloworld*
        ├── helloworld.d
        └── incremental/
```

### Run

```bash
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/helloworld`
Hello, world!
```

