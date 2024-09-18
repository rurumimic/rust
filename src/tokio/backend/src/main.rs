use axum::{response::Html, routing::get, serve, Router};
use tokio::net::TcpListener;

use backend::errors::AppError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("listening on {}", listener.local_addr()?);

    serve(listener, app).await?;

    Ok(())
}

#[axum::debug_handler]
async fn handler() -> Result<Html<&'static str>, AppError> {
    try_thing()?;
    Ok(Html("<h1>Hello, World!</h1>"))
}

fn try_thing() -> anyhow::Result<()> {
    anyhow::bail!("oh no");
}
