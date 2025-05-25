use deque::with_channel::Dispatcher;

fn main() {
    let mut dispatcher = Dispatcher::new(3);

    dispatcher.run(0, || println!("Task 0 executed"));
    dispatcher.run(1, || println!("Task 1 executed"));
    dispatcher.run(2, || println!("Task 2 executed"));
    dispatcher.run(3, || println!("Task 3 executed"));
    dispatcher.run(3, || println!("Task 4 executed"));
    dispatcher.run(3, || println!("Task 5 executed"));
    dispatcher.run(3, || println!("Task 6 executed"));
    dispatcher.run(3, || println!("Task 7 executed"));

    std::thread::sleep(std::time::Duration::from_millis(100));

    dispatcher.stop();
}
