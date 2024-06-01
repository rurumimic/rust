use tokio::sync::oneshot;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(set_val_later(tx));


    match rx.await {
        Ok(n) => println!("Receive: {}", n),
        Err(_) => println!("Failed to receive"),
    }

    Ok(())
}

async fn set_val_later(tx: oneshot::Sender<i32>) {
    let ten_secs = std::time::Duration::from_secs(5);
    tokio::time::sleep(ten_secs).await;
    if let Err(_) = tx.send(123) {
        println!("failed to send");
    }
}
