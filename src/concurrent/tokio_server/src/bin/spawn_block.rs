#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut v = Vec::new();
    for n in 0..32 {
        let t = tokio::task::spawn_blocking(move || do_block(n));
        v.push(t);
    }

    let p = tokio::spawn(do_print());

    for t in v {
        let n = t.await.unwrap();
        println!("Receive: {}", n);
    }

    p.await.unwrap();

    Ok(())
}

fn do_block(n: u64) -> u64 {
    let ten_secs = std::time::Duration::from_secs(5);
    std::thread::sleep(ten_secs);
    n
}

async fn do_print() {
    let sec = std::time::Duration::from_secs(1);
    for _ in 0..20 {
        tokio::time::sleep(sec).await;
        println!("tick");
    }
}
