# minigrep

- book: [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

grep: **g**lobally search a **r**egular **e**xpression and **p**rint

## Start

```bash
cargo new minigrep
```

[src/main.rc](src/main.rs):

```rs
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}
```

### args

```bash
cargo run -- needle haystack

    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep needle haystack`

[src/main.rs:5] args = [
    "target/debug/minigrep",
    "needle",
    "haystack",
]
```

## Run

```bash
IGNORE_CASE=1 cargo run -- to poem.txt

    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/minigrep to poem.txt`

Searching for to
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

### stderr

```bash
cargo run > output.txt

    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/minigrep`

Problem parsing arguments: not enough arguments
```

```bash
-rw-r--r-- 0B output.txt
```

## Test

```bash
cargo test

running 2 tests
test tests::case_sensitive ... ok
test tests::case_insensitive ... ok

test result: ok. 2 passed
```

## Codes

- [src/main.rs](src/main.rs)
- [src/lib.rs](src/lib.rs)
