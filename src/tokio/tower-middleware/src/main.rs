use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{Bytes, Incoming};
use hyper::server::conn::http1;
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tower::{Service, ServiceBuilder};

use tower_middleware::timeout::layer::TimeoutLayer;
// use tower_middleware::timeout::service::Timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let svc = Svc {};

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        let svc_clone = svc.clone();
        let svc_clone = ServiceBuilder::new()
            .layer(TimeoutLayer::new(Duration::from_secs(1)))
            // .layer_fn(|inner| Timeout::new(inner, Duration::from_secs(1)))
            .service(svc_clone);
        let svc_clone = TowerToHyperService::new(svc_clone);

        let conn = http1::Builder::new().serve_connection(io, svc_clone);
        tokio::task::spawn(async move {
            if let Err(err) = conn.await {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

#[derive(Debug, Clone)]
struct Svc {}

impl Service<Request<Incoming>> for Svc {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        let future = async move {
            let res = match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Ok(Response::new(
                    Full::new(Bytes::from("Hello, World!"))
                        .map_err(|never| match never {})
                        .boxed(),
                )),
                (&Method::GET, "/slow") => {
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    Ok(Response::new(
                        Full::new(Bytes::from("Slow response!"))
                            .map_err(|never| match never {})
                            .boxed(),
                    ))
                }
                _ => {
                    let mut not_found = Response::new(
                        Empty::<Bytes>::new()
                            .map_err(|never| match never {})
                            .boxed(),
                    );
                    *not_found.status_mut() = StatusCode::NOT_FOUND;
                    Ok(not_found)
                }
            };
            res
        };

        Box::pin(future)
    }
}
