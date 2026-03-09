# sccache

- GitHub: [mozilla/sccache](https://github.com/mozilla/sccache)

Rust builds can take a long time, so it is often useful to enable compiler caching with `sccache`.

## Install sccache

Install from source with Cargo:

```bash
cargo install sccache --locked

# ...
    Finished `release` profile [optimized] target(s) in 6m 06s
  Installing ~/.cargo/bin/sccache
   Installed package `sccache v0.14.0` (executable `sccache`)
````

## Set the rustc wrapper

Configure `sccache` as the `rustc` wrapper:

```bash
if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER=sccache
fi
```

It can also be configured at the project level:

```toml
# .cargo/config.toml
[build]
rustc-wrapper = "sccache"
```

## Cache management

```bash
# Show cache statistics
sccache --show-stats

# Reset statistics
sccache --zero-stats

# Remove the entire cache
sccache --stop-server
rm -rf $SCCACHE_DIR
sccache --start-server

# Debug cache misses
SCCACHE_LOG=debug sccache --start-server
```
