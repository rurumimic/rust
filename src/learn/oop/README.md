# Object-Oriented Programming

- book: [Object-Oriented Programming Features of Rust](https://doc.rust-lang.org/book/ch17-00-oop.html)

## Characteristics of Object-Oriented Languages

### Encapsulation

- [average/src/lib.rs](average/src/lib.rs)
- [average/src/main.rs](average/src/main.rs)

```rs
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![], // field `list` of struct `AveragedCollection` is private: private field
            average: 0.0, // field `average` of struct `AveragedCollection` is private: private field
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn average(&self) -> f64 {
        self.average
    }
}
```

```rs
let mut s = AveragedCollection::new();
assert_eq!(s.average(), 0.0);

s.add(1);
s.add(2);
assert_eq!(s.average(), 1.5);
```

### Inheritance

- resue: Learn/Generics#[Trait](src/learn/generics/README.md#trait)
- polymorphism: substitute multiple objects for each other at runtime

#### Polymorphism

- Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. 
- This is sometimes called *bounded parametric polymorphism*.

---


