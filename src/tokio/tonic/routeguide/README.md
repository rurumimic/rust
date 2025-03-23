# Route Guide

- tonic: [routeguide](https://github.com/hyperium/tonic/blob/master/examples/routeguide-tutorial.md)

```bash
cargo new routeguide
cd routeguide
```

```bash
cargo add tonic
cargo add prost
cargo add tokio -F macros -F rt-multi-thread -F sync -F time
cargo add tokio-stream
cargo add async-stream
cargo add serde -F derive
cargo add serde_json
cargo add rand
cargo add --build tonic-build
```

