use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{io, thread};

fn some_work() {
    thread::sleep(Duration::from_secs(1));
    println!("."); // auto-flush stdout
}

fn main() {
    // static STOP: AtomicBool = AtomicBool::new(false);
    let stop = Arc::new(AtomicBool::new(false));

    let background_thread = thread::spawn({
        let stop = Arc::clone(&stop);
        move || {
            while !stop.load(Ordering::Relaxed) {
                some_work();
            }
        }
    });

    for line in io::stdin().lines() {
        match line {
            Ok(line) => match line.as_str() {
                "help" => println!("commands: help, stop"),
                "stop" => break,
                cmd => println!("unknown command: {cmd:?}"),
            },
            Err(e) => {
                eprintln!("error reading line: {e}");
                break;
            }
        }
    }

    stop.store(true, Ordering::Relaxed);

    match background_thread.join() {
        Ok(_) => println!("background thread stopped"),
        Err(e) => {
            if let Some(msg) = e.downcast_ref::<&str>() {
                eprintln!("background thread panicked: {msg}");
            } else {
                eprintln!("background thread errored: {:?}", e.type_id());
            }
        }
    }
}
