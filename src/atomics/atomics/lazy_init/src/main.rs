use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Read x after 1 second:");
    dbg!(get_x());
    dbg!(get_x());
    dbg!(get_x());
    dbg!(get_x());
    dbg!(get_x());
    dbg!(get_x());
    dbg!(get_x());
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Ordering::Relaxed);

    if x == 0 {
        x = calculate_x();
        X.store(x, Ordering::Relaxed);
    }

    x
}

fn calculate_x() -> u64 {
    thread::sleep(Duration::from_secs(1));
    42
}
