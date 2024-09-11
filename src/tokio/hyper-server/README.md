# Hyper Server

- [hyper.rs](https://hyper.rs/)
  - docs: [server](https://hyper.rs/guides/1/server/hello-world/)
- github: [hyper](https://github.com/hyperium/hyper)
  - [http-body](https://github.com/hyperium/http-body)
- [examples](https://github.com/hyperium/hyper/blob/master/examples/README.md)
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
cargo add tower
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

### Graceful shutdown

Request takes 5 seconds:

```bash
curl localhost:3000/slow
```

Timeout 3 second limit + Graceful Shutdown:

```bash
cargo run

Listening on http://127.0.0.1:3000
Tower Middleware Process request: GET /slow
^Cgraceful shutdown signal received
timed out wait for all connections to close
```

Graceful shutdown if all requests are returned:

```bash
cargo run

Listening on http://127.0.0.1:3000
Tower Middleware Process request: GET /slow
^Cgraceful shutdown signal received
all connections gracefully closed
```

