use std::{fmt, str::FromStr};

use axum::extract::{Query, TypedHeader};
use axum::{response::Html, routing::get, serve, Router};
use listenfd::ListenFd;
use serde::{de, Deserialize, Deserializer};
use tokio::net::TcpListener;

use backend::errors::AppError;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut listenfd = ListenFd::from_env();

    let listener = match listenfd.take_tcp_listener(0)? {
        Some(listener) => {
            listener.set_nonblocking(true)?;
            TcpListener::from_std(listener)?
        }
        None => TcpListener::bind("127.0.0.1:3000").await?,
    };

    println!("listening on {}", listener.local_addr()?);

    serve(listener, app()).await?;

    Ok(())
}

fn app() -> Router {
    Router::new().route("/", get(handler))
}

#[axum::debug_handler]
async fn handler(
    Query(params): Query<Params>,
    TypedHeader(accept): TypedHeader<Accept>,
) -> Result<Html<String>, AppError> {
    let html = format!(
        "<h1>Hello, World!</h1>
         <p>GET <a href=\"/?no=23&name=jordan\">/?no=23&name=jordan</a></p>
         <pre>no: {}</pre>
         <pre>name: {}</pre>",
        params.no.unwrap_or(0),
        params.name.unwrap_or_else(|| "''".into()),
    );

    Ok(Html(html))
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    no: Option<i32>,
    name: Option<String>,
    content_type: Option<String>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, http::Request};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_something() {}

    async fn send_request_get_body(query: &str) -> String {
        let body = app()
            .oneshot(
                Request::builder()
                    .uri(format!("/?{query}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap()
            .into_body();
        let bytes = body.collect().await.unwrap().to_bytes();
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}
