use std::sync::atomic::{fence, AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

static mut DATA: [u64; 10] = [0; 10];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = some_calculation(i);
            unsafe { DATA[i] = data };
            READY[i].store(true, Ordering::Release);
        });
    }

    thread::sleep(Duration::from_millis(500));

    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));

    if ready.contains(&true) {
        fence(Ordering::Acquire); // for main thread with one fence

        // data read after fence
        for i in 0..10 {
            if ready[i] {
                println!("Data[{}] = {}", i, unsafe { DATA[i] });
            }
        }
    }
}

fn some_calculation(i: usize) -> u64 {
    thread::sleep(Duration::from_millis(400 + i as u64 % 3 * 100));
    i as u64
}
