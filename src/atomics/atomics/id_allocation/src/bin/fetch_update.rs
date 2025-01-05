use std::sync::atomic::{AtomicU32, Ordering};

/// ```rust
/// pub fn fetch_update<F>(&self,
///                        set_order: Ordering,
///                        fetch_order: Ordering,
///                        mut f: F) -> Result<$int_type, $int_type>
/// where F: FnMut($int_type) -> Option<$int_type> {
///     let mut prev = self.load(fetch_order);
///     while let Some(next) = f(prev) {
///         match self.compare_exchange_weak(prev, next, set_order, fetch_order) {
///             x @ Ok(_) => return x,
///             Err(next_prev) => prev = next_prev
///         }
///     }
///     Err(prev)
/// }
/// ```
fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    NEXT_ID
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| n.checked_add(1))
        .expect("too many IDs allocated!")
}

fn main() {
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
    dbg!(allocate_new_id());
}
