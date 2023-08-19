use std::sync::{Arc, Mutex};
use std::thread;

use basic::mutex::add_one;

/*

1
2
3
4
...

*/

fn main() {
    let lock0 = Arc::new(Mutex::new(0));
    let lock1 = lock0.clone();

    let thread0 = thread::spawn(move || {
        add_one(lock0);
    });

    let thread1 = thread::spawn(move || {
        add_one(lock1);
    });

    thread0.join().unwrap();
    thread1.join().unwrap();
}

