# Processor Commands

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
cargo asm --lib "add_ten"
```

```bash
commands::add_ten::add_ten:
        .cfi_sections .debug_frame
        .cfi_startproc
        add dword ptr [rdi], 10
        ret
```

```bash
cargo asm --lib "add_ten" --target aarch64-unknown-linux-gnu
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

