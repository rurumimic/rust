use session_types::{
    offer, session_channel, Branch, Chan, Eps, HasDual, Offer, Rec, Recv, Send, Var, Z,
};
use std::collections::HashMap;
use std::thread;

type Put = Recv<u64, Recv<u64, Var<Z>>>;
type Get = Recv<u64, Send<Option<u64>, Var<Z>>>;

type Server = Rec<Offer<Put, Offer<Get, Eps>>>;
type Client = <Server as HasDual>::Dual;

fn server(c: Chan<(), Server>) {
    let mut c_enter = c.enter();
    let mut db = HashMap::new();

    loop {
        match c_enter.offer() {
            Branch::Left(c) => {
                let (c, key) = c.recv();
                let (c, val) = c.recv();
                db.insert(key, val);
                c_enter = c.zero();
            }
            Branch::Right(c) => match c.offer() {
                Branch::Left(c) => {
                    let (c, key) = c.recv();
                    let c = if let Some(val) = db.get(&key) {
                        c.send(Some(*val))
                    } else {
                        c.send(None)
                    };
                    c_enter = c.zero();
                }
                Branch::Right(c) => {
                    c.close();
                    return;
                }
            },
        }
    }
}

fn client(c: Chan<(), Client>) {
    let c = c.enter();

    let c = c.sel1().send(10).send(4).zero();
    let c = c.sel1().send(50).send(7).zero();

    let (c, val) = c.sel2().sel1().send(10).recv();
    println!("val = {:?}", val);

    let c = c.zero();

    let (c, val) = c.sel2().sel1().send(20).recv();
    println!("val = {:?}", val);

    let _ = c.zero().sel2().sel2().close();
}

fn server_macro(c: Chan<(), Server>) {
    let mut c_enter = c.enter();
    let mut db = HashMap::new();

    loop {
        let c = c_enter;
        offer! {
            c,
            Put => {
                let (c, key) = c.recv();
                let (c, val) = c.recv();
                db.insert(key, val);
                c_enter = c.zero();
            },
            Get => {
                let (c, key) = c.recv();
                let c = if let Some(val) = db.get(&key) {
                    c.send(Some(*val))
                } else {
                    c.send(None)
                };
                c_enter = c.zero();
            },
            Quit => {
                c.close();
                return;
            }
        }
    }
}

type SChan = Chan<(), Send<(), Eps>>;
type ChanRecv = Recv<SChan, Eps>;
type ChanSend = <ChanRecv as HasDual>::Dual;

fn chan_recv(c: Chan<(), ChanRecv>) {
    let (c, cr) = c.recv();
    c.close();
    let cr = cr.send(());
    cr.close();
}

fn chan_send(c: Chan<(), ChanSend>) {
    let (c1, c2) = session_channel();
    let c = c.send(c1);
    c.close();
    let (c2, _) = c2.recv();
    c2.close();
}

fn main() {
    let (server_chan, client_chan) = session_channel();
    let srv_t = thread::spawn(move || server(server_chan));
    let cli_t = thread::spawn(move || client(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    println!("------");

    let (server_chan, client_chan) = session_channel();
    let srv_t = thread::spawn(move || server_macro(server_chan));
    let cli_t = thread::spawn(move || client(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();

    println!("------");

    let (server_chan, client_chan) = session_channel();
    let srv_t = thread::spawn(move || chan_recv(server_chan));
    let cli_t = thread::spawn(move || chan_send(client_chan));
    srv_t.join().unwrap();
    cli_t.join().unwrap();
}
