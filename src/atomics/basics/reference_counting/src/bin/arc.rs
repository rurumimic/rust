use std::sync::Arc;
use std::thread;

fn show(a: Arc<[i32; 3]>) {
    dbg!(&a, a.as_ptr());
}

fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());
    dbg!(a.as_ptr(), b.as_ptr());

    thread::spawn(|| show(a)).join().unwrap();
    thread::spawn(|| dbg!(b)).join().unwrap();
}
