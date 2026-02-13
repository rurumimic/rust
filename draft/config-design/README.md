# Config Design

raw schema -> domain validation -> app wiring.

## Run

```sh
cargo run -p app --bin apple
cargo run -p app --bin banana
cargo run -p app --bin orange
```

Unknown-key and validation demo:

```sh
cargo run -p app --bin test_unknown
```

## Structure

- `schema/`: raw settings types + unknown-key policy
- `fruits/`: domain configs + validation
- `app/`: loader + conversion to domain types
- `config/`: sample YAML files

## Purpose

- Separate parsing, validation, and app wiring
- Test unknown-key handling per fruit
- Use `kind` to select a type-safe variant
