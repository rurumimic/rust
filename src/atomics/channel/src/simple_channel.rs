use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};
use std::thread;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel() {
        let c = Channel::new();

        thread::scope(|s| {
            s.spawn(|| {
                c.send(1);
                c.send(2);
                c.send(3);
            });

            s.spawn(|| {
                assert_eq!(c.receive(), 1);
                assert_eq!(c.receive(), 2);
                assert_eq!(c.receive(), 3);
            });
        });
    }
}
