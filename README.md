# Rust

- [rust](https://www.rust-lang.org)
   - [book](https://doc.rust-lang.org/book/)
   - [install](https://www.rust-lang.org/tools/install)
   - [learn](https://www.rust-lang.org/learn)
      - [getting started](https://www.rust-lang.org/learn/get-started)
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
rsutup update
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

---

## Codes

- [Book](https://doc.rust-lang.org/book/)
  - [Hello World](src/helloworld/README.md)
  - [Guessing Game](src/guessing_game/README.md)
  - [Ownership](src/ownership/README.md)
