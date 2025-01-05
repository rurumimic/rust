use std::sync::atomic::{AtomicU16, Ordering};

fn allocate_new_id() -> u16 {
    static NEXT_ID: AtomicU16 = AtomicU16::new(0);
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    assert!(id < 1000, "too many IDs allocated!");
    id
}

fn main() {
    dbg!(allocate_new_id()); // 0

    for _ in 1..1000 {
        allocate_new_id();
    }

    println!("overflowing the counter... (this might take a minute)");

    std::panic::set_hook(Box::new(|_| {}));

    for _ in 1..=u16::MAX {
        let _ = std::panic::catch_unwind(|| allocate_new_id());
    }

    println!("overflowed!");

    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
}
