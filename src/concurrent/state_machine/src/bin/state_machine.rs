use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use futures::FutureExt;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

struct Hello {
    state: StateHello,
}

enum StateHello {
    HELLO,
    WORLD,
    END,
}

impl Hello {
    fn new() -> Self {
        Hello {
            state: StateHello::HELLO,
        }
    }
}

impl Future for Hello {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        match (*self).state {
            StateHello::HELLO => {
                print!("Hello, ");
                (*self).state = StateHello::WORLD;
                Poll::Pending
            }
            StateHello::WORLD => {
                print!("World");
                (*self).state = StateHello::END;
                Poll::Pending
            }
            StateHello::END => {
                println!("!");
                Poll::Ready(())
            }
        }
    }
}

struct Task {
    hello: Mutex<BoxFuture<'static, ()>>,
}

impl Task {
    fn new() -> Self {
        let hello = Hello::new();

        Task {
            hello: Mutex::new(hello.boxed()),
        }
    }
}

impl ArcWake for Task {
    fn wake_by_ref(_arc_self: &std::sync::Arc<Self>) {}
}

fn main() {
    let task = Arc::new(Task::new());
    let waker = waker_ref(&task);
    let mut ctx = Context::from_waker(&waker);
    let mut hello = task.hello.lock().unwrap();

    let _ = hello.as_mut().poll(&mut ctx);
    let _ = hello.as_mut().poll(&mut ctx);
    let _ = hello.as_mut().poll(&mut ctx);
}
