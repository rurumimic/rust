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
let s = String::new();
let s = String::from("== string_literal.to_string()");

s.push_str("string");
s.push('c');

let s3 = s1 + &s2;
format!("{}-{}-{}", s1, s2, s3);

let s = &hello[0..4];

for c in "Зд".chars() {
    println!("{}", c); // З, д
}
```

### hash map

- [hashmap/src/main.rs](hashmap/src/main.rs)
- wiki: [SipHash](https://en.wikipedia.org/wiki/SipHash)

```rs
use std::collections::HashMap; // SipHash

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.entry(String::from("Yellow")).or_insert(50);

let score = scores.get(&team_name);
if let Some(score) = score {
    println!("{}: {}", &team_name, score);
}

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```
