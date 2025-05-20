use deque::Dispatcher;

fn main() {
    let mut dispatcher = Dispatcher::new(3);

    dispatcher.run(0, || {
        println!("Task 0 executed");
    });

    dispatcher.stop();
}
