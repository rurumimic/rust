use axum::serve;
use axum::{response::Html, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    println!("listening on {}", listener.local_addr()?);

    serve(listener, app).await?;

    Ok(())
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
