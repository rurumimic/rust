# Lock Free

- Always keep in mind that due to concurrency, the order of operations can be reversed.
- For the correctness of the program, data consistency is more important than the order of operations.
- Lock-free algorithms ensure data consistency but do not guarantee the order of operations.

## Stack

```bash
cargo run
```

```bash
push: 0
push: 1
push: 2
finished push: #0
pop: 2
pop: 0
pop: 6 # before push 6
push: 6
push: 7
push: 8
finished push: #2
pop: 1
pop: 8
finished pop: #3
pop: 7
finished pop: #1
```

- push: 0, 1, 2, 6, 7, 8
- pop: 2, 0, 6, 1, 8, 7

