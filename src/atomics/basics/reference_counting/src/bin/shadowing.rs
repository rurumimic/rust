use std::sync::Arc;
use std::thread;

fn main() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(&a, a.as_ptr());
        }
    })
    .join()
    .unwrap();

    thread::spawn({
        let b = b.clone();
        move || {
            dbg!(&b, b.as_ptr());
        }
    })
    .join()
    .unwrap();

    dbg!(a.as_ptr(), b.as_ptr());
}
