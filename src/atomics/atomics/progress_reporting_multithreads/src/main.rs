use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

fn main() {
    let num_done = &AtomicUsize::new(0);
    let main_thread = &thread::current();

    thread::scope(|s| {
        // background threads
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    progress_item(t * 25 + i);
                    num_done.fetch_add(1, Ordering::Release);
                    main_thread.unpark();
                }
            });
        }

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
