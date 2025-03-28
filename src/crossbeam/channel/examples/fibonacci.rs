use std::sync::{Arc, Mutex, mpsc};
use std::thread;

trait Sendable<T: 'static + Send> {
    fn send(&self, value: T) -> Result<(), Box<dyn std::error::Error>>;
}

trait Receivable<T: 'static + Send>: Clone + Send + Sync {
    fn recv(&self) -> Option<T>;

    //fn iter(&self) -> impl Iterator<Item = T> + '_ {
    //    std::iter::from_fn(move || Receivable::recv(&self))
    //}

    fn iter(&self) -> impl Iterator<Item = T> + '_;
}

impl<T: 'static + Send> Sendable<T> for mpsc::SyncSender<T> {
    fn send(&self, value: T) -> Result<(), Box<dyn std::error::Error>> {
        self.send(value)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

impl<T: 'static + Send> Sendable<T> for crossbeam_channel::Sender<T> {
    fn send(&self, value: T) -> Result<(), Box<dyn std::error::Error>> {
        self.send(value)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

impl<T: 'static + Send> Receivable<T> for Arc<Mutex<mpsc::Receiver<T>>> {
    fn recv(&self) -> Option<T> {
        self.lock().ok()?.recv().ok()
    }

    fn iter(&self) -> impl Iterator<Item = T> + '_ {
        let r = self.clone();
        std::iter::from_fn(move || Receivable::recv(&r))
    }
}

impl<T: 'static + Send> Receivable<T> for crossbeam_channel::Receiver<T> {
    fn recv(&self) -> Option<T> {
        self.recv().ok()
    }

    fn iter(&self) -> impl Iterator<Item = T> + '_ {
        let r = self.clone();
        std::iter::from_fn(move || Receivable::recv(&r))
    }
}

fn fibonacci<S: Sendable<u64>>(sender: S) {
    let mut x = 0;
    let mut y = 1;

    while sender.send(x).is_ok() {
        let tmp = x;
        x = y;
        y += tmp;
    }
}

fn print_sequence<I>(iter: I, limit: usize)
where
    I: Iterator<Item = u64>,
{
    for n in iter.take(limit) {
        print!("{} ", n);
    }
    println!();
}

fn main() {
    let (s1, r1) = mpsc::sync_channel::<u64>(0);
    let (s2, r2) = crossbeam_channel::bounded::<u64>(0);

    thread::spawn(|| fibonacci(s1));
    thread::spawn(|| fibonacci(s2));

    let r1 = Arc::new(Mutex::new(r1));

    let handles = vec![
        thread::spawn({
            let r1 = r1.clone();
            move || print_sequence(Receivable::iter(&r1), 10)
        }),
        thread::spawn({
            let r2 = r2.clone();
            move || print_sequence(Receivable::iter(&r2), 10)
        }),
    ];

    for handle in handles {
        handle.join().unwrap();
    }
}
