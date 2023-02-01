# Concurrency

- book: [Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)

Lower-level languages are expected to provide the solution with the best performance in any given situation and have fewer abstractions over the hardware. Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.

## Threads

- book: [Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)

Rust standard library uses a 1:1 model of thread implementation.

### spawn

[threads/src/main.rs](threads/src/main.rs)

```rs
use std::thread;
use std::time::Duration;

let handle = thread::spawn(|| {
  // spawned thread
});

handle.join().unwrap();
```

output:

```bash
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```

### move values

```rs
let v = vec![1, 2, 3];

let handle = thread::spawn(move || {
    println!("Here's a vector: {:?}", v);
});

// drop(v); // use of moved value: `v`
```

---

## Message Passing

- book: [Using Message Passing to Transfer Data Between Threads](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

### channel

- multiple producer single consumer
  - transmitter
  - receiver: `recv()/try_recv() -> Result<T, E>`
    - `recv`: block
    - `try_recv`: non block

```rs
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
  let val = String::from("hi");
  tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```

### ownership

1. `send`: take ownership of its paramter
1. receiver tkae ownership of it

```rs
let tx1 = tx.clone();
tx1.send(val).unwrap();
```

output:

```bash
Got: hi
Got: more
Got: messages
Got: from
Got: for
Got: the
Got: you
Got: thread
```

---

 Shared-State

- book: [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)

---

## Sync and Send Traits

- book: [Extensible Concurrency with the Sync and Send Traits](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)


