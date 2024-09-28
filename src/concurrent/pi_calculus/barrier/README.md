# Barrier

## π-calculus

- wikipedia: [π-calculus](https://en.wikipedia.org/wiki/%CE%A0-calculus)

$$
barrier = c\braket{x}.c\braket{y}.c\braket{z}.(vd)\overline{x}d.\overline{y}.\overline{z}d.0
$$

$$
node = (va)\overline{c}a.a\braket{d}.0
$$

$$
(vc)(barrier | node | node | node )
$$

## Run

```bash
cargo run
```

```bash
node 0: send!
node 1: send!
node 2: send!
barrier: send!
node 0: received!
node 2: received!
node 1: received!
```

