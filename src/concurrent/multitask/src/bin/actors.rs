use multitask::green;

const ITER: u64 = 10;
const STACK_SIZE: usize = 2 * 1024 * 1024; // 2MB

fn producer() {
    let consumers = vec![green::spawn(consumer, STACK_SIZE), green::spawn(consumer, STACK_SIZE)];
    for i in 0..ITER {
        let id = consumers[i as usize % consumers.len()];
        green::send(id, i);
    }
}

fn consumer() {
    for _ in 0..ITER {
        let msg = green::recv().unwrap();
        println!("received: count = {}", msg);
    }
}

fn main() {
    green::spawn_from_main(producer, STACK_SIZE);
}
