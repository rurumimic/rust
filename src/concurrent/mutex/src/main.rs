#![allow(dead_code, unused)]

use std::ptr::{read_volatile, write_volatile};
use std::sync::atomic::{fence, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock};
use std::thread;

fn main() {
    // ex_mutex();
    // ex_condvar();
    // ex_rwlock();
    // ex_barrier();
    ex_bakery();
}

/* Mutex */
fn add_one(lock: Arc<Mutex<u64>>) {
    loop {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
        break;
    }
}

fn ex_mutex() {
    let lock0 = Arc::new(Mutex::new(0));
    let lock1 = lock0.clone();

    let thread0 = thread::spawn(move || {
        add_one(lock0);
    });

    let thread1 = thread::spawn(move || {
        add_one(lock1);
    });

    thread0.join().unwrap();
    thread1.join().unwrap();
}

/* Condvar */

fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();

    // cvar.wait_while(started, |started| !*started).unwrap();
    // or
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("child {}", id);
}

fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_all();

    println!("parent");
}

fn ex_condvar() {
    let pair0 = Arc::new((Mutex::new(false), Condvar::new()));
    let pair1 = Arc::clone(&pair0);
    let pair2 = Arc::clone(&pair0);

    let c0 = thread::spawn(move || child(0, pair0));
    let c1 = thread::spawn(move || child(1, pair1));
    let p = thread::spawn(move || parent(pair2));

    c0.join().unwrap();
    c1.join().unwrap();
    p.join().unwrap();
}

/* RwLock */
fn ex_rwlock() {
    let lock = RwLock::new(10);

    {
        let v1 = lock.read().unwrap();
        let v2 = lock.read().unwrap();
        println!("v1 = {}", v1);
        println!("v2 = {}", v2);
    }

    {
        let mut v = lock.write().unwrap();
        *v = 7;
        println!("v = {}", v);
    }
}

/* Barrier */
fn ex_barrier() {
    let mut v = Vec::new();

    let barrier = Arc::new(Barrier::new(10));

    for _ in 0..10 {
        let b = barrier.clone();
        let t = thread::spawn(move || {
            b.wait();
            println!("finished barrier");
        });
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}

/* Bakery */
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

fn ex_bakery() {
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
