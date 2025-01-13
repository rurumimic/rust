use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

/// impl<'a, T> Deref for Guard<'a, T> {
impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

/// impl<'a, T> DerefMut for Guard<'a, T> {
impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

/// impl<'a, T> Drop for Guard<'a, T> {
impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

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

    /// 1. minimal design:
    /// pub fn lock(&self) -> &mut T {
    /// 2. unsafe design:
    /// pub fn lock<'a>(&'a self) -> &'a mut T {
    /// 3. with guard:
    pub fn lock<'a>(&'a self) -> Guard<'a, T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        // 2. unsafe design:
        // unsafe { &mut *self.value.get() }
        Guard { lock: self }
    }

    // safety: &mut T from lock() must be gone!
    // (And no cheating by keeping reference to fields of that T around!)
    //pub unsafe fn unlock(&self) {
    //    self.locked.store(false, Ordering::Release);
    //}
}

#[allow(clippy::trait_impl_incorrect_safety)]
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

/// unsafe impl<'a, T> Sync for Guard<'a, T> where T: Send {}
unsafe impl<T> Sync for Guard<'_, T> where T: Send {}
