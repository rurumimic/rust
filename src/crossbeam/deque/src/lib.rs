use crossbeam_deque::{Injector, Steal, Stealer, Worker};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::JoinHandle;

pub struct Task {
    pub kind: usize,
    pub function: Box<dyn FnOnce() + Send + 'static>,
}

pub struct GlobalQueue {
    queue: Injector<Task>,
}

impl GlobalQueue {
    pub fn new() -> Self {
        GlobalQueue {
            queue: Injector::new(),
        }
    }

    pub fn push(&self, task: Task) {
        self.queue.push(task);
    }
}

pub struct WorkerQueue {
    queue: Worker<Task>,
}

impl WorkerQueue {
    pub fn new() -> Self {
        WorkerQueue {
            queue: Worker::new_fifo(),
        }
    }

    pub fn push(&self, task: Task) {
        self.queue.push(task);
    }

    pub fn stealer(&self) -> Stealer<Task> {
        self.queue.stealer()
    }
}

pub struct Dispatcher {
    global: Arc<GlobalQueue>,
    workers: Vec<Arc<WorkerQueue>>,
    runners: Vec<TaskRunner>,
}

impl Dispatcher {
    pub fn new(size: usize) -> Self {
        let global = Arc::new(GlobalQueue::new());
        let workers: Vec<Arc<WorkerQueue>> =
            std::iter::repeat_with(|| Arc::new(WorkerQueue::new()))
                .take(size)
                .collect();

        let runners = workers
            .iter()
            .enumerate()
            .map(|(i, worker)| {
                let others = Self::exclude(&workers, i);
                let kind = i + 1;
                TaskRunner::new(kind, Arc::clone(worker), Arc::clone(&global), others)
            })
            .collect();

        Dispatcher {
            global,
            workers,
            runners,
        }
    }

    fn exclude(workers: &[Arc<WorkerQueue>], index: usize) -> Vec<Stealer<Task>> {
        let (left, right) = workers.split_at(index);
        let others: Vec<Stealer<Task>> = left
            .iter()
            .chain(&right[1..])
            .map(|w| w.stealer())
            .collect();
        others
    }

    pub fn run(&self, kind: usize, function: impl FnOnce() + Send + 'static) {
        if kind > self.workers.len() {
            panic!("kind {} > workers {}", kind, self.workers.len());
        }

        let task = Task {
            kind,
            function: Box::new(function),
        };

        match kind {
            0 => {
                self.global.push(task);
            }
            n => {
                self.workers[n].push(task);
            }
        };
    }

    pub fn stop(&mut self) {
        for runner in &mut self.runners {
            runner.stop();
            runner.join();
        }
    }
}

pub struct TaskRunner {
    kind: usize,
    queue: Arc<WorkerQueue>,
    global: Arc<GlobalQueue>,
    others: Vec<Stealer<Task>>,
    thread: Option<JoinHandle<()>>,
    signal: Sender<TaskRunnerState>,
}

enum TaskRunnerState {
    Run,
    Stop,
}

impl TaskRunner {
    pub fn new(
        kind: usize,
        queue: Arc<WorkerQueue>,
        global: Arc<GlobalQueue>,
        others: Vec<Stealer<Task>>,
    ) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        let thread = std::thread::spawn(move || {
            loop {
                let receiver = receiver.recv();

                match receiver {
                    Ok(TaskRunnerState::Stop) => {
                        println!("Stopping task runner {}", kind);
                        break;
                    }
                    Ok(_) => {}
                    Err(_) => {
                        break;
                    }
                }

                //let task = &queue.queue.pop();
                //match task {
                //    Some(task) => {
                //        println!("Task runner {} executing task", kind);
                //        (task.function)();
                //    }
                //    None => {}
                //}
            }
        });

        TaskRunner {
            kind,
            queue,
            global,
            others,
            thread: Some(thread),
            signal: sender,
        }
    }

    fn stop(&self) {
        self.signal
            .send(TaskRunnerState::Stop)
            .expect("Failed to send stop signal");
    }

    fn join(&mut self) {
        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
