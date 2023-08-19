use std::sync::{Arc, Mutex};

fn main() {
    println!("Hello, world!");

    let lock0 = Arc::new(Mutex::new(0));
    let lock1 = lock0.clone();

    let a = lock0.lock().unwrap();

    // DEADLOCK
    // let b = lock1.lock().unwrap();

    println!("{}", a);
    // println!("{}", b);
}
