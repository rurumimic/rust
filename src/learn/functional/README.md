# Functional Features

- book: [Functional Language Features: Iterators and Closures](https://doc.rust-lang.org/book/ch13-00-functional-features.html)

## Closures

Anonymous functions that capture their environment

### Capturing the Environment with Closures

[shirt-company/src/main.rs](shirt-company/src/main.rs)

```rs
fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
    user_preference.unwrap_or_else(|| self.most_stocked())
}
```

closure expression: `|| self.most_stocked()`

```bash
cargo run

The user with preference Some(Red) gets Red
The user with preference None gets Blue
```

### Closure Type Inference and Annotation

```rs
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

```rs
fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // a function definition
let add_one_v2 = |x: u32| -> u32 { x + 1 }; // a fully annotated closure definition
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

### Capturing References or Moving Ownership

- [closures/src/main.rs](closures/src/main.rs)

#### Borrow

```rs
let list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);

let only_borrows = || println!("From closure: {:?}", list);

println!("Before calling closure: {:?}", list);
only_borrows();
println!("After calling closure: {:?}", list);
```

```bash
Before defining closure: [1, 2, 3]
Before calling closure: [1, 2, 3]
From closure: [1, 2, 3]
After calling closure: [1, 2, 3]
```

#### Mutable

```rs
let mut list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);

let mut borrows_mutably = || list.push(7);

borrows_mutably();
println!("After calling closure: {:?}", list);
```

```bash
Before defining closure: [1, 2, 3]
After calling closure: [1, 2, 3, 7]
```

```rs
let mut borrows_mutably = || list.push(7);

println!("Before calling closure: {:?}", list);
borrows_mutably();
```

```bash
   |
21 |     let mut borrows_mutably = || list.push(7);
   |                               -- ---- first borrow occurs due to use of `list` in closure
   |                               |
   |                               mutable borrow occurs here
22 |     println!("Before calling closure: {:?}", list);
   |                                              ^^^^ immutable borrow occurs here
23 |     borrows_mutably();
   |     --------------- mutable borrow later used here
```

#### Move ownership

```rs
let list = vec![1, 2, 3];
println!("Before defining closure: {:?}", list);

thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();
```

```bash
Before defining closure: [1, 2, 3]
From thread: [1, 2, 3]
```

```rs
thread::spawn(|| println!("From thread: {:?}", list))
    .join()
    .unwrap();
```

```bash
   |
34 |     thread::spawn(|| println!("From thread: {:?}", list))
   |                   ^^                               ---- `list` is borrowed here
   |                   |
   |                   may outlive borrowed value `list`
   |
note: function requires argument type to outlive `'static`
  --> src/main.rs:34:5
   |
34 |     thread::spawn(|| println!("From thread: {:?}", list))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `list` (and any other referenced variables), use the `move` keyword
   |
34 |     thread::spawn(move || println!("From thread: {:?}", list))
   |                   ++++
```

#### Moving Captured Values Out of Closures and the Fn Traits

Fn Traits:

- Trait std::ops::[FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html)
- Trait std::ops::[FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html)
- Trait std::ops::[Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html)

##### unwrap_or_else

- Enum core::option::Option::[unwrap_or_else](https://doc.rust-lang.org/src/core/option.rs.html#821-830)

`FnOnce() -> T`: `F` must be able to be called once, take no arguments, and return a `T`

```rs
impl<T> Option<T> {
    pub const fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: ~const FnOnce() -> T,
        F: ~const Destruct,
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

##### sort_by_key

- [rectangles/src/main.rs](rectangles/src/main.rs)
- Struct std::vec::Vec::[sort_by_key](https://doc.rust-lang.org/src/alloc/slice.rs.html#300-303)

`|r| r.width` doesn’t capture, mutate, or move out anything from its environment

```rs
let mut list = [
   Rectangle { width: 10, height: 1 },
   Rectangle { width: 3, height: 5 },
   Rectangle { width: 7, height: 12 },
];

list.sort_by_key(|r| r.width);
```

```bash
[
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
    Rectangle { width: 10, height: 1 },
]
```

```rs
impl<T> [T] {
    pub fn sort_by_key<K, F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> K,
        K: Ord,
    {
        merge_sort(self, |a, b| f(a).lt(&f(b)));
    }
}
```

Error:

```rs
let mut sort_operations = vec![];
let value = String::from("by key called");

list.sort_by_key(|r| {
   sort_operations.push(value);
   r.width
});

println!("{:#?}", list);
```

- cannot move out of `value`, a captured variable in an `FnMut` closure
- move occurs because `value` has type `String`, which does not implement the `Copy` trait

Fix:

```rs
let mut num_sort_operations = 0;

list.sort_by_key(|r| {
   num_sort_operations += 1;
   r.width
});

println!("{:#?}, sorted in {num_sort_operations} operations", list);
```

```bash
[
    Rectangle { width: 3, height: 5 },
    Rectangle { width: 7, height: 12 },
    Rectangle { width: 10, height: 1 },
], sorted in 6 operations
```

---

## Iterators

- book: [Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)

iterators are *lazy*.

- src: [iterators/src/main.rs](iterators/src/main.rs)

```rs
let v1 = vec![1, 2, 3];

let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

### Trait

- Trait std::iter::[Iterator](https://doc.rust-lang.org/stable/src/core/iter/traits/iterator.rs.html#67)
- [The three forms of iteration](https://doc.rust-lang.org/stable/std/iter/index.html#the-three-forms-of-iteration)
  - `iter()`, which iterates over &T.
  - `iter_mut()`, which iterates over &mut T.
  - `into_iter()`, which iterates over T.

```rs
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

#### next: consuming adaptors

```rs
#[test]
fn iterator_immutable() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_take_ownership() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

#### sum

```rs
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // sum takes ownership of the iterator

    assert_eq!(total, 6);
}
```

#### map: iterator adaptors

```rs
fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);
}
```

### With Closures

```rs
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
```

---

## Improving I/O Project - minigrep

- book: [Improving Our I/O Project](https://doc.rust-lang.org/book/ch13-03-improving-our-io-project.html)

[minigrep](/src/learn/minigrep/README.md)#[improving](/src/learn/minigrep/README.md#)

---

## Performance: Loops vs. Iterators

- book: [Comparing Performance: Loops vs. Iterators](https://doc.rust-lang.org/book/ch13-04-performance.html)

- Iterators are one of Rust’s zero-cost abstractions
- no additional runtime overhead

[performance/src/main.rs](performance/src/main.rs):

```rs
let buffer: &mut [i32];
let coefficients: [i64; 12];
let qlp_shift: i16;

for i in 12..buffer.len() {
    let prediction = coefficients.iter()
                                 .zip(&buffer[i - 12..i])
                                 .map(|(&c, &s)| c * s as i64)
                                 .sum::<i64>() >> qlp_shift;
    let delta = buffer[i];
    buffer[i] = prediction as i32 + delta;
}
```

```bash
cargo rustc -- --emit asm
# target/debug/deps/<crate_name>-<hash>.s
cargo rustc --release -- --emit asm
# target/release/deps/<crate_name>-<hash>.s
```

Rust knows that there are 12 iterations, so it “unrolls” the loop. Unrolling is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop.

Now that you know this, you can use iterators and closures without fear!
