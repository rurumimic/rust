# Structure

- book: [struct](https://doc.rust-lang.org/book/ch05-00-structs.html)

## Start

### User

```bash
cargo new user
```

- [user/src/main.rs](user/src/main.rs)

```rs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```
