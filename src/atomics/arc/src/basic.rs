use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{fence, AtomicUsize, Ordering};

struct ArcData<T> {
    ref_count: AtomicUsize,
    data: T,
}

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>, // niche optimization: use only 1 word (without some/none overhead)
}

unsafe impl<T: Send + Sync> Send for Arc<T> {} // send: can be sent to another thread
unsafe impl<T: Send + Sync> Sync for Arc<T> {} // sync: can be shared between threads

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                ref_count: AtomicUsize::new(1),
                data,
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.data().ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);

            unsafe { Some(&mut arc.ptr.as_mut().data) }
        } else {
            None
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data().data
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        if self.data().ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
            }
        }

        let x = Arc::new(("hello", DetectDrop));
        let y = x.clone();

        let t = std::thread::spawn(move || {
            assert_eq!(x.0, "hello");
        });

        assert_eq!(y.0, "hello");

        t.join().unwrap();

        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);

        drop(y);

        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_mut() {
        let mut x = Arc::new(42);
        if let Some(v) = Arc::get_mut(&mut x) {
            *v = 27;
        } else {
            assert_eq!(*x, 42);
        }

        let y = x.clone();

        assert!(Arc::get_mut(&mut x).is_none());

        drop(y);

        if let Some(v) = Arc::get_mut(&mut x) {
            *v = 38;
        } else {
            assert_eq!(*x, 27);
        }

        assert_eq!(*x, 38);
    }
}
