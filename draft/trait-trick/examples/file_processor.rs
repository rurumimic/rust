use std::collections::BinaryHeap;
use std::marker::PhantomData;

fn main() {
    demo_file_processing();
    demo_network_requests();
    demo_priority_execution();
}

fn demo_file_processing() {
    println!("=== File Processing Demo ===");
    let mut processor = TaskProcessor::new();
    processor.enqueue(FileTask::new(1, "config.json")).unwrap();
    processor.enqueue(FileTask::new(2, "data.csv")).unwrap();
    processor.enqueue(FileTask::new(3, "log.txt")).unwrap();

    while let Ok(task) = processor.dequeue() {
        if let Err(error) = task.execute() {
            eprintln!("Task execution failed: {}", error);
        }
    }
    println!();
}

fn demo_network_requests() {
    println!("=== Network Request Demo ===");
    let mut processor = TaskProcessor::with_fifo_queue();
    processor
        .enqueue(HttpRequest::new("https://api.example.com/users"))
        .unwrap();
    processor
        .enqueue(HttpRequest::new("https://api.example.com/posts"))
        .unwrap();

    while let Ok(request) = processor.dequeue() {
        if let Err(error) = request.execute() {
            eprintln!("Request execution failed: {}", error);
        }
    }
    println!();
}

fn demo_priority_execution() {
    println!("=== Priority Execution Demo ===");
    let mut processor = TaskProcessor::with_priority_queue();
    processor
        .enqueue(FileTask::new(1, "low_priority.log"))
        .unwrap();
    processor
        .enqueue(FileTask::new(100, "critical_config.json"))
        .unwrap();
    processor
        .enqueue(FileTask::new(50, "user_data.csv"))
        .unwrap();

    while let Ok(task) = processor.dequeue() {
        if let Err(error) = task.execute() {
            eprintln!("Priority task execution failed: {}", error);
        }
    }
    println!();
}

/// A task processor that manages executable tasks using different queue strategies
struct TaskProcessor<S, T> {
    storage: S,
    _task_type: PhantomData<T>,
}

impl<S, T> TaskProcessor<S, T>
where
    S: TaskStorage<T>,
    T: Executable,
{
    fn enqueue(&mut self, task: T) -> Result<(), String> {
        self.storage.store(task)
    }

    fn dequeue(&mut self) -> Result<T, String> {
        self.storage.retrieve()
    }
}

impl<T: Executable> TaskProcessor<FifoQueue<T>, T> {
    fn new() -> Self {
        Self::with_fifo_queue()
    }

    fn with_fifo_queue() -> Self {
        let storage = FifoQueue::new();
        TaskProcessor {
            storage,
            _task_type: PhantomData,
        }
    }
}

impl<T: Executable + Ord> TaskProcessor<PriorityQueue<T>, T> {
    fn with_priority_queue() -> Self {
        let storage = PriorityQueue::new();
        TaskProcessor {
            storage,
            _task_type: PhantomData,
        }
    }
}

/// Trait for tasks that can be executed
trait Executable: Send + 'static {
    fn execute(&self) -> Result<(), String>;
}

/// Trait for storing tasks
trait Storable<T: Executable> {
    fn store(&mut self, task: T) -> Result<(), String>;
}

/// Trait for retrieving tasks
trait Retrievable<T: Executable> {
    fn retrieve(&mut self) -> Result<T, String>;
}

/// Combined trait for task storage systems
trait TaskStorage<T: Executable>: Storable<T> + Retrievable<T> {}

impl<S, T> TaskStorage<T> for S
where
    S: Storable<T> + Retrievable<T>,
    T: Executable,
{
}

/// A file processing task with priority
#[derive(Eq, PartialEq)]
struct FileTask {
    priority: u32,
    filename: String,
}

impl FileTask {
    fn new(priority: u32, filename: &str) -> Self {
        FileTask {
            priority,
            filename: filename.to_string(),
        }
    }
}

impl Executable for FileTask {
    fn execute(&self) -> Result<(), String> {
        println!(
            "Processing file '{}' (priority: {})",
            self.filename, self.priority
        );
        // Simulate file processing
        Ok(())
    }
}

impl Ord for FileTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for FileTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// An HTTP request task
struct HttpRequest {
    url: String,
}

impl HttpRequest {
    fn new(url: &str) -> Self {
        HttpRequest {
            url: url.to_string(),
        }
    }
}

impl Executable for HttpRequest {
    fn execute(&self) -> Result<(), String> {
        println!("Sending HTTP request to: {}", self.url);
        // Simulate HTTP request
        Ok(())
    }
}

/// FIFO (First-In-First-Out) queue implementation
struct FifoQueue<T: Executable> {
    buffer: Vec<T>,
}

impl<T: Executable> FifoQueue<T> {
    fn new() -> Self {
        FifoQueue { buffer: Vec::new() }
    }
}

impl<T: Executable> Storable<T> for FifoQueue<T> {
    fn store(&mut self, task: T) -> Result<(), String> {
        self.buffer.push(task);
        Ok(())
    }
}

impl<T: Executable> Retrievable<T> for FifoQueue<T> {
    fn retrieve(&mut self) -> Result<T, String> {
        if let Some(task) = self.buffer.pop() {
            Ok(task)
        } else {
            Err("FIFO queue is empty".to_string())
        }
    }
}

/// Priority queue implementation using binary heap
struct PriorityQueue<T: Executable + Ord> {
    heap: BinaryHeap<T>,
}

impl<T: Executable + Ord> PriorityQueue<T> {
    fn new() -> Self {
        PriorityQueue {
            heap: BinaryHeap::new(),
        }
    }
}

impl<T: Executable + Ord> Storable<T> for PriorityQueue<T> {
    fn store(&mut self, task: T) -> Result<(), String> {
        self.heap.push(task);
        Ok(())
    }
}

impl<T: Executable + Ord> Retrievable<T> for PriorityQueue<T> {
    fn retrieve(&mut self) -> Result<T, String> {
        if let Some(task) = self.heap.pop() {
            Ok(task)
        } else {
            Err("Priority queue is empty".to_string())
        }
    }
}
