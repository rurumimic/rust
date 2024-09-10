# Declarative Macros

## Nightly

```bash
rustup toolchain install nightly
```

## Run

### my_vec

```bash
cargo run --bin my_vec

[]
[]
[1]
[1, 2, 3]
```

### my_greeting

```bash
cargo run --bin my_greeting

Heya, Sam!
Hello, Sam!
```

### main.rs

```bash
cargo +nightly run --bin declarative

"The name passed to test is ", "Sam"
note: trace_macro
  --> src/main.rs:11:18
   |
11 |     let _greet = greeting!("Sam", "Heya");
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: expanding `greeting! { "Sam", "Heya" }`
   = note: to `base_greeting_fn("Sam", "Heya")`

note: trace_macro
  --> src/main.rs:12:31
   |
12 |     let _greet_with_default = greeting!("Sam");
   |                               ^^^^^^^^^^^^^^^^
   |
   = note: expanding `greeting! { "Sam" }`
   = note: to `base_greeting_fn("Sam", "Hello")`

note: trace_macro
  --> src/main.rs:13:36
   |
13 |     let _greet_with_default_test = greeting!(test "Sam");
   |                                    ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: expanding `greeting! { test "Sam" }`
   = note: to `{
               log_syntax! ("The name passed to test is ", "Sam"); print
ln!
               ("Returning default greeting"); base_greeting_fn("Sam", "
Hello")
           }`
   = note: expanding `println! { "Returning default greeting" }`
   = note: to `{
               $crate :: io ::
               _print($crate :: format_args_nl! ("Returning default gree
ting"));
           }`

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/declarative`
Returning default greeting
```

### my_calculate

```bash
cargo run --bin my_calculate

1000 -> 2000
```

### my_recursive

```bash
cargo +nightly run --bin my_recursive

error: recursion limit reached while expanding `count!`
  --> src/bin/my_recursive.rs:9:13
   |
9  |             count!($val - 1)
   |             ^^^^^^^^^^^^^^^^
...
16 |     count!(3);
   |     --------- in this macro invocation
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "6"]` attribute to your crate (
`my_recursive`)
   = note: this error originates in the macro `count` (in Nightly builds, run with -Z macro-backtrace for more info)

note: trace_macro
  --> src/bin/my_recursive.rs:16:5
   |
16 |     count!(3);
   |     ^^^^^^^^^
   |
   = note: expanding `count! { 3 }`
   = note: to `if 3 == 1 { 1 } else { count! (3 - 1) }`
   = note: expanding `count! { 3 - 1 }`
   = note: to `if 3 - 1 == 1 { 1 } else { count! (3 - 1 - 1) }`
   = note: expanding `count! { 3 - 1 - 1 }`
   = note: to `if 3 - 1 - 1 == 1 { 1 } else { count! (3 - 1 - 1 - 1) }`

error: could not compile `declarative` (bin "my_recursive") due to 1 previous error
```

### my_account

```bash
cargo run --bin my_account

The poor has: 200
The rich has: 800
How generous # expect Cheapskate
```

## Syntax

all valid:

```rust
(matcher) => (transcriber);
{matcher} => {transcriber};
[matcher] => [transcriber];
```

```rust
my_vec!(1, 2, 3);
my_vec!{1, 2, 3};
my_vec![1, 2, 3];
```
