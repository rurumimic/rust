# Hyper Client

- [hyper.rs](https://hyper.rs/)
  - docs: [server](https://hyper.rs/guides/1/server/hello-world/)
- github: [hyper](https://github.com/hyperium/hyper)
  - [http-body](https://github.com/hyperium/http-body)
- [examples](https://github.com/hyperium/hyper/blob/master/examples/README.md)

## Init

```bash
cargo new hyper-client
cd hyper-client
```

```bash
cargo add hyper -F full
cargo add hyper-util -F full
cargo add tokio -F full
cargo add http-body-util
cargo add pretty_env_logger
```

## Run

```bash
cargo run
cargo run "http://httpbin.org/ip"
```

