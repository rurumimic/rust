# Total Modification Order

## One

```bash
cargo run --bin one
```

```bash
a = 15, b = 15, c = 15, d = 15
```

possibly:

```bash
a = 0, b = 0, c = 5, d = 15
a = 0, b = 5, c = 5, d = 15
a = 0, b = 0, c = 10, d = 15
```

## Two

```bash
cargo run --bin two
```

```bash
a = 15, b = 15, c = 15, d = 15
```

possibly:

```bash
a = 0, b = 0, c = 0, d = 0
a = 5, b = 5, c = 5, d = 5
a = 10, b = 10, c = 10, d = 10
```

