use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.store(10, Ordering::Relaxed);
    Y.store(20, Ordering::Relaxed);
}

fn b() {
    let y = Y.load(Ordering::Relaxed);
    let x = X.load(Ordering::Relaxed);
    println!("x = {:>2}, y = {:>2}", x, y);
}

fn main() {
    thread::scope(|s| {
        s.spawn(a);
        s.spawn(b);
    });
}
