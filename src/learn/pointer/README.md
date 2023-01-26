# Smart Pointers

- book: [Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)

pointer: a variable contains an address in memory  
smart pointers: data structures act like a pointer and have additional metadata and capabilities

- `Box<T>`: allocate values on the heap
- `Rc<T>`: a reference counting type that enables multiple ownership
- `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

---

## Box

- book: [Using `Box<T>` to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)

### Store data on the heap

- Struct [std::boxed::Box](https://doc.rust-lang.org/std/boxed/struct.Box.html)

```rs
pub struct Box<
    T: ?Sized,
    #[unstable(feature = "allocator_api", issue = "32838")] A: Allocator = Global,
>(Unique<T>, A);

impl<T: ?Sized, A: Allocator> const Deref for Box<T, A> {}
unsafe impl<#[may_dangle] T: ?Sized, A: Allocator> Drop for Box<T, A> {}
```

```rs
Box::new(5)
```

### Recursive types

- instead of storing a value directly
- store the value indirectly by stroing a pointer

[box/src/main.rs](box/src/main.rs)

```rs
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("recursive type: {:?}", list); // Cons(1, Cons(2, Cons(3, Nil)))
}
```

Boxes provide only the indirection and heap allocation

---

## Deref Trait

- book: [Treating Smart Pointers Like Regular References with the Deref Trait](https://doc.rust-lang.org/book/ch15-02-deref.html)

dereference operator: `*`

### follow the pointer

[deref/src/main.rs](deref/src/main.rs)

```rs
let x = 5;
let y = &x;
let z = Box::new(x);
let my_box = MyBox::new(x);

assert_eq!(5, x);
assert_eq!(5, *y);
assert_eq!(5, *z);
assert_eq!(5, *my_box); // &MyBox<i32> -> deref -> &i32 -> *(&i32)
```

```rs
use std::ops::Deref;

struct MyBox<T>(T); // Tuple Struct

impl<T> Deref for MyBox<T> {
    type Target = T;  // associated type

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

```rs
*(my_box.deref())
```

### Deref Corecion

```rs
let m = MyBox::new(String::from("Rust")); // MyBox<String>
hello(&m); // &MyBox<String> -> deref -> &String
hello(&(*m)[..]);

fn hello(name: &str) { // &String -> deref -> &str
    println!("Hello, {name}!");
}
```

### DerefMut

- From `&T` to `&U` when `T: Deref<Target=U>`
- From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
- From `&mut T` to `&U` when `T: Deref<Target=U>`

Not possible: From `&T` to `&mut U`

---

## Drop Trait

- book: [Running Code on Cleanup with the Drop Trait](https://doc.rust-lang.org/book/ch15-03-drop.html)

[drop/src/main.rs](drop/src/main.rs)

```rs
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```

```bash
CustomSmartPointer created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

### drop value early

```rs
use std::mem:drop;

drop(c);
```

```bash
CustomSmartPointer created.
Dropping CustomSmartPointer with data `my stuff`!
CustomSmartPointer dropped before the end of main.
Dropping CustomSmartPointer with data `other stuff`!
```

---

## Reference Counting

- book: [Rc<T>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)

`Rc<T>` is only for use in single-threaded scenarios.

### Share data

value used here after move:

```rs
9  |     let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
   |         - move occurs because `a` has type `List`, which does not implement the `Copy` trait
10 |     let b = Cons(3, Box::new(a));
   |                              - value moved here
11 |     let c = Cons(4, Box::new(a));
   |                              ^ value used here after move
```

with ref count:

```rs
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
let b = Cons(3, Rc::clone(&a));
let c = Cons(4, Rc::clone(&a));
```

- `Rc::clone`: count reference
- doesn't make a deep copy of all the data
- only increments the reference count
- visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count

### Cloning and Counting

```rs
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
println!("count after creating a = {}", Rc::strong_count(&a));
let b = Cons(3, Rc::clone(&a));
println!("count after creating b = {}", Rc::strong_count(&a));
{
    let c = Cons(4, Rc::clone(&a));
    println!("count after creating c = {}", Rc::strong_count(&a));
}
println!("count after c goes out of scope = {}", Rc::strong_count(&a));
```

```bash
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of = 2
```

---

## Interior mutability Pattern

- book: [RefCell<T> and the Interior Mutability Pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)

### Enforcing Borrowing Rules at Runtime with RefCell<T>

- `RefCell<T>` type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.
- Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios.

#### Borrowing rules

- At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
- References must always be valid.

#### Box vs Rc vs RefCell

- **Box**
  - 1 owner
  - borrows checked
    - compile time: immutable & mutable (compiler error)
- **Rc**
  - multiple owners
  - borrows checked
    - compile time: immutable (compiler error)
- **RefCell**
  - 1 owner
  - borrows checked
    - runtime: immutable & mutable (program panic)
  - can mutate the value inside the immutable `RefCell`

#### Halting problem

- Wikipedia: [Halting problem](https://en.wikipedia.org/wiki/Halting_problem)

the problem of determining, from a description of an arbitrary computer program and an input, whether the program will finish running, or continue to run forever.

### Interior Mutability: A Mutable Borrow to an Immutable Value

#### Mock Objects

- [refcell/src/lib.rs](refcell/src/lib.rs)

```rs
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}
```

- `borrow` get an **immutable** reference in `RefCell<Vec<String>>`
  - return `Ref<T>`
- `borrow_mut` get a **mutable** reference in `RefCell<Vec<String>>`
  - return `RefMut<T>`

#### Track of Borrows at Runtime

```bash
cargo test

panicked at 'already borrowed: BorrowMutError'
```

```rs
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}
```

### Combining Rc<T> and RefCell<T>

`Rc<T>` that holds a `RefCell<T>`: get a value that can have multiple owners and that you can mutate

[refcell/src/main.rs](refcell/src/main.rs)

```rs
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
```

```bash
a after = Cons(RefCell { value: 15 }, Nil)
b after = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c after = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

---

## Reference Cycle

- book: [Reference Cycles Can Leak Memory](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html)
