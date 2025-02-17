# Locks

- github: [atomic-wait](https://github.com/m-ou-se/atomic-wait)
- github: [parking_lot](https://github.com/Amanieu/parking_lot)

## Mutex

```bash
cargo run --example mutex_v1

Bench 1: locked 5000000 times in 573.964275ms
Bench 2: locked 20000000 times in 2.96610396s
```

```bash
cargo run --example mutex_v2

Bench 1: locked 5000000 times in 131.580433ms
Bench 2: locked 20000000 times in 2.081323213s
```

```bash
cargo run --example mutex_v3

Bench 1: locked 5000000 times in 133.842266ms
Bench 2: locked 20000000 times in 2.416116844s
```

## Condvar

```bash
cargo test condvar -- --nocapture
```

## RwLock

```bash
cargo test rwlock -- --nocapture
```

