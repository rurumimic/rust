use deque::with_injector::Dispatcher;
use deque::with_injector::InjectorKind;

fn main() {
    let mut dispatcher = Dispatcher::new(3);

    let (tx, rx) = std::sync::mpsc::channel();

    dispatcher.run(InjectorKind::Global, || println!("Task 0 executed"));
    dispatcher.run(InjectorKind::Worker(0), {
        let tx = tx.clone();
        move || {
            tx.send(10).unwrap();
            println!("Task 1 executed");
        }
    });

    dispatcher.run(InjectorKind::Worker(1), || println!("Task 2 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 3 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 4 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 5 executed"));
    dispatcher.run(InjectorKind::Worker(2), || println!("Task 6 executed"));

    dispatcher.run(InjectorKind::Worker(2), {
        let tx = tx.clone();
        move || {
            tx.send(27).unwrap();
            println!("Task 7 executed");
        }
    });

    let recv = dispatcher
        .submit_task(InjectorKind::Worker(2), |x: i32| x * 3, 58)
        .recv()
        .unwrap();

    //std::thread::sleep(std::time::Duration::from_millis(100));

    dispatcher.stop();

    let result = rx.recv().unwrap();
    println!("Received from worker: {}", result);

    let result = rx.recv().unwrap();
    println!("Received from worker: {}", result);

    println!("Received from worker: {}", recv);
}
