use std::{error::Error, process, thread, time::Duration};
use signal_hook::{iterator::Signals, consts::SIGUSR1};

fn main() -> Result<(), Box<dyn Error>> {
    println!("kill -s SIGUSR1 {}", process::id());

    let mut signals = Signals::new(&[SIGUSR1])?;
    thread::spawn(move || {
        for sig in signals.forever() {
            println!("recived signal: {:?}", sig); // sig = 30
        }
    });


    thread::sleep(Duration::from_secs(10));
    Ok(())
}
