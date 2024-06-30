mod green;

const ITER: i32 = 10;
const STACK_SIZE: usize = 2 * 1024 * 1024; // 2MB

#[allow(dead_code)]
fn ortega() {
    for _ in 0..ITER {
        println!("Ortega!");
        green::schedule();
    }
}

#[allow(dead_code)]
fn mash() {
    green::spawn(ortega, STACK_SIZE);
    for _ in 0..ITER {
        println!("Mash!");
        green::schedule();
    }
}

#[allow(dead_code)]
fn gaia() {
    green::spawn(mash, STACK_SIZE);
    for _ in 0..ITER {
        println!("Gaia!");
        green::schedule();
    }
}

fn main() {
    green::spawn_from_main(gaia, STACK_SIZE);
}
