use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct UnsafeChannel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

impl<T> UnsafeChannel<T> {
    pub fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// Safety: Only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Ordering::Release);
    }

    /// Safety: Only call this once!
    /// and only if `is_ready` returns `true`
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_channel() {
        let channel = UnsafeChannel::new();
        assert!(!channel.is_ready());

        unsafe {
            channel.send(42);
            assert!(channel.is_ready());
            assert_eq!(channel.receive(), 42);
        }
    }
}
