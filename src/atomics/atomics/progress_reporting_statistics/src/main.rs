use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);
    let main_thread = &thread::current();

    thread::scope(|s| {
        // background threads
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    progress_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Ordering::Relaxed);
                    total_time.fetch_add(time_taken, Ordering::Relaxed);
                    max_time.fetch_max(time_taken, Ordering::Relaxed);
                    main_thread.unpark();
                }
            });
        }

        // main thread
        loop {
            let total_time = Duration::from_micros(total_time.load(Ordering::Relaxed));
            let max_time = Duration::from_micros(max_time.load(Ordering::Relaxed));
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            if n == 0 {
                println!("Working.. {n}/100");
            } else {
                println!(
                    "Working.. {n}/100, avg time: {:?}, peak time: {:?}",
                    total_time / n as u32,
                    max_time
                );
            }
            //thread::sleep(Duration::from_secs(1));
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

fn progress_item(i: u64) {
    thread::sleep(Duration::from_millis(i));
}
