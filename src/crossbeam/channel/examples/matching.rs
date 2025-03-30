use crossbeam_channel::{bounded, select};
use std::thread;

fn main() {
    let people = vec!["Alice", "Bob", "Cody", "Dave", "Eva"];
    let (s, r) = bounded(1);

    // Either send my name into the channel or receive someone else's, whatever happens first.
    let seek = |name, s: crossbeam_channel::Sender<&str>, r: crossbeam_channel::Receiver<&str>| {
        select! {
            recv(r) -> peer => println!("{} received a message from {}.", name, peer.unwrap()),
            send(s, name) -> _ => {}, // Wait for someone to receive my message.
        }
    };

    thread::scope(|scope| {
        for name in people {
            let (s, r) = (s.clone(), r.clone());
            scope.spawn(move || seek(name, s, r));
        }
    });

    if let Ok(name) = r.try_recv() {
        println!("No one received {}'s message.", name);
    }
}
