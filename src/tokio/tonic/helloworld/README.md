# Hello World

- tonic: [helloworld](https://github.com/hyperium/tonic/blob/master/examples/helloworld-tutorial.md)

```bash
cargo new helloworld
cd helloworld
```

```bash
cargo add tonic
cargo add prost
cargo add tokio -F macros -F rt-multi-thread
cargo add --build tonic-build
```
