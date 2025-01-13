use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut q = self.queue.lock().unwrap();
        loop {
            if let Some(message) = q.pop_front() {
                return message;
            }

            q = self.ready.wait(q).unwrap();
        }
    }
}
