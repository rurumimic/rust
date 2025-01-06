use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    thread::spawn(|| {
        DATA.store(42, Ordering::Relaxed);
        READY.store(true, Ordering::Release);
    });

    while !READY.load(Ordering::Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("Waiting for data...");
    }

    println!("DATA: {}", DATA.load(Ordering::Relaxed));
}
