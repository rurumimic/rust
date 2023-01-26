#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[derive(Debug)]
enum _List<'a> {
    _Cons(i32, &'a _List<'a>),
    _Nil,
}

use crate::List::{Cons, Nil};
use crate::_List::{_Cons, _Nil};
use std::rc::Rc;

fn with_lifetime() {
    let a = _Cons(5, &_Cons(10, &_Nil));
    let b = _Cons(3, &a);
    let c = _Cons(4, &a);

    println!("_a {:?}", a);
    println!("_b {:?}", b);
    println!("_c {:?}", c);
}

fn with_refcount() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("a {:?}", a);
    println!("b {:?}", b);
    println!("c {:?}", c);
}

fn counting() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of = {}", Rc::strong_count(&a));
}

fn main() {
    with_lifetime();
    with_refcount();
    counting();
}
