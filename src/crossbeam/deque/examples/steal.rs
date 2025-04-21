use std::thread;

use crossbeam_deque::{Injector, Steal, Worker};

fn main() {
    let queue = Injector::<i32>::new();
    let worker1 = Worker::<i32>::new_fifo();
    let worker2 = Worker::<i32>::new_fifo();
    let worker2_stealer = worker2.stealer();

    thread::scope(|s| {
        queue.push(1);
        worker1.push(2);
        worker1.push(3);
        worker2.push(4);

        s.spawn(move || {
            for _ in 1..=5 {
                let item = worker1.pop();
                match item {
                    Some(item) => {
                        println!("Worker 1 popped item: {:?}", item);
                    }
                    None => {
                        let item = queue.steal();
                        match item {
                            Steal::Success(item) => {
                                println!("Worker 1 stole item from global queue: {:?}", item);
                            }
                            _ => {
                                let item = worker2_stealer.steal();
                                match item {
                                    Steal::Success(item) => {
                                        println!("Worker 1 stole item from worker 2: {:?}", item);
                                    }
                                    _ => {
                                        println!("Worker 1 could not steal any items.");
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });
    });
}
