# string sum

## Init

```bash
mkdir string_sum
cd string_sum
```

### venv

```bash
uv venv

Using CPython 3.11.7 interpreter:
Creating virtual environment at: .venv
Activate with: source .venv/bin/activate
```

```bash
source .venv/bin/activate
```

### maturin

```bash
uv pip install maturin
```

```bash
maturin init

✔ 🤷 Which kind of bindings to use?
  📖 Documentation: https://maturin.rs/bindings.html · pyo3
  ✨ Done! Initialized project ./string_sum
```

```bash
string_sum
├── Cargo.toml
├── pyproject.toml
├── README.md
└── src
    └── lib.rs

1 directory, 4 files
```

