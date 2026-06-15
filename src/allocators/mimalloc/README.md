# mimalloc

- github: [purpleprotocol/mimalloc_rust](https://github.com/purpleprotocol/mimalloc_rust)

```bash
RUST_LOG=info cargo run
```

## Dependencies

```bash
# The dependency crate is literally named `mimalloc`, which collides with this
# package's own name, so it is renamed to `mimallocator`.
cargo add mimalloc@0.1 --rename mimallocator
cargo add libmimalloc-sys@0.1 --features extended --optional
```

## Run

```bash
cargo run                       # mimalloc backend (default `stats` feature)
cargo run --no-default-features # noop backend, no introspection
```

mimalloc supports MSVC, so there is no `cfg(not(target_env = "msvc"))` gating
like the jemalloc project needs — the backend is selected by the `stats` feature
instead.

---

## How this differs from the jemalloc project

The two projects share the same shape (`Metrics` handle, compile-time backend
selection, a `Stats` struct), but mimalloc's introspection is much thinner:

| | jemalloc (`tikv-jemalloc-ctl`) | mimalloc (`libmimalloc-sys`) |
| --- | --- | --- |
| Stats source | rich `ctl` MIB tree | only `mi_process_info` |
| Granularity | byte-accurate per size class (`allocated`, `active`, …) | process-level (RSS / commit / faults) |
| Refresh | must `epoch.advance()` first | live, no epoch |
| Fallibility | MIB lookups & reads can fail → `Result` + degradation | infallible → no error type, no `Option` |
| Platforms | excludes MSVC | all platforms |
| Hardening | — | `secure` feature (guard pages, encrypted free lists) |

Because `mi_process_info` is page-granular, the demo allocates and touches 4 MiB
per step (the jemalloc demo only pushes `i32`s) so the numbers actually move.

## Stats

Read via `libmimalloc-sys::mi_process_info` (behind its `extended` feature). All
values are bytes except `page_faults`.

| Metric | Meaning |
| --- | --- |
| `current_rss` | Current resident set size (touched pages). Precise on Windows/macOS; on Linux it is **estimated from `current_commit`**, so the two are usually equal. |
| `peak_rss` | Highest `current_rss` seen so far. |
| `current_commit` | Currently committed memory (read/write accessible memory reserved by mimalloc). |
| `peak_commit` | Highest `current_commit` seen so far. |
| `page_faults` | Hard page-fault count. **Precise on Windows; reports `0` on Linux.** |

`mi_process_info` also returns `elapsed_msecs`, `user_msecs`, and `system_msecs`
(process timing); they are read but not surfaced here, since this demo is about
memory.
