use axum::{response::Html, routing::get, serve, Router};
use listenfd::ListenFd;
use tokio::net::TcpListener;

use backend::errors::AppError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(handler));

    let mut listenfd = ListenFd::from_env();

    let listener = match listenfd.take_tcp_listener(0)? {
        Some(listener) => {
            listener.set_nonblocking(true)?;
            TcpListener::from_std(listener)?
        }
        None => TcpListener::bind("127.0.0.1:3000").await?,
    };

    println!("listening on {}", listener.local_addr()?);

    serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn handler() -> Result<Html<&'static str>, AppError> {
    Ok(Html("<h1>Hello, World!</h1>"))
}
