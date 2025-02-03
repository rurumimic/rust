# Atomics

## Book

"Rust Atomics and Locks by Mara Bos (O’Reilly). Copyright 2023 Mara Bos, 978-1-098-11944-7."

- book: [marbos.nl/atomics](https://marabos.nl/atomics/)
- repo: [rust-atomics-and-locks](https://github.com/m-ou-se/rust-atomics-and-locks)

## Contents

### basics

- [threads](basics/threads/README.md): scope, builder, leak, ownership
- [reference counting](basics/reference_counting/README.md): rc, arc, shadowing

### atomics

- basics
  - [stop flag](atomics/stop_flag/README.md): atomic bool with thread
  - [progress reporting](atomics/progress_reporting/README.md): ordering::relaxed, ordering::acquire, ordering::release
  - [progress reporting with unpack](atomics/progress_reporting_unpack/README.md): unpark, park_timeout
  - [lazy init](atomics/lazy_init/README.md): atomicu64
- fetch and modify
  - [fetch add](atomics/fetch_add/README.md): atomici32
  - [progress reporting with multiple threads](atomics/progress_reporting_multithreads/README.md)
  - [progress reporting with statistics](atomics/progress_reporting_statistics/README.md)
  - [id allocation](atomics/id_allocation/README.md): fetch_add, compare_exchange, compare_exchange_weak, fetch_update, panic::set_hook, panic::catch_unwind, lazy init, once, oncelock

### ordering

- memory model, happens-before relationship
  - [relaxed](ordering/relaxed/README.md), acquire, release, acq_rel, seq_cst
  - [spawn join](ordering/spawn_join/README.md)
- [total modification order](ordering/total_modification_order/README.md)
- [circular reference](ordering/circular_reference/README.md)
- [release acquire](ordering/release_acquire/README.md): unsafe
  - [lock](ordering/lock/README.md): compare_exchange, swap
  - [lazy init box](ordering/lazy_init_box/README.md): atomicptr, box::new, box::into_raw, box::from_raw, drop
  - consume ordering
- [seq_cst](ordering/seq_cst/README.md)
- [fence](ordering/fence/README.md), compiler_fence, [membarrier](https://docs.rs/membarrier/latest/membarrier/)

### spin lock

- [spinlock](spinlock/README.md)
  - swap, compare_exchange, std::hint::spin_loop
  - unsafecell, unsafe func/impl, &mut T, !Sync, lifetime, clippy
  - guard, deref, deref_mut, drop

### channel

- [channel](channel/README.md)
  - simple: mutex, vecdeque, condvar
  - unsafe: unsafe cell, maybeuninit, atomicbool
  - runtime check: thread::park
  - single atomic: atomicu8
  - type check: sender, receiver, arc
  - borrowing: lifetime
  - blocking: phantomdata, thread::park

### arc

- [arc](arc/README.md)
  - basic: nonnull, send, sync, box::leak, fence, acquire, release, drop
  - weak: unsafecell
  - optimized

