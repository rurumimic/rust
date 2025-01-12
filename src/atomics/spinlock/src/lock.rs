use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    //pub fn lock(&self) -> &mut T {
    pub fn lock<'a>(&'a self) -> &'a mut T {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        unsafe { &mut *self.value.get() }
    }

    /// safety: &mut T from lock() must be gone!
    /// (And no cheating by keeping reference to fields of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

#[allow(clippy::trait_impl_incorrect_safety)]
unsafe impl<T> Sync for SpinLock<T> where T: Send {}
