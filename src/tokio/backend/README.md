# Backend

## Init

```bash
cargo new backend
cd backend
```

```bash
cargo add axum
cargo add axum -F macros
cargo add tokio -F full
```

## Run

```bash
cargo run
```

### auto-reload

```bash
cargo watch -x run
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

### Packages

#### auto-reload

- reddit: [ANN: Auto reloading development web servers with systemfd/listenfd](https://www.reddit.com/r/rust/comments/8kpea2/ann_auto_reloading_development_web_servers_with/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
  - comment: [get a coffee](https://www.reddit.com/r/rust/comments/8kpea2/comment/dza85pe/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
- issue #2166: [Add auto-reload example](https://github.com/tokio-rs/axum/pull/2166)
  - comment: [is there any advantage using listenfd?](https://github.com/tokio-rs/axum/pull/2166#issuecomment-1912730893)
- [cargo-watch](https://github.com/watchexec/cargo-watch)
- [systemfd](https://github.com/mitsuhiko/systemfd)

```bash
cargo install cargo-watch systemfd
```

```bash
systemfd --no-pid -s http::3000 -- cargo watch -x run
```

#### Anyhow

- examples: [anyhow-error-response](https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs)
- docs.rs: [anyhow](https://docs.rs/anyhow/latest/anyhow/)

```bash
cargo add anyhow
```

#### serde

- [serde.rs](https://serde.rs/)

```bash
cargo add serde -F derive
```

