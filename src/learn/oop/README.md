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

Rust doesn’t have inheritance.

- resue: Learn/Generics#[Trait](src/learn/generics/README.md#trait)
- polymorphism: substitute multiple objects for each other at runtime

#### Polymorphism

- Rust uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide.
- This is sometimes called *bounded parametric polymorphism*.

---

## Trait Objects

- book: [Using Trait Objects That Allow for Values of Different Types](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)

### Defining a Trait for Common Behavior

- [gui/src/lib.rs](gui/src/lib.rs)
- [gui/src/main.rs](gui/src/main.rs)

- Struct and Enum != Object.
- Can't add data to a trait object.

```rs
pub trait Draw {
    fn draw(&self);
}
```

```rs
pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

or:

```rs
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
    where T: Draw {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

### Implementing the Trait

```rs
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

```rs
Screen {
    components: vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }),
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ],
};
```

### Trait Objects Perform Dynamic Dispatch

- static dispatch
  - compiler knows what method you’re calling at compile time
  - trait bounds on generics
- dynamic dispatch
  - compiler emits code that at runtime will figure out which method to call
  - trait objects

---

## Object-Oriented Design Pattern

- book: [Implementing an Object-Oriented Design Pattern](https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html)
