static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    // cannot borrow `num` as mutable because it is also borrowed as immutable
    // let ref_num = &num;
    //               ---- immutable borrow occurs here
    // let mut ref_num2 = &mut num;
    //                    ^^^^^^^^ mutable borrow occurs here
    // println!("ref_num is: {}", ref_num);
    //                            ------- immutable borrow later used here
    // println!("ref_num2 is: {}", ref_num2);

    let r1 = &num as *const i32; // r1 is: 5
    let r2 = &mut num as *mut i32; // r2 is: 5

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // unsafe function
    unsafe {
        dangerous();
    }

    // safe abstraction
    safe_abstraction();

    // static variable
    println!("name is: {}", HELLO_WORLD);
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {
    println!("fn dangerous");
}

fn safe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
