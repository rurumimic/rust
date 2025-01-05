use std::sync::atomic::{AtomicU32, Ordering};

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    if id >= 1000 {
        NEXT_ID.fetch_sub(1, Ordering::Relaxed);
        panic!("too many IDs allocated!");
    }
    id
}

fn main() {
    for _ in 0..=1000 {
        dbg!(allocate_new_id());
    }
}
