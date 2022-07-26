# Rust

- [rust](https://www.rust-lang.org)
   - [book](https://doc.rust-lang.org/book/) + [quiz](https://rust-book.cs.brown.edu)
   - [install](https://www.rust-lang.org/tools/install)
   - [learn](https://www.rust-lang.org/learn)
      - [getting started](https://www.rust-lang.org/learn/get-started)
   - [api guidelines](https://rust-lang.github.io/api-guidelines/)
   - [tools](https://www.rust-lang.org/tools)
   - [users forum](https://users.rust-lang.org)
- [cargo](https://github.com/rust-lang/cargo): package manager
   - [doc](https://doc.rust-lang.org/cargo/index.html)
   - [fmt](https://github.com/rust-lang/rustfmt)
   - [clippy](https://github.com/rust-lang/rust-clippy)
- editors
   - [vim](https://github.com/rust-lang/rust.vim)

## Start

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

[Book](https://doc.rust-lang.org/book/) + [quiz](https://rust-book.cs.brown.edu):

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
