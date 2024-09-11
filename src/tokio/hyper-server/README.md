# Hyper Server

- [hyper.rs](https://hyper.rs/)
  - docs: [server](https://hyper.rs/guides/1/server/hello-world/)
- github: [hyper](https://github.com/hyperium/hyper)
  - [http-body](https://github.com/hyperium/http-body)
- examples
  - [hello](https://github.com/hyperium/hyper/blob/master/examples/hello.rs)
  - [service_struct_impl](https://github.com/hyperium/hyper/blob/master/examples/service_struct_impl.rs)
  - [echo](https://github.com/hyperium/hyper/blob/master/examples/echo.rs)

## Init

```bash
cargo new hyper-server
cd hyper-server
```

```bash
cargo add hyper -F full
cargo add hyper-util -F full
cargo add tokio -F full
cargo add http-body-util
```

## Run

```bash
cargo run
```

### Echo

```bash
curl -X POST -H "Content-Type: text/plain" --data "this is raw data"
curl -X POST -H "Content-Type: text/plain" --data "Yo, banana boy\!" localhost:3000/echo/reversed
```

