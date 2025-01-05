# ID Allocation

## Run

### Overflowed

```bash
cargo run --bin overflowed
```

```bash
[src/bin/overflowed.rs:10:5] allocate_new_id() = 0
[src/bin/overflowed.rs:11:5] allocate_new_id() = 1
[src/bin/overflowed.rs:12:5] allocate_new_id() = 2
overflowing the counter... (this might take a while)
overflowed
[src/bin/overflowed.rs:22:5] allocate_new_id() = 2
```

### Panic

```bash
cargo run --bin panic
```

```bash
[src/bin/panic.rs:12:5] allocate_new_id() = 0
overflowing the counter... (this might take a few hours)
overflowed!
[src/bin/panic.rs:28:5] allocate_new_id() = 999
```

### Subtract before panic

```bash
cargo run --bin subtract
```

```bash
[src/bin/subtract.rs:16:9] allocate_new_id() = 0
[src/bin/subtract.rs:16:9] allocate_new_id() = 1
[src/bin/subtract.rs:16:9] allocate_new_id() = 2

[src/bin/subtract.rs:16:9] allocate_new_id() = 999
thread 'main' panicked at src/bin/subtract.rs:9:9:
too many IDs allocated!
```

