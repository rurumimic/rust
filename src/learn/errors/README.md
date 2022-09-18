# Error Handling

- book: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

Rust doesn’t have exceptions.

- `Result<T, E>`: recoverable error
   - like: file not found
   - want to retry
- `panic!`: unrecoverable error
   - like: access a location beyond the end of an array
   - want to immediately stop

## Start

### Panic

- [panic/src/main.rs](panic/src/main.rs)

```rs
panic!("crash and burn");
```

1. print a failure message
2. **unwind**, clean up the stack
3. quit

#### switch to Immediately Aborting

`Cargo.toml`:

```toml
[profile.release]
panic = 'abort'
```

#### Output

```bash
RUST_BACKTRACE=1 cargo run
   Compiling panic v0.1.0 (rust/src/learn/errors/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.84s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: panic::main
             at ./src/main.rs:2:5
   3: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

### Recover


