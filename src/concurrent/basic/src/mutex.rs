use std::sync::{Arc, Mutex};

pub fn add_one(lock: Arc<Mutex<u64>>) {
    loop {
        let mut val = lock.lock().unwrap();
        *val += 1;
        println!("{}", *val);
        break;
    }
}
