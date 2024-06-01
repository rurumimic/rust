use std::sync::Arc;
use tokio::sync::Mutex;

const NUM_TASKS: usize = 8;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let val = Arc::new(Mutex::new(0));
    let mut v = Vec::new();

    let t = tokio::spawn(lock_sleep(val.clone()));
    v.push(t);

    for _ in 0..NUM_TASKS {
        let n = val.clone();
        let t = tokio::spawn(lock_only(n));
        v.push(t);
    }

    for i in v {
        i.await?;
    }

    println!("Count = {}", *val.lock().await);
    Ok(())
}

async fn lock_only(v: Arc<Mutex<u64>>) {
    let mut n = v.lock().await;
    *n += 1;
}

async fn lock_sleep(v: Arc<Mutex<u64>>) {
    let mut n = v.lock().await;
    let ten_secs = std::time::Duration::from_secs(5);
    tokio::time::sleep(ten_secs).await;
    *n += 1000;
}

