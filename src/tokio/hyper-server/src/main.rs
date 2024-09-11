use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Poll;

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{Body, Bytes, Frame, Incoming};
use hyper::server::conn::http1;
// use hyper::service::{service_fn, Service};
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use tokio::net::TcpListener;
use tower::Service;
use tower::ServiceBuilder;

// use hyper_server::middleware::hyper_logger::Logger as HyperLogger;
use hyper_server::middleware::tower_logger::Logger as TowerLogger;

type Counter = i32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let http = http1::Builder::new();
    let graceful = hyper_util::server::graceful::GracefulShutdown::new();
    let mut signal = std::pin::pin!(shutdown_signal());

    let svc = Svc {
        counter: Arc::new(Mutex::new(0)),
    };

    loop {
        tokio::select! {
            Ok((stream, _addr)) = listener.accept() => {
                let io = TokioIo::new(stream);

                let svc_clone = svc.clone();
                // let svc_clone = ServiceBuilder::new()
                //     .layer_fn(HyperLogger::new)
                //     .service(svc_clone);
                let svc_clone = ServiceBuilder::new()
                    .layer_fn(TowerLogger::new)
                    .service(svc_clone);
                let svc_clone = TowerToHyperService::new(svc_clone);

                let conn = http.serve_connection(io, svc_clone);
                let fut = graceful.watch(conn);

                tokio::spawn(async move {
                    if let Err(err) = fut.await
                    {
                        eprintln!("Error serving connection: {:?}", err);
                    }
                });
            }
            _ = &mut signal => {
                eprintln!("graceful shutdown signal received");
                break;
            }
        }
    }

    tokio::select! {
        _ = graceful.shutdown() => {
            eprintln!("all connections gracefully closed");
        }
        _ = tokio::time::sleep(std::time::Duration::from_secs(3)) => {
            eprintln!("timed out wait for all connections to close");
        }
    }

    Ok(())
}

#[allow(dead_code)]
async fn hello(_: Request<Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[derive(Debug, Clone)]
struct Svc {
    counter: Arc<Mutex<Counter>>,
}

impl Service<Request<Incoming>> for Svc {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    /* for tower middleware */
    fn poll_ready(
        &mut self,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    /* hyper middleware
    // fn call(&self, req: Request<Incoming>) -> Self::Future {
     */
    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        let counter = self.counter.clone();

        let future = async move {
            if req.uri().path() != "/favicon.ico" {
                *counter.lock().expect("lock poisoned") += 1;
            }

            let res = match (req.method(), req.uri().path()) {
                (&Method::GET, "/") => Ok(Response::new(full(format!(
                    "home! counter = {:?}",
                    counter
                )))),
                (&Method::GET, "/posts") => Ok(Response::new(full(format!(
                    "posts, of course! counter = {:?}",
                    counter
                )))),
                (&Method::GET, "/authors") => Ok(Response::new(full(format!(
                    "authors extraodinare! counter = {:?}",
                    counter
                )))),
                (&Method::GET, "/slow") => {
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    Ok(Response::new(full(format!("slow response"))))
                }
                (&Method::POST, "/echo") => Ok(Response::new(req.into_body().boxed())),
                (&Method::POST, "/echo/uppercase") => {
                    let frame_stream = req.into_body().map_frame(|frame| {
                        let frame = if let Ok(data) = frame.into_data() {
                            data.iter()
                                .map(|byte| byte.to_ascii_uppercase())
                                .collect::<Bytes>()
                        } else {
                            Bytes::new()
                        };
                        Frame::data(frame)
                    });
                    Ok(Response::new(frame_stream.boxed()))
                }
                (&Method::POST, "/echo/reversed") => {
                    let upper = req.body().size_hint().upper().unwrap_or(u64::MAX);
                    if upper > 1024 * 64 {
                        let mut resp = Response::new(full("Body too big"));
                        *resp.status_mut() = hyper::StatusCode::PAYLOAD_TOO_LARGE;
                        return Ok(resp);
                    }

                    let whole_body = req.collect().await?.to_bytes();
                    let reversed_body = whole_body.iter().rev().cloned().collect::<Vec<u8>>();
                    Ok(Response::new(full(reversed_body)))
                }
                _ => {
                    let mut not_found = Response::new(empty());
                    *not_found.status_mut() = StatusCode::NOT_FOUND;
                    Ok(not_found)
                }
            };

            res
        };

        Box::pin(future)
    }
}

fn empty() -> BoxBody<Bytes, hyper::Error> {
    Empty::<Bytes>::new()
        .map_err(|never| match never {})
        .boxed()
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}
