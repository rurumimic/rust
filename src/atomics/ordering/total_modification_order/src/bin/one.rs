use std::sync::atomic::{AtomicI32, Ordering};
use std::thread;

static X: AtomicI32 = AtomicI32::new(0);

fn a() {
    X.fetch_add(5, Ordering::Relaxed);
    X.fetch_add(10, Ordering::Relaxed);
}

fn b() {
    let a = X.load(Ordering::Relaxed);
    let b = X.load(Ordering::Relaxed);
    let c = X.load(Ordering::Relaxed);
    let d = X.load(Ordering::Relaxed);
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
}

fn main() {
    thread::scope(|s| {
        s.spawn(a);
        s.spawn(b);
    });
}
