use std::thread;

static STATIC_NUMBERS: [i32; 3] = [1, 2, 3];

fn main() {
    let mut threads = vec![];

    threads.push(thread::spawn(|| dbg!(&STATIC_NUMBERS)));
    threads.push(thread::spawn(|| dbg!(&STATIC_NUMBERS)));

    for t in threads {
        t.join().unwrap();
    }
}
