# Rust

## Links

<details>
    <summary>rust</summary>

- [rust](https://www.rust-lang.org)
  - [tools](https://www.rust-lang.org/tools)
  - [install](https://www.rust-lang.org/tools/install)
  - [learn](https://www.rust-lang.org/learn)

</details>
<details>
    <summary>docs</summary>

- [rust by example](https://doc.rust-lang.org/rust-by-example/)
- [book](https://doc.rust-lang.org/book/) + [quiz](https://rust-book.cs.brown.edu)
- [rustlings](https://github.com/rust-lang/rustlings)
- core
  - [crate std](https://doc.rust-lang.org/std/)
  - [edition guide](https://doc.rust-lang.org/edition-guide)
  - [cargo](https://github.com/rust-lang/cargo): package manager
    - [doc](https://doc.rust-lang.org/cargo/index.html)
    - [fmt](https://github.com/rust-lang/rustfmt)
    - [clippy](https://github.com/rust-lang/rust-clippy)
  - [rustdoc](https://doc.rust-lang.org/rustdoc)
  - [rustc](https://doc.rust-lang.org/rustc)
  - [error codes](https://doc.rust-lang.org/error_codes)
- skills
  - [cli](https://rust-cli.github.io/book)
  - [wasm](https://rustwasm.github.io/docs/book/)
  - [embedded](https://doc.rust-lang.org/stable/embedded-book)
- master
  - [reference](https://doc.rust-lang.org/reference)
  - [rustonomicon](https://doc.rust-lang.org/stable/nomicon/)
  - [unstable](https://doc.rust-lang.org/nightly/unstable-book)
- [api guidelines](https://rust-lang.github.io/api-guidelines/)
- [little book of rust macros](https://danielkeep.github.io/tlborm/book/index.html)

</details>
<details>
    <summary>community</summary>

- [community](https://www.rust-lang.org/community)
  - [users forum](https://users.rust-lang.org)

</details>
<details>
    <summary>blog</summary>

- blog: [main](https://blog.rust-lang.org/)
- blog: [inside](<https://blog.rust-lang.org/inside-rust>)

</details>
<details>
    <summary>editor</summary>

- [vim](https://github.com/rust-lang/rust.vim)

</details>

---

## Contents

- how to [start](#start)
- read
  - [rust book](#rust-book)
- source code
  - [hello world](src/helloworld/README.md)
  - container: [backyard](src/container/backyard/README.md)
  - app: [tauri](src/tauri/README.md)
  - [wasm](src/wasm/README.md)
    - Game of Life
  - [rustpython](src/python/README.md)
    - call between rust and python
- [concurrent](src/concurrent/README.md)
- [kubernetes](src/kubernetes/README.md)

---

## Start

### Update Rust

```bash
rustup update stable
```

### Install by [rustup](https://rustup.rs)

help:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --help
```

install:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Customize installation

1. `2) Customize installation`
2. `Modify PATH variable? (y/N)`

```bash
Current installation options:


   default host triple: x86_64-apple-darwin
     default toolchain: stable
               profile: default
  modify PATH variable: no
```

#### Modify PATH variable

`~/.zprofile`:

```bash
source "$HOME/.cargo/env"
```

#### Update rustup

```bash
rustup update
```

#### Add components

Install `rust-src` and `rustfmt`, `clippy`:

```bash
rustup component add rust-src
rustup component add rustfmt
rustup component add clippy
```

### Editor

#### VSCode

- extension: [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [.vscode/settings.json](.vscode/settings.json)

`settings.json`:

```json
{
  "rust-analyzer.checkOnSave.command": "clippy",
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  },
  "rust-analyzer.cargo.features": ["all"]
}
```

#### SpaceVim

- spacevim layer
  - [rust](https://spacevim.org/use-vim-as-a-rust-ide/)
- [rust-analyzer](https://rust-analyzer.github.io/manual.html)
- [rust.vim](https://github.com/rust-lang/rust.vim)
  - tags
    - [universal ctags](https://ctags.io)
    - [rusty-tags](https://github.com/dan-t/rusty-tags)

Open SpaceVim Configuration File: `SPC f v d`

```toml
[options]
  autocomplete_method = "coc"

[[layers]]
  name = "autocomplete"

[[layers]]
  name = "lang#rust"
```

in SpaceVim:

```bash
:CocInstall coc-rust-analyzer
```

##### Universal Ctags

on macOS:

```bash
sudo port install global
sudo port install universal-ctags
cargo install rusty-tags
```

in `~/.zshrc`:

```bash
export RUST_SRC_PATH=$(rustc --print sysroot)/lib/rustlib/src/rust/library/
```

in a project directory:

```bash
ctags -R
```

---

## Learn

### Rust [Book](https://doc.rust-lang.org/book/)

with [quiz](https://rust-book.cs.brown.edu)

- [Hello World](src/helloworld/README.md)
- [Guessing Game](src/learn/guessing_game/README.md): pattern match, expection
- [Ownership](src/learn/ownership/README.md): reference, borrow, slice
- [Structure](src/learn/struct/README.md): implementation, associated functions, debug
- [Enumeration](src/learn/enums/README.md): `Option<T>`, pattern match, `if let`
- [Cargo Project](src/learn/cargo/README.md): bin crate, lib crate, `mod`, `pub`, `use`
- [Collections](src/learn/collections/README.md): vector, string, hash map
- [Error Handling](src/learn/errors/README.md): error kind, recover errors, ? operator
- [Generics](src/learn/generics/README.md): trait, where clauses, lifetimes, lifetime elision rules, static lifetime
- [Testing](src/learn/testing/README.md): test macro, super, left!=right, custom failure message, `Result<T, E>`, should_panic, expected, help, threads, show output, select, ignore, unit tests, integration tests, submodules
- I/O Project - [minigrep](src/learn/minigrep/README.md): refactoring, CLI, file i/o, error handling in main/lib, unittest, process exit, environment variables
- [Functional](src/learn/functional/README.md): closures, Fn traits, iterators, imporring minigrep, performance
- [More Cargo](src/learn/cargo/README.md#more-cargo): profiles, workspace, documentation, crates.io, binary, custom commands
- [Smart Pointers](src/learn/pointer/README.md): reference count, box, rc, refcell, weak, deref, drop
- [Concurrency](src/learn/concurrency/README.md): threads, spawn, join, channel, mutex, arc, sync, send
- [Object-Oriented](src/learn/oop/README.md): encapsulation, public & private, inheritance, trait, polymorphism, dynamic dispatch, design pattern with types
- [Patterns & Matching](src/learn/pattern/README.md): refutable patterns, irrefutable patterns, match, match guard, `@` binding, `_` ignore
- [Advanced](src/learn/advanced/README.md)
  - unsafe: raw pointer, extern, mutable static variable
  - traits: associated types, fully qualified syntax, supertraits, newtype
  - types: type aliases, never type, `Sized` trait
  - function pointer, return clousre
  - macro: declarative macro, `macro_rules!`, procedural macro, `derive`, attribute-like, function-like
- [Final Project](src/learn/final_project/README.md) - Multithreaded Web Server
- Appendix
  - read: [derivable traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
