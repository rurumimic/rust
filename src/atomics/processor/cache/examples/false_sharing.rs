use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Instant;

#[repr(align(128))]
struct Aligned(AtomicU64);

static A: [Aligned; 3] = [
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
];

fn main() {
    black_box(&A);

    thread::spawn(|| {
        A[0].0.store(1, Ordering::Relaxed);
        A[2].0.store(1, Ordering::Relaxed);
    });

    let start = Instant::now();

    for _ in 0..1_000_000_000 {
        black_box(A[1].0.load(Ordering::Relaxed));
    }

    println!("{:?}", start.elapsed());
}
