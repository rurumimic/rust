use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use http_body_util::{combinators::BoxBody, BodyExt, Empty, Full};
use hyper::body::{Body, Bytes, Frame};
use hyper::server::conn::http1;
use hyper::service::{service_fn, Service};
use hyper::{Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

type Counter = i32;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    let svc = Svc {
        counter: Arc::new(Mutex::new(0)),
    };

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        let svc_clone = svc.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                // .serve_connection(io, service_fn(hello))
                .serve_connection(io, svc_clone)
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

#[derive(Debug, Clone)]
struct Svc {
    counter: Arc<Mutex<Counter>>,
}

impl Service<Request<hyper::body::Incoming>> for Svc {
    type Response = Response<BoxBody<Bytes, hyper::Error>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<hyper::body::Incoming>) -> Self::Future {
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
