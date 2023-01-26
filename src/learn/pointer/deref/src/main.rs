use std::ops::Deref;

struct MyBox<T>(T); // Tuple Struct

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let my_box = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *my_box);

    // deref coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // &MyBox<String> -> deref -> &String
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    // &String -> deref -> &str
    println!("Hello, {name}!");
}
