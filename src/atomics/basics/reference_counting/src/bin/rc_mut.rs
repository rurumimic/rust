use std::rc::Rc;

fn main() {
    let mut a = Rc::new(vec![1, 2, 3]);
    let b = Rc::clone(&a);

    assert!(Rc::ptr_eq(&a, &b));
    assert_eq!(Rc::strong_count(&a), 2);
    dbg!(a.as_ptr(), b.as_ptr());

    Rc::make_mut(&mut a).push(4);

    assert_eq!(a.as_slice(), &[1, 2, 3, 4]);
    assert_eq!(b.as_slice(), &[1, 2, 3]);
    dbg!(a.as_ptr(), b.as_ptr());
}
