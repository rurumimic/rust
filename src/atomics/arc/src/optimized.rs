use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::mem::ManuallyDrop;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{fence, AtomicUsize, Ordering};
use std::usize;

struct ArcData<T> {
    data_ref_count: AtomicUsize,       // Arc
    alloc_ref_count: AtomicUsize,      // Weak. plus one if  there are any Arc
    data: UnsafeCell<ManuallyDrop<T>>, // Dropped if there are only weak pointers left
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>, // niche optimization: use only 1 word (without some/none overhead)
}

unsafe impl<T: Send + Sync> Send for Weak<T> {} // send: can be sent to another thread
unsafe impl<T: Send + Sync> Sync for Weak<T> {} // sync: can be shared between threads

pub struct Arc<T> {
    ptr: NonNull<ArcData<T>>,
}

unsafe impl<T: Send + Sync> Send for Arc<T> {} // send: can be sent to another thread
unsafe impl<T: Send + Sync> Sync for Arc<T> {} // sync: can be shared between threads

impl<T> Weak<T> {
    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() } // NonNull<T>.as_ref() = &T
                                     // NonNull<T> -> unsafe {*mut T -> *const T} -> &T
    }

    pub fn upgrade(&self) -> Option<Arc<T>> {
        let mut n = self.data().data_ref_count.load(Ordering::Relaxed);
        loop {
            if n == 0 {
                return None;
            }
            assert!(n < usize::MAX);
            if let Err(e) = self.data().data_ref_count.compare_exchange_weak(
                n,
                n + 1,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                n = e;
                continue;
            }
            return Some(Arc { ptr: self.ptr });
        }
    }
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Arc {
            ptr: NonNull::from(Box::leak(Box::new(ArcData {
                data_ref_count: AtomicUsize::new(1),
                alloc_ref_count: AtomicUsize::new(1),
                data: UnsafeCell::new(ManuallyDrop::new(data)),
            }))),
        }
    }

    fn data(&self) -> &ArcData<T> {
        unsafe { self.ptr.as_ref() }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        // Acquire -> Weak::drop#fetch_sub(1, Release)
        // to make sure any upgraded pointers are visible in the next data_ref_count.load
        // == lock weak pointer
        if arc
            .data()
            .alloc_ref_count
            .compare_exchange(1, usize::MAX, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return None;
        }

        // last arc
        let is_unique = arc.data().data_ref_count.load(Ordering::Relaxed) == 1;

        // == unlock weak pointer
        // Release -> Arc::downgrade#compare_exchange_weak(n, n+1, Acquire, Relaxed)
        // to make sure any changes to the data_ref_count
        // that come after `downgrade` don't change the is_unique result above
        arc.data().alloc_ref_count.store(1, Ordering::Release);

        if !is_unique {
            return None;
        }

        // Acquire -> Arc::drop#fetch_sub(1, Release)
        fence(Ordering::Acquire);

        unsafe { Some(&mut *arc.data().data.get()) }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        let mut n = arc.data().alloc_ref_count.load(Ordering::Relaxed);
        loop {
            if n == usize::MAX {
                std::hint::spin_loop();
                n = arc.data().alloc_ref_count.load(Ordering::Relaxed);
                continue;
            }

            assert!(n < usize::MAX - 1);

            // Arc::get_mut#store(1, Release)
            if let Err(e) = arc.data().alloc_ref_count.compare_exchange_weak(
                n,
                n + 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                n = e;
                continue;
            }

            return Weak { ptr: arc.ptr };
        }
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe {
            //&*self.data().data.get()
            let arcdata: &ArcData<T> = self.data();
            let data: &UnsafeCell<ManuallyDrop<T>> = &arcdata.data;
            let data: *mut ManuallyDrop<T> = data.get();
            &(*data)
        }
    }
}

impl<T> Clone for Weak<T> {
    fn clone(&self) -> Self {
        if self.data().alloc_ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Weak { ptr: self.ptr }
    }
}

impl<T> Clone for Arc<T> {
    fn clone(&self) -> Self {
        if self.data().data_ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { ptr: self.ptr }
    }
}

impl<T> Drop for Weak<T> {
    fn drop(&mut self) {
        println!("Weak drop");
        dbg!(&self);

        if self.data().alloc_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            println!("Weak remove: {:?}", self.ptr);
            fence(Ordering::Acquire);
            unsafe {
                drop(Box::from_raw(self.ptr.as_ptr()));
            }
        }
    }
}

impl<T> Drop for Arc<T> {
    fn drop(&mut self) {
        println!("Arc drop");
        dbg!(&self.ptr);

        if self.data().data_ref_count.fetch_sub(1, Ordering::Release) == 1 {
            fence(Ordering::Acquire);

            // Safety: data reference = 0
            // nothing will access the data
            unsafe { ManuallyDrop::drop(&mut *self.data().data.get()) };

            // no Arc<T> left
            // drop implicit weak pointer that represents all `Arc<T>`s
            drop(Weak { ptr: self.ptr });
        }
    }
}

impl<T> Debug for Weak<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Weak")
            .field("data_ref (arc)", &self.data().data_ref_count)
            .field("alloc_ref (weak + (1))", &self.data().alloc_ref_count)
            .finish()
    }
}

impl<T> Debug for Arc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Arc")
            .field("data_ref (arc)", &self.data().data_ref_count)
            .field("alloc_ref (weak + (1))", &self.data().alloc_ref_count)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("=== Test Start ===");

        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        println!("Num drops: {:?}", NUM_DROPS.load(Ordering::Relaxed));

        #[derive(Debug)]
        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
                println!("Drop: DetectDrop {:?}", NUM_DROPS.load(Ordering::Relaxed));
            }
        }

        println!("new arc x");
        let x = Arc::new(("hello", DetectDrop));
        dbg!(&x);

        println!("new weak x->y");
        let y = Arc::downgrade(&x);
        dbg!(&y);

        println!("new weak x->z");
        let z = Arc::downgrade(&x);
        dbg!(&z);

        let t = std::thread::spawn(move || {
            println!("weak y to new thread");
            dbg!(&y);

            println!("new arc w<-y in new thread");
            let w = y.upgrade().unwrap();
            dbg!(&y);
            assert_eq!(w.0, "hello");
        });

        println!("main thread");
        assert_eq!(x.0, "hello");
        dbg!(&z);
        t.join().unwrap();
        println!("thread end");
        dbg!(&x);
        dbg!(&z);

        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 0);

        println!("new arc <- z");
        assert!(z.upgrade().is_some());
        dbg!(&z);

        println!("drop x");
        drop(x);
        println!("---");

        {
            assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
            println!("new arc <- z");
            assert!(z.upgrade().is_none());
            dbg!(&z);
        }

        println!("new arc <- z");
        assert!(z.upgrade().is_none());
        dbg!(&z);
        println!("=== Test End ===");
    }
}
