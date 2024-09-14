# Tower Middleware

- [tower-rs](https://github.com/tower-rs)
  - [inventing the service trait](https://tokio.rs/blog/2021-05-14-inventing-the-service-trait): [tower::Service](https://docs.rs/tower/latest/tower/trait.Service.html)
  - [building a middleware from scratch](https://github.com/tower-rs/tower/blob/master/guides/building-a-middleware-from-scratch.md): [tower/src/timeout/mod.rs](https://github.com/tower-rs/tower/blob/master/tower/src/timeout/mod.rs)
  - issue: [consider changing middleware erros to box](https://github.com/tower-rs/tower/issues/131)
- rust api guidelines
  - future proofing: [data structures do not duplicate derived trait bounds](https://rust-lang.github.io/api-guidelines/future-proofing.html#c-struct-bounds)
- rust
  - [trait from](https://doc.rust-lang.org/stable/std/convert/trait.From.html)
- github
  - [pin-project](https://github.com/taiki-e/pin-project)
- read
  - medium: [backpressure explained — the resisted flow of data through software](https://medium.com/@jayphelps/backpressure-explained-the-flow-of-data-through-software-2350b3e77ce7)
  - aws: [using load shedding to avoid overload](https://aws.amazon.com/builders-library/using-load-shedding-to-avoid-overload/)
- youtube
  - [the why, what, and how of pinning in rust](https://www.youtube.com/watch?v=DkMwYxfSYNQ&ab_channel=JonGjengset) by jon gjengset

## Init

```bash
cargo new tower-middleware
cd tower-middleware
```

```bash
cargo add tower
cargo add pin-project

cargo add hyper -F full
cargo add hyper-util -F full
cargo add tokio -F full
cargo add http-body-util
```

## Run

```bash
cargo run
```

### Timeout

```bash
curl -i localhost:3000/slow
```

```bash
Listening on http://127.0.0.1:3000
Error serving connection: hyper::Error(User(Service), TimeoutError(()))
```

