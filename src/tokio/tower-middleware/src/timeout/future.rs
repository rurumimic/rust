use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use pin_project::pin_project;
use tokio::time::Sleep;

use super::error::{BoxError, TimeoutError};

#[pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    response_future: F,
    #[pin]
    sleep: Sleep,
}

impl<F> ResponseFuture<F> {
    pub(crate) fn new(response_future: F, sleep: Sleep) -> Self {
        ResponseFuture {
            response_future,
            sleep,
        }
    }
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<BoxError>,
{
    type Output = Result<Response, BoxError>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let response_future: Pin<&mut F> = this.response_future;
        let sleep: Pin<&mut Sleep> = this.sleep;

        match response_future.poll(cx) {
            Poll::Ready(result) => {
                let result = result.map_err(Into::into);
                return Poll::Ready(result);
            }
            Poll::Pending => {}
        }

        match sleep.poll(cx) {
            Poll::Ready(()) => {
                let error = Box::new(TimeoutError::new());
                return Poll::Ready(Err(error));
            }
            Poll::Pending => {}
        }

        Poll::Pending
    }
}
