use session_types::{session_channel, Branch, Chan, Choose, Eps, HasDual, Recv, Send};
use std::thread;

type Client = Send<u64, Choose<Recv<u64, Eps>, Recv<bool, Eps>>>;
type Server = <Client as HasDual>::Dual;

enum Op {
    Square,
    Even,
}

fn server(c: Chan<(), Server>) {
    let (c, n) = c.recv();
    match c.offer() {
        Branch::Left(c) => {
            c.send(n * n).close();
        }
        Branch::Right(c) => {
            c.send(n & 1 == 0).close();
        }
    }
}

fn client(c: Chan<(), Client>, n: u64, op: Op) {
    let c = c.send(n);
    match op {
        Op::Square => {
            let c = c.sel1();
            let (c, val) = c.recv();
            c.close();
            println!("{}^2 = {}", n, val);
        }
        Op::Even => {
            let c = c.sel2();
            let (c, val) = c.recv();
            c.close();
            if val {
                println!("{} is even", n);
            } else {
                println!("{} is odd", n);
            }
        }
    };
}

fn main() {
    let (server_chan, client_chan) = session_channel();
    let srv_t = thread::spawn(move || server(server_chan));
    let cli_t = thread::spawn(move || client(client_chan, 11, Op::Even));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    let (server_chan, client_chan) = session_channel();
    let srv_t = thread::spawn(move || server(server_chan));
    let cli_t = thread::spawn(move || client(client_chan, 11, Op::Square));
    srv_t.join().unwrap();
    cli_t.join().unwrap();
}
