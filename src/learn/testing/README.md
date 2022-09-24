# Testing

- book: [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

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

```