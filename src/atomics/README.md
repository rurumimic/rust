# Atomics

## Book

"Rust Atomics and Locks by Mara Bos (O’Reilly). Copyright 2023 Mara Bos, 978-1-098-11944-7."

- book: [marbos.nl/atomics](https://marabos.nl/atomics/)
- repo: [rust-atomics-and-locks](https://github.com/m-ou-se/rust-atomics-and-locks)

## Contents

- basics
  - [threads](basics/threads/README.md): scope, builder, leak, ownership
  - [reference counting](basics/reference_counting/README.md): rc, arc, shadowing
- atomics
  - basics
    - [stop flag](atomics/stop_flag/README.md): atomic bool with thread
    - [progress reporting](atomics/progress_reporting/README.md): ordering::relaxed, ordering::acquire, ordering::release
    - [progress reporting with unpack](atomics/progress_reporting_unpack/README.md): unpark, park_timeout
    - [lazy init](atomics/lazy_init/README.md): atomicu64
  - fetch and modify
    - [fetch add](atomics/fetch_add/README.md): atomici32
    - [progress reporting with multiple threads](atomics/progress_reporting_multithreads/README.md)
    - [progress reporting with statistics](atomics/progress_reporting_statistics/README.md)

