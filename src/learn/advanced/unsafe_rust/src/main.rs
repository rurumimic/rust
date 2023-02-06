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
}
