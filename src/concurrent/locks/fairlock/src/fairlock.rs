use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{fence, AtomicBool, AtomicUsize, Ordering};

pub const NUM_LOCK: usize = 8; // maximum number of threads. num_lock has to be a power of 2

const MASK: usize = NUM_LOCK - 1; // x % NUM_LOCK == x & MASK

pub struct FairLock<T> {
    waiting: Vec<AtomicBool>, // threads attempting to acquire the lock
    lock: AtomicBool,         // for spin lock
    turn: AtomicUsize,        /* thread index with priority for lock acquisition.
                               * turn increments sequentially.
                               * cycles based on the NUM_LOCK value.
                               */
    data: UnsafeCell<T>, // protected data
}

pub struct FairLockGuard<'a, T> {
    // RAII guard
    fair_lock: &'a FairLock<T>, // reference to the fairlock
    idx: usize,                 // thread index
}

impl<T> FairLock<T> {
    pub fn new(v: T) -> Self {
        let mut vec = Vec::new();
        for _ in 0..NUM_LOCK {
            vec.push(AtomicBool::new(false));
        }

        FairLock {
            waiting: vec,
            lock: AtomicBool::new(false),
            turn: AtomicUsize::new(0),
            data: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self, idx: usize) -> FairLockGuard<T> {
        assert!(idx < NUM_LOCK);

        // set the idx-th thread as waiting
        // Relaxed: no ordering constraints
        self.waiting[idx].store(true, Ordering::Relaxed);
        loop {
            // first, if another thread has yielded its priority to the current thread
            if !self.waiting[idx].load(Ordering::Relaxed) {
                break; // the lock is acquired
            }

            // second, if no one else holds the lock
            if !self.lock.load(Ordering::Relaxed) {
                if let Ok(_) = self.lock.compare_exchange_weak(
                    false, // expected value
                    true,  // new value
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                ) {
                    break; // the lock is acquired
                }
            }
        }
        // Acquire: ensure that the lock is acquired before the data is accessed
        fence(Ordering::Acquire);

        FairLockGuard {
            fair_lock: self,
            idx,
        }
    }
}

impl<'a, T> Drop for FairLockGuard<'a, T> {
    fn drop(&mut self) {
        let fl = self.fair_lock;

        // set the idx-th thread as not waiting
        fl.waiting[self.idx].store(false, Ordering::Relaxed);

        let turn = fl.turn.load(Ordering::Relaxed);
        let next = if turn == self.idx {
            // If a thread completes its work during its turn,
            // it yields the priority to the next in line.
            (turn + 1) & MASK
        } else {
            // If the thread with priority does not perform any work,
            // it maintains the existing priority.
            turn
        };

        if fl.waiting[next].load(Ordering::Relaxed) { 
            // if the next thread is waiting
            fl.turn.store(next, Ordering::Relaxed); // set the next thread as the priority
            fl.waiting[next].store(false, Ordering::Relaxed); // set the next thread as not waiting
        } else {
            // if the next thread is not waiting
            fl.turn.store((next + 1) & MASK, Ordering::Relaxed); // it moves to the following thread in line
            fl.lock.store(false, Ordering::Release); // and release the lock
        }
    }
}

// can be shared among threads
unsafe impl<T> Sync for FairLock<T> {}
unsafe impl<T> Send for FairLock<T> {}

impl<'a, T> Deref for FairLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.fair_lock.data.get() }
    }
}

impl<'a, T> DerefMut for FairLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.fair_lock.data.get() }
    }
}
