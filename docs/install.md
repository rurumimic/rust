# Install

- [rustup](https://rustup.rs)

## stable-x86_64-unknown-linux-gnu

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -q -y --profile complete
```

**help**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --help
```

### Update Rust

```bash
rustup update stable
```

### Modify PATH variable

`~/.zprofile`:

```bash
source "$HOME/.cargo/env"
```

---

## VSCode

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

## Universal Ctags

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

