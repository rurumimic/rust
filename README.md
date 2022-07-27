# Rust

- [rust](https://www.rust-lang.org)
   - [doc](https://doc.rust-lang.org/book/)
   - [install](https://www.rust-lang.org/tools/install)
   - [learn](https://www.rust-lang.org/learn)
      - [getting started](https://www.rust-lang.org/learn/get-started)
   - [tools](https://www.rust-lang.org/tools)
   - [users forum](https://users.rust-lang.org)
- [cargo](https://github.com/rust-lang/cargo): package manager
   - [doc](https://doc.rust-lang.org/cargo/index.html)
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

