# Ownership

- book: [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)

## Start

```bash
cargo new ownership
```

write [main.rs](src/main.rs) and run:

```bash
cargo run
```

## Output

```bash
[Copy and Move]
in Stack:
Hello, world!
x = 5 -> y = 5

in Heap:
s1 = Hello, world!
s2 = Hello, world! <- s1
s2 = Hello, world! -> s3 = Hello, world!

[Ownership and Functions]
hello
5

[Return values]
s1 = yours, s2 = _, s3 = hello

[Return tuple]
The length of 'hello' is 5.
```

