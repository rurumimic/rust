use std::thread;

use channel::channel::Channel;

fn main() {
    let c = Channel::new();

    thread::scope(|s| {
        s.spawn(|| {
            c.send(1);
            c.send(2);
            c.send(3);
        });

        s.spawn(|| {
            println!("{}", c.receive());
            println!("{}", c.receive());
            println!("{}", c.receive());
        });
    });
}
