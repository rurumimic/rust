use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicU8, Ordering};

const EMPTY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

pub struct AtomicChannel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T> Sync for AtomicChannel<T> where T: Send {}

impl<T> AtomicChannel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMPTY),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Relaxed) == READY
    }

    /// Safety: Only call this once!
    pub fn send(&self, message: T) {
        if self
            .state
            .compare_exchange(EMPTY, WRITING, Ordering::Relaxed, Ordering::Relaxed)
            .is_err()
        {
            panic!("Cannot send more than one message");
        }
        unsafe { (*self.message.get()).write(message) };
        self.state.store(READY, Ordering::Release);
    }

    /// After `is_ready` returns `true`
    pub fn receive(&self) -> T {
        if self
            .state
            .compare_exchange(READY, READING, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            panic!("Channel is not ready");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for AtomicChannel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_check_channel() {
        let channel = AtomicChannel::new();
        let t = thread::current();

        thread::scope(|s| {
            s.spawn(|| {
                channel.send("Hello, World!");
                // channel.send("Hello, World!"); // panic
                t.unpark();
            });

            while !channel.is_ready() {
                thread::park();
            }

            assert_eq!(channel.receive(), "Hello, World!");
        });
    }
}
