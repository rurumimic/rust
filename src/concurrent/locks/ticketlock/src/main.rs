use std::sync::Arc;

const NUM_LOOP: usize = 100000;
const NUM_THREADS: usize = 4;

mod ticketlock;

fn main() {
    let lock = Arc::new(ticketlock::TicketLock::new(0));
    let mut v = Vec::new();

    for _ in 0..NUM_THREADS {
        let lock0 = lock.clone();
        let t = std::thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                let mut data = lock0.lock();
                *data += 1;
            }
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    let data = lock.lock();
    let count = *data;

    println!("COUNT = {} (expected = {}).", count, NUM_LOOP * NUM_THREADS);
}
