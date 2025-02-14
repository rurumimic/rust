use std::sync::atomic::{AtomicU32, Ordering};

use atomic_wait::{wait, wake_all, wake_one};

use crate::mutex::v2::MutexGuard;

pub struct Condvar {
    counter: AtomicU32,
}

impl Condvar {
    pub const fn new() -> Self {
        Self {
            counter: AtomicU32::new(0),
        }
    }

    pub fn notify_one(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
        wake_one(&self.counter);
    }

    pub fn notify_all(&self) {
        self.counter.fetch_add(1, Ordering::Relaxed);
        wake_all(&self.counter);
    }

    pub fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        let counter_value = self.counter.load(Ordering::Relaxed);

        let mutex = guard.mutex;
        drop(guard); // = wake_one() = unlock the mutex

        wait(&self.counter, counter_value);

        mutex.lock()
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use crate::mutex::v2::Mutex;

    use super::*;

    #[test]
    fn test() {
        let mutex = Mutex::new(0);
        let condvar = Condvar::new();

        let mut wakeups = 0;

        thread::scope(|s| {
            s.spawn(|| {
                // wait until the main thread is in wait()
                thread::sleep(Duration::from_secs(1));
                *mutex.lock() = 123;
                condvar.notify_one();
            });

            let mut m = mutex.lock(); // lock before the thread
            while *m < 100 {
                // prevent spurious wakeups
                m = condvar.wait(m);
                wakeups += 1;
            }

            println!("wakeups: {}", wakeups);
            assert_eq!(*m, 123);
        });

        assert!(wakeups < 10);
    }
}
