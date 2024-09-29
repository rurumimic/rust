use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        println!("Main thread id: {:?}", thread::current().id());

        s.spawn(|| {
            println!("lenght: {}", numbers.len());
        });

        s.spawn(|| {
            for n in &numbers {
                print!("{} ", n);
            }
            println!("");
        });
    });

    println!("End.");
}
