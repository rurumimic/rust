use std::net::SocketAddr;

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const LISTEN_ADDR: &str = "127.0.0.1:10000";

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let listener = TcpListener::bind(LISTEN_ADDR).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("accept: addr = {:?}", addr);

        tokio::spawn(async move {
            server(socket, addr).await;
        });
    }
}

async fn server(mut socket: TcpStream, addr: SocketAddr) {
    let (r, w) = socket.split();
    let mut reader = tokio::io::BufReader::new(r);
    let mut writer = tokio::io::BufWriter::new(w);

    let mut line = String::new();

    loop {
        line.clear();
        match reader.read_line(&mut line).await {
            Ok(0) => {
                println!("closed: addr = {:?}", addr);
                break;
            }
            Ok(_) => {
                println!("read: {:#?} {}", addr, line);
                writer.write_all(line.as_bytes()).await.unwrap();
                writer.flush().await.unwrap();
            }
            Err(e) => {
                eprintln!("error: {:#?} {}", addr, e);
                break;
            }
        }
    }
}
