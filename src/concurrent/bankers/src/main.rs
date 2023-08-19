mod banker;

use banker::Banker;
use std::{thread, vec};

const NUM_LOOP: usize = 100000;

fn main() {
    let b = Banker::<2, 2>::new([1, 1], [[1, 1], [1, 1]]);

    let mut philosophers = vec![];

    for i in 0..=1 {
        let banker = b.clone();

        let p = thread::spawn(move || {
            for _ in 0..NUM_LOOP {
                while !banker.take(i, 0) {}
                while !banker.take(i, 1) {}

                print!("{}", i);

                banker.release(i, 0);
                banker.release(i, 1);
            }
        });

        philosophers.push(p);
    }

    for p in philosophers {
        p.join().unwrap();
    }

    println!("\nFinished");
}
