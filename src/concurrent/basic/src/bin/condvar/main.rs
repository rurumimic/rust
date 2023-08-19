use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use basic::condvar::lib::{child, parent};

/*

parent
child 0
child 1

*/

fn main() {
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = Arc::clone(&pair0);
    let pair2 = Arc::clone(&pair0);

    let c0 = thread::spawn(move || child(0, pair0));
    let c1 = thread::spawn(move || child(1, pair1));
    let p = thread::spawn(move || parent(pair2));

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}

