# wasm

- doc: [wasm](https://rustwasm.github.io/docs/book/)
  - [Why Rust and WebAssembly?](https://rustwasm.github.io/docs/book/why-rust-and-webassembly.html)
- WebAssembly [Specification](https://webassembly.github.io/spec/core/)

## Setup wasm

1. install [rust](/README.md#install-by-rustup)
2. install wasm-pack
3. install cargo-generate

```bash
# install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# install cargo-generate
cargo install cargo-generate
```

```bash
rustc --version
wasm-pack --help
cargo generate --help
```

### Node v16

- install [npm](https://github.com/rurumimic/supply/blob/master/languages/node.md) v16

```bash
nvm ls-remote
nvm install lts/gallium # v16
nvm use lts/gallium # v16

node --version # v16
```

---

## Game of Life

- doc: [tutorial](https://rustwasm.github.io/docs/book/game-of-life/introduction.html)
- [rustwasm/wasm-pack](https://github.com/rustwasm/wasm-pack)
  - [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [rustwasm/wasm-pack-template](https://github.com/rustwasm/wasm-pack-template)

### Content

- [Hello, World!](wasm-game-of-life/docs/README.md)
- [Implementing Conway's Game of Life](wasm-game-of-life/docs/implementing.md)
- [Testing Conway's Game of Life](wasm-game-of-life/docs/test.md)
- [Debugging](wasm-game-of-life/docs/debug.md)
- [Interactive](wasm-game-of-life/docs/interactive.md)
- [Time Profile](wasm-game-of-life/docs/profile.md)

