use spinlock::lock::SpinLock;

fn main() {
    let spinlock = SpinLock::new(1);
    let value: &mut u32 = spinlock.lock();

    *value += 1;

    unsafe { spinlock.unlock() };
}
