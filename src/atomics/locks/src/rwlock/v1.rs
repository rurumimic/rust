use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU32, Ordering};

use atomic_wait::{wait, wake_all, wake_one};

pub struct RwLock<T> {
    state: AtomicU32, // 0 (unlocked), readers or u32::MAX (write locked)
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for RwLock<T> where T: Send + Sync {}

impl<T> RwLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn read(&self) -> ReadGuard<T> {
        let mut s = self.state.load(Ordering::Relaxed);

        loop {
            if s < u32::MAX {
                assert!(s != u32::MAX - 1, "too many readers");
                match self.state.compare_exchange_weak(
                    s,
                    s + 1,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => return ReadGuard { rwlock: self },
                    Err(e) => s = e,
                }
            }

            if s == u32::MAX {
                wait(&self.state, u32::MAX);
                s = self.state.load(Ordering::Relaxed);
            }
        }
    }

    pub fn write(&self) -> WriteGuard<T> {
        while let Err(s) =
            self.state
                .compare_exchange(0, u32::MAX, Ordering::Acquire, Ordering::Relaxed)
        {
            wait(&self.state, s);
        }
        WriteGuard { rwlock: self }
    }
}

pub struct ReadGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

impl<T> Deref for ReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        if self.rwlock.state.fetch_sub(1, Ordering::Release) == 1 {
            wake_one(&self.rwlock.state);
        }
    }
}

pub struct WriteGuard<'a, T> {
    rwlock: &'a RwLock<T>,
}

impl<T> Deref for WriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.rwlock.value.get() }
    }
}

impl<T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.rwlock.value.get() }
    }
}

impl<T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        self.rwlock.state.store(0, Ordering::Release);
        wake_all(&self.rwlock.state);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_rwlock() {
        let rwlock = RwLock::new(12);

        let r1 = rwlock.read();
        let r2 = rwlock.read();
        assert_eq!(*r1, 12);
        assert_eq!(*r2, 12);

        drop(r1);
        drop(r2);

        let mut w = rwlock.write();
        *w = 34;
        drop(w);

        let r3 = rwlock.read();
        assert_eq!(*r3, 34);
    }

    #[test]
    fn test_multithread() {
        let rwlock = Arc::new(RwLock::new(0));

        let r1 = rwlock.read();
        let r2 = rwlock.read();

        let rwlock_clone = Arc::clone(&rwlock);
        let writer = thread::spawn(move || {
            let mut w = rwlock_clone.write();
            *w = 123;
        });

        thread::sleep(Duration::from_secs(1));
        assert_eq!(*r1, 0);

        drop(r1);
        drop(r2);

        writer.join().unwrap();

        let r3 = rwlock.read();
        assert_eq!(*r3, 123);
    }
}
