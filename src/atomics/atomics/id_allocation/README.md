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

### Compare and Exchange

```bash
cargo run --bin compare_exchange
```

```bash
a: 0
a: 1
a: 2
a: 3
```

### Compare and Exchange without Overflow

```bash
cargo run --bin compare_exchange_no_overflow
```

```bash
[src/bin/compare_exchange_no_overflow.rs:16:5] allocate_new_id() = 0
[src/bin/compare_exchange_no_overflow.rs:17:5] allocate_new_id() = 1
[src/bin/compare_exchange_no_overflow.rs:18:5] allocate_new_id() = 2
```

### Fetch Update

```bash
cargo run --bin fetch_update
```

```bash
[src/bin/fetch_update.rs:11:5] allocate_new_id() = 0
[src/bin/fetch_update.rs:12:5] allocate_new_id() = 1
[src/bin/fetch_update.rs:13:5] allocate_new_id() = 2
```

### Compare and Exchange with Lazy Initialization

```bash
cargo run --bin compare_exchange_lazy_init
```

```bash
[src/bin/compare_exchange_lazy_init.rs:22:5] get_key() = 42
[src/bin/compare_exchange_lazy_init.rs:23:5] get_key() = 42
[src/bin/compare_exchange_lazy_init.rs:24:5] get_key() = 42
```

### Once with Lazy Initialization

```bash
cargo run --bin once_lazy_init
```

```bash
[src/bin/once_lazy_init.rs:20:5] get_key() = 42
[src/bin/once_lazy_init.rs:21:5] get_key() = 42
[src/bin/once_lazy_init.rs:22:5] get_key() = 42
```

### Once Lock with Lazy Initialization

```bash
cargo run --bin once_lock_lazy_init
```

```bash
[src/bin/once_lock_lazy_init.rs:14:5] get_key() = 42
[src/bin/once_lock_lazy_init.rs:15:5] get_key() = 42
[src/bin/once_lock_lazy_init.rs:16:5] get_key() = 42
```

