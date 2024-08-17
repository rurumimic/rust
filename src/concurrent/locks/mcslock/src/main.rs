use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 4;

mod mcslock;

fn main() {
    let lock = Arc::new(mcslock::MCSLock::new(0));
    let mut v = Vec::new();

    for _ in 0..NUM_THREADS {
        let lock0 = lock.clone();
        let t = std::thread::spawn(move || {
            let mut node = mcslock::MCSNode::new();
            for _ in 0..NUM_LOOP {
                let mut data = lock0.lock(&mut node);
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    let mut node = mcslock::MCSNode::new();
    let data = lock.lock(&mut node);
    println!("COUNT = {} (expected = {})", *data, NUM_LOOP * NUM_THREADS);
}
