use std::sync::{Arc, Barrier};
use std::thread;

/*

finished barrier x10

*/

fn main() {
    let mut v = Vec::new();

    let barrier = Arc::new(Barrier::new(10));

    for _ in 0..10 {
        let b = barrier.clone();
        let t = thread::spawn(move || {
            b.wait();
            println!("finished barrier");
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}

