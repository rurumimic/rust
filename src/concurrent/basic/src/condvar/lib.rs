use std::sync::{Arc, Mutex, Condvar};

pub fn child(id: u64, p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();

    // cvar.wait_while(started, |started| !*started).unwrap();
    // or
    while !*started {
        started = cvar.wait(started).unwrap();
    }

    println!("child {}", id);
}

pub fn parent(p: Arc<(Mutex<bool>, Condvar)>) {
    let &(ref lock, ref cvar) = &*p;

    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_all();

    println!("parent");
}

