use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED
        .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        unsafe { DATA.push('!') };
        LOCKED.store(false, Ordering::Release);
    }
}

fn g() {
    if !LOCKED.swap(true, Ordering::Acquire) {
        unsafe { DATA.push('!') };
        LOCKED.store(false, Ordering::Release);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
            s.spawn(g);
        }
    });

    assert!(unsafe { DATA.len() } > 0);
    assert!(unsafe { DATA.chars().all(|c| c == '!') });
}
