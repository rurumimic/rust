use spinlock::lock::SpinLock;

fn main() {
    let spinlock = SpinLock::new();
    spinlock.lock();

    // locked

    spinlock.unlock();
}
