# Processor Commands

```bash
cargo doc --open
```

## Rustc

### Compile Target

List a toolchain's supported targets

```bash
rustup target list

aarch64-unknown-linux-gnu
x86_64-unknown-linux-gnu (installed)
```

#### Add target

```bash
rustup target add aarch64-unknown-linux-gnu
```

### Toolchains

Show the active and installed toolchains or profiles

```bash
rustup show
```

---

## Assembler

- crates: [cargo-show-asm](https://crates.io/crates/cargo-show-asm)
- [godbolt.org](https://godbolt.org/)

```bash
cargo install cargo-show-asm
```

### Show Assembly

```bash
cargo asm --target x86_64-unknown-linux-gnu  --lib add_ten
```

```bash
commands::add_ten::add_ten:
        .cfi_sections .debug_frame
        .cfi_startproc
        add dword ptr [rdi], 10
        ret
```

```bash
cargo asm --target aarch64-unknown-linux-gnu --lib add_ten
```

```bash
commands::add_ten::add_ten:
        .cfi_sections .debug_frame
        .cfi_startproc
        ldr w8, [x0]
        add w8, w8, #10
        str w8, [x0]
        ret
```

### Assembly Output

#### add_ten

```bash
cargo asm --target x86_64-unknown-linux-gnu  --lib add_ten
cargo asm --target aarch64-unknown-linux-gnu --lib add_ten
```

#### load_store

```bash
cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::store_x
cargo asm --target aarch64-unknown-linux-gnu --lib load_store::store_x

cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::load_x
cargo asm --target aarch64-unknown-linux-gnu --lib load_store::load_x

cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::atomic_store_x
cargo asm --target aarch64-unknown-linux-gnu --lib load_store::atomic_store_x

cargo asm --target x86_64-unknown-linux-gnu  --lib load_store::atomic_load_x
cargo asm --target aarch64-unknown-linux-gnu --lib load_store::atomic_load_x
```

