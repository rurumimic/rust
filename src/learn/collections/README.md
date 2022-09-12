# Common Collections

- book: [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

Collections stored on the **heap**.

## Start

### vector

- [vector/src/main.rs](vector/src/main.rs)

```rs
let v = vec![1, 2, 3]; // macro

let mut v = Vec::new();
v.push(1);

let first = &v[0];
let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {}", third),
    None => (),
}

for i in &v {
    println!("{}", i);
}
```

### string

- [strings/src/main.rs](strings/src/main.rs)

```rs

```

### hash map

- [hashmap/src/main.rs](hashmap/src/main.rs)

```rs

```
