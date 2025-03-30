use std::io;
use std::thread;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, bounded, select, tick};
use signal_hook::consts::SIGINT;
use signal_hook::iterator::Signals;

fn sigint_notifier() -> io::Result<Receiver<()>> {
    let (s, r) = bounded(100);
    let mut signals = Signals::new([SIGINT])?;

    thread::spawn(move || {
        for _ in signals.forever() {
            if s.send(()).is_err() {
                break;
            }
        }
    });

    Ok(r)
}

fn show(dur: Duration) {
    println!("Elapsed: {}.{:03} sec", dur.as_secs(), dur.subsec_millis());
}

fn main() {
    let start = Instant::now();
    let update = tick(Duration::from_secs(1));
    let ctrl_c = sigint_notifier().unwrap();

    loop {
        select! {
            recv(update) -> _ => {
                show(start.elapsed());
            }
            recv(ctrl_c) -> _ => {
                println!();
                println!("Goodbye");
                show(start.elapsed());
                break;
            }
        }
    }
}
