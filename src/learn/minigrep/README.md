# minigrep

- book: [An I/O Project: Building a Command Line Program](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

grep: **g**lobally search a **r**egular **e**xpression and **p**rint

## Start

```bash
cargo new minigrep
```

[src/main.rs](src/main.rs):

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

---

## Improving

[Improving I/O Project - minigrep](/src/learn/functional/README.md#improving-io-project---minigrep)

### main.rs

before:

```rs
let args: Vec<String> = env::args().collect();

let config = Config::build(&args).unwrap_or_else(|err| { ... });
```

→ after:

```rs
let config = Config::build(env::args()).unwrap_or_else(|err| { ... });
```

### lib.rs

- Function std::env::[args](https://doc.rust-lang.org/stable/std/env/fn.args.html)
- Struct std::env::[Args](https://doc.rust-lang.org/stable/std/env/struct.Args.html)

```rs
pub fn args() -> Args

pub struct Args { /* private fields */ }
```

#### build

before:

```rs
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        // ...
    }
```

→ after:

```rs
impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next(); // name of the program

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        // ...
    }
```

#### search

before:

```rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

→ after:

```rs
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .map(|line| line.to_lowercase())
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}
```

### Rerun

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
cargo test
```
