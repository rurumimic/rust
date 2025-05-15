use crossbeam_deque::{Injector, Steal, Stealer, Worker};
use std::sync::Arc;

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

        let mut runners = Vec::with_capacity(size);

        for (i, worker) in workers.iter().enumerate() {
            let (left, right) = workers.split_at(i);
            let others: Vec<Stealer<Task>> = left
                .iter()
                .chain(&right[1..])
                .map(|w| w.clone().stealer())
                .collect();

            let kind = i + 1;
            let runner = TaskRunner::new(kind, worker.clone(), global.clone(), others);
            runners.push(runner);
        }

        Dispatcher {
            global,
            workers,
            runners,
        }
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
}

pub struct TaskRunner {
    kind: usize,
    queue: Arc<WorkerQueue>,
    global: Arc<GlobalQueue>,
    others: Vec<Stealer<Task>>,
}

impl TaskRunner {
    pub fn new(
        kind: usize,
        queue: Arc<WorkerQueue>,
        global: Arc<GlobalQueue>,
        others: Vec<Stealer<Task>>,
    ) -> Self {
        TaskRunner {
            kind,
            queue,
            global,
            others,
        }
    }
}
