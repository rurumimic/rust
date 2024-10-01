use std::rc::Rc;
// use std::thread;

fn main() {
    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());
    dbg!(a.as_ptr(), b.as_ptr());

    /*
     * `Rc<[{integer}; 3]>` cannot be sent between threads safely
     * the trait `Send` is not implemented for `Rc<[{integer}; 3]>`
     */
    // thread::spawn(move || dbg!(b)).join().unwrap();
}
