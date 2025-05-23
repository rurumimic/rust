use crossbeam_deque::{Injector, Steal, Stealer, Worker};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::JoinHandle;

pub struct Task {
    pub kind: usize,
    pub function: Box<dyn FnOnce() + Send + 'static>,
}

impl Task {
    pub fn run(self) {
        (self.function)();
    }
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

    pub fn pop(&self) -> Option<Task> {
        match self.queue.steal() {
            Steal::Empty => None,
            Steal::Retry => None,
            Steal::Success(task) => Some(task),
        }
    }
}

pub struct Dispatcher {
    global: Arc<GlobalQueue>,
    runners: Vec<TaskRunner>,
    senders: Vec<Sender<Task>>,
}

impl Dispatcher {
    pub fn new(size: usize) -> Self {
        let global = Arc::new(GlobalQueue::new());

        let mut senders = Vec::with_capacity(size);
        let mut receivers = Vec::with_capacity(size);
        let mut workers = Vec::with_capacity(size);
        let mut stealers = Vec::with_capacity(size);
        let mut runners = Vec::with_capacity(size);

        for _ in 0..size {
            let (sender, receiver) = std::sync::mpsc::channel();
            let worker = Worker::<Task>::new_fifo();
            senders.push(sender);
            receivers.push(receiver);
            stealers.push(worker.stealer());
            workers.push(worker);
        }

        for i in 0..size {
            let kind = i + 1;
            let receiver = receivers.remove(0);
            let worker = workers.remove(0);
            let my_stealers = Self::exclude(i, &stealers);

            let runner = TaskRunner::new(kind, receiver, worker, Arc::clone(&global), my_stealers);
            runners.push(runner);
        }

        Dispatcher {
            global,
            runners,
            senders,
        }
    }

    fn exclude(index: usize, stealers: &[Stealer<Task>]) -> Vec<Stealer<Task>> {
        let (left, right) = stealers.split_at(index);
        left.iter().chain(&right[1..]).cloned().collect()
    }

    pub fn run(&self, kind: usize, function: impl FnOnce() + Send + 'static) {
        if kind > self.runners.len() + 1 {
            panic!("kind {} > runners {}", kind, self.runners.len());
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
                println!("Task dispatcher sending task to runner {}", n);
                let _ = self.senders[n - 1].send(task);
            }
        };
    }

    pub fn stop(&mut self) {
        for runner in &mut self.runners {
            runner.join();
        }
    }
}

pub struct TaskRunner {
    kind: usize,
    thread: Option<JoinHandle<()>>,
    signal_sender: Sender<TaskRunnerState>,
}

enum TaskRunnerState {
    Run,
    Stop,
}

impl TaskRunner {
    pub fn new(
        kind: usize,
        receiver: Receiver<Task>,
        worker: Worker<Task>,
        global: Arc<GlobalQueue>,
        stealers: Vec<Stealer<Task>>,
    ) -> Self {
        let (signal_sender, signal_receiver) = std::sync::mpsc::channel();

        let thread = std::thread::spawn(move || {
            loop {
                std::thread::sleep(std::time::Duration::from_millis(10));

                match signal_receiver.try_recv() {
                    Ok(TaskRunnerState::Stop) => {
                        println!("Task runner {} stopping from signal", kind);
                        break;
                    }
                    _ => {}
                }

                while let Ok(task) = receiver.try_recv() {
                    println!("Task runner {} received task from receiver", kind);
                    worker.push(task);
                }

                match worker.pop() {
                    Some(task) => {
                        println!("Task runner {} executing task", kind);
                        task.run();
                        continue;
                    }
                    None => {}
                }

                match global.pop() {
                    Some(task) => {
                        println!("Task runner {} executing task from global queue", kind);
                        task.run();
                        continue;
                    }
                    None => {}
                }

                for (i, stealer) in stealers.iter().enumerate() {
                    match stealer.steal() {
                        Steal::Empty => {}
                        Steal::Retry => {}
                        Steal::Success(task) => {
                            println!(
                                "Task runner {} executing task from stealer {}",
                                kind,
                                i + kind
                            );
                            task.run();
                        }
                    }
                }
            }
        });

        TaskRunner {
            kind,
            thread: Some(thread),
            signal_sender,
        }
    }

    fn join(&mut self) {
        self.signal_sender.send(TaskRunnerState::Stop).unwrap();

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
