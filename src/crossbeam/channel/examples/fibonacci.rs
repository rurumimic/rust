use std::sync::mpsc;
use std::thread;

trait Sendable<T: Send> {
    fn send(&self, value: T) -> Result<(), Box<dyn std::error::Error>>;
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

    print_sequence(r1.iter(), 20);
    print_sequence(r2.iter(), 20);
}
