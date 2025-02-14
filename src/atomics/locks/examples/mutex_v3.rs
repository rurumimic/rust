use locks::mutex::v3::Mutex;
use std::thread;
use std::time::Instant;

fn bench1() {
    let m = Mutex::new(0);
    std::hint::black_box(&m);

    let start = Instant::now();
    for _ in 0..5_000_000 {
        *m.lock() += 1;
    }
    let duration = start.elapsed();
    println!("Bench 1: locked {} times in {:?}", *m.lock(), duration);
}

fn bench2() {
    let m = Mutex::new(0);
    std::hint::black_box(&m);

    let start = Instant::now();
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..5_000_000 {
                    *m.lock() += 1;
                }
            });
        }
    });

    let duration = start.elapsed();
    println!("Bench 2: locked {} times in {:?}", *m.lock(), duration);
}

fn main() {
    bench1();
    bench2();
}
