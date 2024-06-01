use std::time;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    tokio::join!(async move {
        println!("Sleep 10 seconds");
        let ten_secs = time::Duration::from_secs(10);
        tokio::time::sleep(ten_secs).await;
    });

    println!("Done");
    Ok(())
}
