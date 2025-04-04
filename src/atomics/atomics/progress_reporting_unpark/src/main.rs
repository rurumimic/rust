use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = AtomicUsize::new(0);
    let main_thread = thread::current();

    thread::scope(|s| {
        // background thread
        s.spawn(|| {
            for i in 0..100 {
                progress_item(i as u64);
                // num_done.store(i + 1, Ordering::Relaxed);
                num_done.store(i + 1, Ordering::Release);
                main_thread.unpark();
            }
        });

        // main thread
        loop {
            let n = num_done.load(Ordering::Acquire);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100");
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn progress_item(i: u64) {
    thread::sleep(Duration::from_millis(i));
}
