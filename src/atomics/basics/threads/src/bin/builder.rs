use std::thread;

fn main() {
    let numbers = Vec::from_iter(0..=1000);

    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.into_iter().sum::<usize>();
        sum / len
    });

    let average = t.join().unwrap();
    println!("Average: {}", average); // Average: 500

    let numbers = Vec::from_iter(0..=1000);

    let t = thread::Builder::new()
        .name("thread2".to_string())
        .stack_size(32 * 1024)
        .spawn(move || {
            let len = numbers.len();
            let sum = numbers.into_iter().sum::<usize>();
            sum / len
        })
        .unwrap();

    let average = t.join().unwrap();
    println!("Average: {}", average); // Average: 500
}
