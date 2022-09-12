# Cargo Project

- book: [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)

Organize code by splitting it into multiple modules and then multiple files.

- **Packages**: A Cargo feature that lets you build, test, and share crates
- **Crates**: A tree of modules that produces a library or executable
- **Modules** and **use**: Let you control the organization, scope, and privacy of paths
- **Paths**: A way of naming an item, such as a struct, function, or module

## Start

### Crate

- `rustc file.rs`
  - **the smallest amount of code** that the Rust compiler considers at a time.
- can contain modules(may be defined in other files that get compiled with the crate)
- form:
  - **Binary crates**: must have a `main` function
    - `src/main.rs`
    - command-line program, server
  - **Library crates**: don’t have a `main` function
    - `src/lib.rs`
- **crate root**: a source file that the Rust compiler starts from and makes up the root module of your crate
  - `src/main.rs`
  - `src/lib.rs`

### Package

- a bundle of one or more crates that provides a set of functionality
- contains a `Cargo.toml`: describes how to build those crates

### Module

#### binary crate: backyard

- backyard
  - Cargo.lock
  - Cargo.toml
  - src
    - [main.rs](backyard/src/main.rs)
    - [garden.rs](backyard/src/garden.rs)
    - garden
      - [vegetables.rs](backyard/src/garden/vegetables.rs) 

##### src/main.rs

```rs
use crate::garden::vegetables::Asparagus;

pub mod garden; // tells the compiler to include the code it finds in `src/garden.rs`

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
```

##### src/garden.rs

```rs
pub mod vegetables;
```

##### src/garden/vegetables.rs

```rs
#[derive(Debug)]
pub struct Asparagus {}
```

#### library crate: restaurant

```bash
cargo new --lib restaurant
cargo build
```

- restaurant
  - Cargo.lock
  - Cargo.toml
  - src
    - [lib.rs](restaurant/src/lib.rs)

##### module tree

```bash
crate
 ├── front_of_house
 │   ├── **pub** hosting
 │   │   ├── **pub** add_to_waitlist
 │   │   └── seat_at_table
 │   └── serving
 │       ├── take_order
 │       ├── serve_order
 │       └── take_payment
 └── back_of_house
     ├── **pub** struct Breakfast
     │   └── **pub** fn summer
     ├── **pub** enum Appetizer
     ├── fix_incorrect_order
     └── cook_order
```

##### Re-exporting

```rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

call: `restaurant::hosting::add_to_waitlist()`

##### use

```rs
use std::cmp::Ordering;
use std::io;
// or
use std::{cmp::Ordering, io};
```

```rs
use std::io;
use std::io::Write;
// or
use std::io::{self, Write};
```

```rs
use std::collections::*;
```

#### library crate: restaurant_mods

```bash
cargo new --lib restaurant_mods
cargo build
```

- restaurant
  - Cargo.lock
  - Cargo.toml
  - src
    - [lib.rs](restaurant_mods/src/lib.rs)
    - [front_of_house.rs](restaurant_mods/src/front_of_house.rs)
    - front_of_house
      - [hosting.rs](restaurant_mods/src/front_of_house/hosting.rs)
