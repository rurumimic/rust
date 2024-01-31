# State Machine

- github
  - oreilly-japan/conc_ytakano: [chapter 5](https://github.com/oreilly-japan/conc_ytakano/tree/main/chap5/)
  - moseskim/concurrent_programming: [chapter 5](https://github.com/moseskim/concurrent_programming/tree/main/chap5/)

## Library

- [futures-rs](https://github.com/rust-lang/futures-rs)
- [nix](https://github.com/nix-rust/nix)

```bash
cargo add futures
cargo add nix -F event
```

## Run

```bash
cargo run --bin state_machine
cargo run --bin scheduler
cargo run --bin epoll_server
```

### epoll_server

```bash
cargo run --bin epoll_server
```

server:

```bash
Run:
  $ telnet 127.0.0.1 10000

accept: 127.0.0.1:51280
recv: 127.0.0.1:51280, hello
recv: 127.0.0.1:51280, world
close: 127.0.0.1:51280
```

client:

```bash
telnet localhost 10000

Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello
hello
world
world
^]
telnet> quit
Connection closed.
```

