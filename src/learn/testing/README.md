# Testing

- book: [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

Help:

```bash
cargo test --help
cargo test -- --help
```

## Start

```bash
cargo new adder --lib
     Created library `adder` package

cd adder
```

### Adder

- [adder/src/lib.rs](adder/src/lib.rs)

```rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

- `#[cfg(test)]`: configuration option is `test`
- `#[test]`: helper function

Run test:

```bash
cargo test
```

Result:

```rs
running 2 tests
test tests::it_works ... ok
test tests::another ... FAILED

failures:

---- tests::another stdout ----
thread 'tests::another' panicked at 'Make this test fail', src/lib.rs:11:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::another

test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

#### use super

the `tests` module is an inner module:

```rs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
      // --snip--
    }
}
```

#### left and right

For structs and enums,  
must implement the `PartialEq` and `Debug` traits:

```rs
#[derive(PartialEq, Debug)]
```

test:

```rs
---- tests::it_adds_two stdout ----
thread 'tests::it_adds_two' panicked at 'assertion failed: `(left == right)`
  left: `4`,
 right: `5`', src/lib.rs:29:9
```

#### Custom Failure Message

```rs
#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

test:

```rs
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name) // success
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!") // fail
}
```

```rust
test tests::greeting_contains_name ... FAILED

failures:

---- tests::greeting_contains_name stdout ----
thread 'tests::greeting_contains_name' panicked at 'Greeting did not contain name, value was Hello!', src/lib.rs:75:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::greeting_contains_name
```

#### Result<T, E>

```rs
#[test]
fn it_works_2() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

- enables you to use the question mark operator in the body of tests
- use `assert!(value.is_err())`

### Guessing Game

- guessing game: [test](../guessing_game/README.md#test)
- [src/main.rs](../guessing_game/src/main.rs)

```rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

#### should panic ok

```bash
running 1 test
test tests::greater_than_100 - should panic ... ok
```

#### should panic failed

```rs
// if value < 1 || value > 100 {
if value < 1 { 
  // ...
}
```

```bash
running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
note: test did not panic as expected

failures:
    tests::greater_than_100
```

#### should panic expected ok

```rs
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {  
  // ...
}
```

```bash
running 1 test
test tests::greater_than_100 - should panic ... ok
```

#### should panic expected failed

```rs
#[should_panic(expected = "less than or equal to 999")]
```

```bash
running 1 test
test tests::greater_than_100 - should panic ... FAILED

failures:

---- tests::greater_than_100 stdout ----
thread 'tests::greater_than_100' panicked at 'Guess value must be less than or equal to 100, got 200.', src/main.rs:25:13
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: panic did not contain expected string
      panic message: `"Guess value must be less than or equal to 100, got 200."`,
 expected substring: `"less than or equal to 999"`

failures:
    tests::greater_than_100
```

---

## Controll testing

### threads

default: run in parallel using threads

```bash
cargo test -- --test-threads=1
```

### println!

```rs
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}
```

```bash
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
thread 'tests::this_test_will_fail' panicked at 'assertion failed: `(left == right)`
  left: `5`,
 right: `10`', src/lib.rs:105:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

```bash
cargo test -- --show-output
```

```bash
test tests::this_test_will_pass ... ok
test tests::this_test_will_fail ... FAILED

successes:

---- tests::this_test_will_pass stdout ----
I got the value 4

failures:

---- tests::this_test_will_fail stdout ----
I got the value 8
```

### a Subset of Tests by Name

```bash
cargo test it_works
```

```bash
running 2 tests
test tests::it_works ... ok
test tests::it_works_2 ... ok
```
### Ignoring Some Tests Unless Specifically Requested

```rs
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

```bash
cargo test

running 9 tests
test tests::expensive_test ... ignored
test tests::it_adds_two ... ok
```

```bash
cargo test -- --ignored

running 1 test
test tests::expensive_test ... ok
```

```bash
cargo test -- --include-ignored

running 9 tests
test tests::expensive_test ... ok
test tests::it_adds_two ... ok
```

---

## Test Organization

### Unit test

#### Testing Private Functions

```rs
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

### Integration Tests

#### `tests` Directory

[adder/tests/integration_test.rs](adder/tests/integration_test.rs)

```bash
adder
├── Cargo.lock
├── Cargo.toml
├── src
│   └── lib.rs
└── tests
    └── integration_test.rs
```

```rs
use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

#### 3 sections

```bash
cargo test

running 10 tests
test tests::internal ... ok

test result: ok. 1 passed

     Running tests/integration_test.rs (target/debug/deps/integration_test-49415628897514bb)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed

   Doc-tests adder

running 0 tests

test result: ok. 0 passed
```

#### Integration section

```bash
cargo test --test integration_test

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed
```

#### Submodules in tests

- `tests/common.rs`
- [tests/common/mod.rs](adder/tests/common/mod.rs)

```rs
pub fn setup() {
    // setup code specific to your library's tests would go here
    println!("setup...");
}
```

```bash
cargo test

running 10 tests
test tests::internal ... ok

test result: ok. 1 passed

     Running tests/integration_test.rs (target/debug/deps/integration_test-49415628897514bb)

running 1 test
test it_adds_two ... ok

test result: ok. 1 passed

   Doc-tests adder

running 0 tests

test result: ok. 0 passed
```

- [tests/integration_test.rs](adder/tests/integration_test.rs)

```rs
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

```bash
cargo test --test integration_test -- --show-output

running 1 test
test it_adds_two ... ok

successes:

---- it_adds_two stdout ----
setup...


successes:
    it_adds_two

test result: ok. 1 passed
```
