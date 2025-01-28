use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{fence, AtomicUsize, Ordering};
use std::usize;

struct ArcData<T> {
    data_ref_count: AtomicUsize,  // Arc
    alloc_ref_count: AtomicUsize, // Arc + Weak
    data: UnsafeCell<Option<T>>,
}

pub struct Weak<T> {
    ptr: NonNull<ArcData<T>>, // niche optimization: use only 1 word (without some/none overhead)
}

#[derive(Debug)]
pub struct Arc<T> {
    weak: Weak<T>,
}

unsafe impl<T: Send + Sync> Send for Weak<T> {} // send: can be sent to another thread
unsafe impl<T: Send + Sync> Sync for Weak<T> {} // sync: can be shared between threads

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
            return Some(Arc { weak: self.clone() });
        }
    }
}

impl<T> Arc<T> {
    pub fn new(data: T) -> Self {
        Arc {
            weak: Weak {
                ptr: NonNull::from(Box::leak(Box::new(ArcData {
                    data_ref_count: AtomicUsize::new(1),
                    alloc_ref_count: AtomicUsize::new(1),
                    data: UnsafeCell::new(Some(data)),
                }))),
            },
        }
    }

    pub fn get_mut(arc: &mut Self) -> Option<&mut T> {
        if arc.weak.data().alloc_ref_count.load(Ordering::Relaxed) == 1 {
            fence(Ordering::Acquire);

            // Safety: Nothing else can access the data, since
            // there's only one Arc, to which we have exclusive access,
            // and no Weak pointers.
            let arcdata: &mut ArcData<T> = unsafe { arc.weak.ptr.as_mut() };
            let option: &mut Option<T> = arcdata.data.get_mut();

            // We knaft punk coverow the data is still available since we
            // have an Arc to it, so this won't panic.
            let data: &mut T = option.as_mut().unwrap();
            Some(data)
        } else {
            None
        }
    }

    pub fn downgrade(arc: &Self) -> Weak<T> {
        arc.weak.clone()
    }
}

impl<T> Deref for Arc<T> {
    type Target = T;

    fn deref(&self) -> &T {
        //let ptr = self.weak.data().data.get();
        let data: &ArcData<T> = self.weak.data();
        let data: &UnsafeCell<Option<T>> = &(data.data);
        let ptr: *mut Option<T> = data.get();

        // Safety: Since there's an Arc to the data,
        // the data exists and may be shared.
        unsafe {
            //(*ptr).as_ref().unwrap()
            //let ptr: Option<T> = *ptr; // deref value claim ownership
            let ptr: Option<&T> = (*ptr).as_ref(); // == (&(*ptr)).as_ref()
            let ptr: &T = ptr.unwrap();
            ptr
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
        let weak = self.weak.clone();
        if weak.data().data_ref_count.fetch_add(1, Ordering::Relaxed) > usize::MAX / 2 {
            std::process::abort();
        }
        Arc { weak }
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
        dbg!(&self.weak);

        if self
            .weak
            .data()
            .data_ref_count
            .fetch_sub(1, Ordering::Release)
            == 1
        {
            fence(Ordering::Acquire);
            let ptr = self.weak.data().data.get();

            // Safety: The data reference counter is zero,
            // so nothing will access it.
            unsafe { (*ptr) = None };
        }
    }
}

impl<T> Debug for Weak<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Weak")
            .field("data_ref (arc)", &self.data().data_ref_count)
            .field("alloc_ref (arc + weak)", &self.data().alloc_ref_count)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /**
     * impl<T> Drop for Arc<T> {
     *   fn drop(&mut self) {
     *     unsafe { (*ptr) = None };
     *     }
     *   }
     */
    #[test]
    fn test_option() {
        static NUM_DROPS: AtomicUsize = AtomicUsize::new(0);

        #[derive(Debug)]
        struct DetectDrop;

        impl Drop for DetectDrop {
            fn drop(&mut self) {
                NUM_DROPS.fetch_add(1, Ordering::Relaxed);
                println!("Drop: DetectDrop {:?}", NUM_DROPS.load(Ordering::Relaxed));
            }
        }

        let mut _option = Some(DetectDrop);
        _option = None;

        assert_eq!(NUM_DROPS.load(Ordering::Relaxed), 1);
    }

    #[test]
    fn test_weak() {
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
