# jemalloc

- github: [tikv/jemallocator](https://github.com/tikv/jemallocator)

```bash
RUST_LOG=warn cargo run
```

## Dependencies

```bash
cargo add tikv-jemallocator@0.7 --features stats --target 'cfg(not(target_env = "msvc"))'
cargo add tikv-jemalloc-ctl@0.7 --features stats --target 'cfg(not(target_env = "msvc"))'
```

## Run

### Current Host Target

```bash
rustc -vV
rustc --print cfg | grep target_env
```

### Rustup Targets

```bash
rustup target list
```

### Run on Linux

```bash
cargo run --target x86_64-unknown-linux-gnu
```

### Run on Windows

Add MSVC Target:

```bash
rustup target add x86_64-pc-windows-msvc
cargo run --target x86_64-pc-windows-msvc
```

---

## Stats

Read via `tikv-jemalloc-ctl` after advancing the epoch. All values are in bytes.

| Metric | Meaning |
| --- | --- |
| `allocated` | Total bytes currently requested by the application (live allocations). |
| `active` | Total bytes in active pages backing allocations; rounds each request up to its size class, so it is `>= allocated`. |
| `resident` | Total bytes in physically resident pages (maps to RSS), including allocator overhead and unused parts of active pages. |
| `mapped` | Total bytes of virtual address space mapped by the allocator. |
| `retained` | Total bytes of virtual memory returned to the OS but retained by the allocator for fast reuse (not counted in `resident`). |
| `metadata` | Total bytes used by jemalloc for its own bookkeeping (arenas, tcaches, internal structures). |

Typical ordering: `allocated <= active <= resident <= mapped`.

