use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    println!("Main thread id: {:?}", thread::current().id());

    thread::scope(|s| {
        println!("Main thread id: {:?}", thread::current().id());

        s.spawn(|| {
            println!("Thread id: {:?}", thread::current().id());
            println!("lenght: {}", numbers.len());
        });

        s.spawn(|| {
            println!("Thread id: {:?}", thread::current().id());
            for n in &numbers {
                print!("{} ", n);
            }
            println!("");
        });
    });
}
