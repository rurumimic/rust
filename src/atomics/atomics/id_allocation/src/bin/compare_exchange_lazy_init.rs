use std::sync::atomic::{AtomicU64, Ordering};

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => new_key,
            Err(v) => v,
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    42
}

fn main() {
    dbg!(get_key());
    dbg!(get_key());
    dbg!(get_key());
}
