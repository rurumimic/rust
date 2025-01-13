use std::thread;

use spinlock::lock::SpinLock;

fn is_sync<S: Sync>(_: &S) {}

fn main() {
    let spinlock = SpinLock::new(Vec::new());

    thread::scope(|s| {
        s.spawn(|| spinlock.lock().push(1));
        s.spawn(|| {
            let mut guard = spinlock.lock();
            guard.push(2);
            guard.push(2);
        });
    });

    let guard = spinlock.lock();
    assert!(guard.as_slice() == [1, 2, 2] || guard.as_slice() == [2, 2, 1]);
    println!("Array: {:?}", guard.as_slice());

    is_sync(&guard);
}
