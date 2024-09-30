use std::thread;

fn main() {
    let mut threads = vec![];
    let leaked: &'static [i32; 3] = Box::leak(Box::new([4, 5, 6]));

    threads.push(thread::spawn(move || dbg!(leaked)));
    threads.push(thread::spawn(move || dbg!(leaked)));

    for t in threads {
        t.join().unwrap();
    }

    println!("Leaked: {:?}", leaked); // Leaked: [4, 5, 6]

    // &T to *const T to *mut T
    let raw_pointer = leaked as *const [i32; 3] as *mut [i32; 3];

    unsafe {
        println!("Leaked: {:?}", leaked); // Leaked: [4, 5, 6]

        // *mut T to Box<T>
        let boxed: Box<[i32; 3]> = Box::from_raw(raw_pointer);

        dbg!(boxed); // [4, 5, 6]
        dbg!(leaked); // [936108917, 6, -1069593694]
    }

    dbg!(leaked); // [936108917, 6, -1069593694]
}
