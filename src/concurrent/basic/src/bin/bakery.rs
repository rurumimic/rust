use std::ptr::{read_volatile, write_volatile};
use std::sync::atomic::{fence, Ordering};
use std::thread;

/*

COUNT = 400000 (expected = 400000)

*/

const NUM_THREADS: usize = 4;
const NUM_LOOP: usize = 100000;

macro_rules! read_mem {
    ($addr: expr) => {
        unsafe {
            read_volatile($addr)
        }
    };
}

macro_rules! write_mem {
    ($addr: expr, $val: expr) => {
        unsafe {
            write_volatile($addr, $val);
        }
    };
}

static mut LOCK: BakeryLock = BakeryLock {
    entering: [false; NUM_THREADS],
    tickets: [None; NUM_THREADS],
};

static mut COUNT: u64 = 0;

struct LockGuard {
    idx: usize,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        fence(Ordering::SeqCst);
        write_mem!(&mut LOCK.tickets[self.idx], None);
    }
}

struct BakeryLock {
    entering: [bool; NUM_THREADS],
    tickets: [Option<u64>; NUM_THREADS],
}

impl BakeryLock {
    fn lock(&mut self, idx: usize) -> LockGuard {
        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], true);
        fence(Ordering::SeqCst);

        let mut max = 0;
        for i in 0..NUM_THREADS {
            if let Some(t) = read_mem!(&self.tickets[i]) {
                max = max.max(t);
            }
        }

        let ticket = max + 1;
        write_mem!(&mut self.tickets[idx], Some(ticket));

        fence(Ordering::SeqCst);
        write_mem!(&mut self.entering[idx], false);
        fence(Ordering::SeqCst);

        for i in 0..NUM_THREADS {
            if i == idx {
                continue;
            }

            while read_mem!(&self.entering[i]) {}

            loop {
                match read_mem!(&self.tickets[i]) {
                    Some(t) => {
                        if ticket < t || (ticket == t && idx < i) {
                            break;
                        }
                    }
                    None => {
                        break;
                    }
                }
            }
        }
        fence(Ordering::SeqCst);
        LockGuard { idx }
    }
}

fn countup(i: usize) {
    for _ in 0..NUM_LOOP {
        let _lock = unsafe { LOCK.lock(i) };
        unsafe {
            let c = read_volatile(&COUNT);
            write_volatile(&mut COUNT, c + 1);
        }
    }
}

fn main() {
    let mut v = Vec::new();
    for i in 0..NUM_THREADS {
        let t = thread::spawn(move || countup(i));
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }

    println!(
        "COUNT = {} (expected = {})",
        unsafe { COUNT },
        NUM_LOOP * NUM_THREADS
    );
}
