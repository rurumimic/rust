mod green;

fn ortega() {
    for _ in 0..10 {
        println!("Ortega!");
        green::schedule();
    }
}

fn mash() {
    green::spawn(ortega, 2 * 1024 * 1024);
    for _ in 0..10 {
        println!("Mash!");
        green::schedule();
    }
}

fn gaia() {
    green::spawn(mash, 2 * 1024 * 1024);
    for _ in 0..10 {
        println!("Gaia!");
        green::schedule();
    }
}

fn main() {
    green::spwan_from_main(gaia, 2 * 1024 * 1024); // stack size: 2MB
}
