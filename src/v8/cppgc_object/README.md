# C++ GC Objects

- denoland/rusty_v8: [examples/cppgc-object.rs](https://github.com/denoland/rusty_v8/blob/ab019251a4c45ee4edb4f7415eec762a589d87f8/examples/cppgc-object.rs)

## Run

```bash
cargo run
```

```bash
Wrappable::trace() dont gc me
Wrappable::drop() gc me pls
Wrappable::drop() dont gc me
```

