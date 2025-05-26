use deque::with_injector::Dispatcher;
use deque::with_injector::InjectorKind;

fn main() {
    let mut dispatcher = Dispatcher::new(3);

    dispatcher.run(InjectorKind::Global, || println!("Task 0 executed"));
    dispatcher.run(InjectorKind::Worker(0), || println!("Task 1 executed"));
    dispatcher.run(InjectorKind::Worker(1), || println!("Task 2 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 3 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 4 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 5 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 6 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 7 executed"));

    //std::thread::sleep(std::time::Duration::from_millis(100));

    dispatcher.stop();
}
