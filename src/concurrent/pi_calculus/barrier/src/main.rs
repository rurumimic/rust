use std::sync::mpsc::{channel, Sender};

fn main() {
    let mut v = Vec::new();

    let (tx, rx) = channel::<Sender<()>>();

    let barrier = move || {
        let x = rx.recv().unwrap();
        let y = rx.recv().unwrap();
        let z = rx.recv().unwrap();
        println!("barrier: send!");
        x.send(()).unwrap();
        y.send(()).unwrap();
        z.send(()).unwrap();
    };

    let t = std::thread::spawn(barrier);
    v.push(t);

    for i in 0..3 {
        let tx_clone = tx.clone();

        let node = move || {
            let (tx_node, rx_node) = channel();
            tx_clone.send(tx_node).unwrap();
            println!("node {}: send!", i);
            rx_node.recv().unwrap();
            println!("node {}: received!", i);
        };

        let t = std::thread::spawn(node);
        v.push(t);
    }

    for t in v {
        t.join().unwrap();
    }
}
