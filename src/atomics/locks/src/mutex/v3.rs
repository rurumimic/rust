use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};

use atomic_wait::{wait, wake_one};

pub struct Mutex<T> {
    state: AtomicU32, // 0: unlocked, 1: locked, 2: locked with waiters
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    pub fn lock(&self) -> MutexGuard<T> {
        if self
            .state
            .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            lock_contended(&self.state);
        }

        MutexGuard { mutex: self }
    }
}

#[cold]
fn lock_contended(state: &AtomicU32) {
    let mut spin_count = 0;

    // 1: no other threads are waiting
    while state.load(Ordering::Relaxed) == 1 && spin_count < 100 {
        spin_count += 1;
        std::hint::spin_loop();
    }

    if state
        .compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed)
        .is_ok()
    {
        return;
    }

    while state.swap(2, Ordering::Acquire) != 0 {
        wait(state, 2);
    }
}

pub struct MutexGuard<'a, T> {
    pub(crate) mutex: &'a Mutex<T>,
}

unsafe impl<T> Sync for MutexGuard<'_, T> where T: Sync {}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        if self.mutex.state.swap(0, Ordering::Release) == 2 {
            wake_one(&self.mutex.state);
        }
    }
}
