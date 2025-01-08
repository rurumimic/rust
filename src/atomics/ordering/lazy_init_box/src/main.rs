use std::sync::atomic::{AtomicPtr, Ordering};

struct Data([u8; 100]);

fn generate_data() -> Data {
    Data([123; 100])
}

fn main() {
    println!("{:p}", get_data());
    println!("{:p}", get_data());
    println!("{:p}", get_data());

    println!("{}", get_data().0[0]);
}

/// ```rust
/// fn get_x() -> u64 {
///    static X: AtomicU64 = AtomicU64::new(0);
///    let mut x = X.load(Ordering::Relaxed);
///
///    if x == 0 {
///        x = calculate_x();
///        X.store(x, Ordering::Relaxed);
///    }
///
///    x
/// }
/// ```
fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Ordering::Acquire);
    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(
            std::ptr::null_mut(),
            p,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            // prevent memory leak.
            // if store failed, drop the pointer.
            drop(unsafe { Box::from_raw(p) }); // doesn't share with other threads
            p = e;
        }
    }

    unsafe { &*p }
}
