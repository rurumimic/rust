use std::sync::atomic::{AtomicI32, Ordering};

fn main() {
    let a = AtomicI32::new(100);
    let b = a.fetch_add(23, Ordering::Relaxed);
    let c = a.load(Ordering::Relaxed);

    assert_eq!(b, 100);
    assert_eq!(c, 123);
}
