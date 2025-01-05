use std::sync::atomic::{AtomicU32, Ordering};

fn increment(a: &AtomicU32) {
    let mut current = a.load(Ordering::Relaxed);
    loop {
        let new = current + 1;
        match a.compare_exchange(current, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(v) => current = v,
        }
    }
}

fn main() {
    let a = AtomicU32::new(0);
    println!("a: {}", a.load(Ordering::Relaxed));

    for _ in 0..3 {
        increment(&a);
        println!("a: {}", a.load(Ordering::Relaxed));
    }
}
