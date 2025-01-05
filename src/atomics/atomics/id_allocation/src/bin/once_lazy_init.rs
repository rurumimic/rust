use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Once;

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        KEY.store(generate_random_key(), Ordering::Relaxed);
    });

    KEY.load(Ordering::Relaxed)
}

fn generate_random_key() -> u64 {
    42
}

fn main() {
    dbg!(get_key());
    dbg!(get_key());
    dbg!(get_key());
}
