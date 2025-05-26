use crossbeam_deque::{Injector, Steal, Stealer, Worker};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::thread::JoinHandle;

pub struct Task {
    pub kind: InjectorKind,
    pub function: Box<dyn FnOnce() + Send + 'static>,
}

impl Task {
    pub fn run(self) {
        (self.function)();
    }
}

pub struct Dispatcher {
    global: Arc<Injector<Task>>,
    injectors: Vec<Arc<Injector<Task>>>,
    runners: Vec<TaskRunner>,
}

pub enum InjectorKind {
    Global,
    Worker(usize),
}

impl Dispatcher {
    pub fn new(size: usize) -> Self {
        if size == 0 {
            panic!("Dispatcher size must be greater than 0");
        }

        let global = Arc::new(Injector::new());

        let injectors = (0..size)
            .map(|_| Arc::new(Injector::new()))
            .collect::<Vec<_>>();

        let mut runners = Vec::with_capacity(size);

        for i in 0..size {
            let my_stealers = Self::exclude(i, &injectors);
            let runner = TaskRunner::new(
                InjectorKind::Worker(i),
                Arc::clone(&injectors[i]),
                Arc::clone(&global),
                my_stealers,
            );

            runners.push(runner);
        }

        Dispatcher {
            global,
            injectors,
            runners,
        }
    }

    fn exclude(index: usize, injectors: &[Arc<Injector<Task>>]) -> Vec<Arc<Injector<Task>>> {
        let (left, right) = injectors.split_at(index);
        left.iter().chain(&right[1..]).cloned().collect()
    }

    pub fn run(&self, kind: InjectorKind, function: impl FnOnce() + Send + 'static) {
        match kind {
            InjectorKind::Global => self.global.push(Task {
                kind,
                function: Box::new(function),
            }),
            InjectorKind::Worker(n) => {
                if n > self.runners.len() {
                    panic!("kind {} > runners {}", n, self.runners.len());
                }
                println!("Task dispatcher sending task to runner {}", n);
                self.injectors[n].push(Task {
                    kind,
                    function: Box::new(function),
                });
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
    sender: Sender<TaskRunnerState>,
}

pub enum TaskRunnerState {
    Run,
    Stop,
}

impl TaskRunner {
    pub fn new(
        kind: InjectorKind,
        injector: Arc<Injector<Task>>,
        global: Arc<Injector<Task>>,
        stealers: Vec<Arc<Injector<Task>>>,
    ) -> Self {
        let (sender, receiver) = std::sync::mpsc::channel();

        let kind = match kind {
            InjectorKind::Global => 0,
            InjectorKind::Worker(n) => n,
        };

        let thread = std::thread::spawn(move || {
            loop {
                //std::thread::sleep(std::time::Duration::from_millis(10));

                match receiver.try_recv() {
                    Ok(TaskRunnerState::Stop) => {
                        println!("Task runner {} stopping from signal", kind);
                        break;
                    }
                    _ => {}
                }

                match injector.steal() {
                    Steal::Empty => {}
                    Steal::Retry => {}
                    Steal::Success(task) => {
                        println!("Task runner {} executing task from self", kind);
                        task.run();
                        continue;
                    }
                }

                match global.steal() {
                    Steal::Empty => {}
                    Steal::Retry => {}
                    Steal::Success(task) => {
                        println!("Task runner {} executing task from global", kind);
                        task.run();
                        continue;
                    }
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
            sender,
        }
    }

    pub fn stop(&self) {
        self.sender.send(TaskRunnerState::Stop).unwrap();
    }

    fn join(&mut self) {
        self.stop();

        if let Some(thread) = self.thread.take() {
            thread.join().unwrap();
        }
    }
}
