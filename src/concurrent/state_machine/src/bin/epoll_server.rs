use futures::{
    future::{BoxFuture, FutureExt},
    task::{waker_ref, ArcWake},
};
use nix::{
    errno::Errno,
    sys::{
        epoll::{
            EpollCreateFlags, EpollEvent, EpollFlags, Epoll,
        },
        eventfd::{eventfd, EfdFlags},
    },
    unistd::write,
};
use std::{
    collections::{HashMap, VecDeque},
    future::Future,
    io::{BufRead, BufReader, BufWriter, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    os::{unix::io::{AsRawFd, RawFd}, fd::{OwnedFd, FromRawFd}},
    pin::Pin,
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};

fn write_eventfd(fd: RawFd, n: usize) {
    let ptr = &n as *const usize as *const u8;
    let val = unsafe {
        std::slice::from_raw_parts(ptr, std::mem::size_of_val(&n))
    };
    write(fd, &val).unwrap();
}

enum IOOps {
    ADD(EpollFlags, OwnedFd, Waker),
    REMOVE(OwnedFd),
}

struct IOSelector {
    wakers: Mutex<HashMap<RawFd, Waker>>,
    queue: Mutex<VecDeque<IOOps>>,
    epfd: Epoll,
    event: OwnedFd,
}

impl IOSelector {
    fn new() -> Arc<Self> {
        let s = IOSelector {
            wakers: Mutex::new(HashMap::new()),
            queue: Mutex::new(VecDeque::new()),
            epfd: Epoll::new(EpollCreateFlags::empty()).unwrap(),
            event: eventfd(0, EfdFlags::empty()).unwrap(),
        };

        let result = Arc::new(s);
        let s = result.clone();

        std::thread::spawn(move || s.select());

        result
    }

    fn add_event(&self, flag: EpollFlags, fd: OwnedFd, waker: Waker, wakers: &mut HashMap<RawFd, Waker>) {
        let mut ev = EpollEvent::new(flag | EpollFlags::EPOLLONESHOT, fd.as_raw_fd() as u64);

        if let Err(err) = self.epfd.add(fd.try_clone().unwrap(), ev) {
            match err {
                Errno::EEXIST => {
                    self.epfd.modify(fd.try_clone().unwrap(), &mut ev).unwrap();
                }
                _ => {
                    panic!("epoll_add: {}", err);
                }
            }
        }

        assert!(!wakers.contains_key(&fd.as_raw_fd()));
        wakers.insert(fd.as_raw_fd(), waker);
    }

    fn rm_event(&self, fd: OwnedFd, wakers: &mut HashMap<RawFd, Waker>) {
        self.epfd.delete(fd.try_clone().unwrap()).ok();
        wakers.remove(&fd.as_raw_fd());
    }

    fn select(&self) {
        let ev = EpollEvent::new(EpollFlags::EPOLLIN, self.event.as_raw_fd() as u64);
        let _ = self.epfd.add(&self.event, ev);

        let mut events = vec![EpollEvent::empty(); 1024];
        while let Ok(nfds) = self.epfd.wait(&mut events, -1) {
            let mut t = self.wakers.lock().unwrap();
            for n in 0..nfds {
                if events[n].data() == self.event.as_raw_fd() as u64 {
                    let mut q = self.queue.lock().unwrap();
                    while let Some(op) = q.pop_front() {
                        match op {
                            IOOps::ADD(flag, fd, waker) => self.add_event(flag, fd, waker, &mut t),
                            IOOps::REMOVE(fd) => self.rm_event(fd, &mut t),
                        }
                    }
                } else {
                    let data = events[n].data() as i32;
                    let waker = t.remove(&data).unwrap();
                    waker.wake_by_ref();
                }
            }

        }
    }

    fn register(&self, flags: EpollFlags, fd: OwnedFd, waker: Waker) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(IOOps::ADD(flags, fd, waker));
        write_eventfd(self.event.as_raw_fd(), 1);
    }

    fn unregister(&self, fd: OwnedFd) {
        let mut q = self.queue.lock().unwrap();
        q.push_back(IOOps::REMOVE(fd));
        write_eventfd(self.event.as_raw_fd(), 1);
    }
}

struct AsyncListener {
    listener: TcpListener,
    selector: Arc<IOSelector>,
}

impl AsyncListener {
    fn listen(addr: &str, selector: Arc<IOSelector>) -> AsyncListener {
        let listener = TcpListener::bind(addr).unwrap();

        listener.set_nonblocking(true).unwrap();

        AsyncListener {
            listener,
            selector,
        }
    }

    fn accept(&self) -> Accept {
        Accept {
            listener: self,
        }
    }
}

impl Drop for AsyncListener {
    fn drop(&mut self) {
        self.selector.unregister(unsafe { OwnedFd::from_raw_fd(self.listener.as_raw_fd()) });
    }
}

struct Accept<'a> {
    listener: &'a AsyncListener,
}

impl<'a> Future for Accept<'a> {
    type Output = (AsyncReader, BufWriter<TcpStream>, SocketAddr);

    fn poll(self: Pin<&mut Self>,
    cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.listener.listener.accept() {
            Ok((stream, addr)) => {
                println!("Accept! {}", addr);
                let stream0 = stream.try_clone().unwrap();
                Poll::Ready((
                    AsyncReader::new(stream, self.listener.selector.clone()),
                    BufWriter::new(stream0),
                    addr,
                ))
            }
            Err(err) => {
                print!("?");
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.listener.selector.register(EpollFlags::EPOLLIN, unsafe { OwnedFd::from_raw_fd(self.listener.listener.as_raw_fd()) }, cx.waker().clone());

                Poll::Pending
                } else {
                    panic!("accept: {}", err);
                }

            }
        }
    }
}

struct AsyncReader {
    fd: OwnedFd,
    reader: BufReader<TcpStream>,
    selector: Arc<IOSelector>,
}

impl AsyncReader {
    fn new(stream: TcpStream, selector: Arc<IOSelector>) -> AsyncReader {
        stream.set_nonblocking(true).unwrap();
        AsyncReader {
            fd: unsafe { OwnedFd::from_raw_fd(stream.as_raw_fd()) },
            reader: BufReader::new(stream),
            selector,
        }
    }

    fn read_line(&mut self) -> ReadLine {
        ReadLine {
            reader: self,
        }
    }
}

impl Drop for AsyncReader {
    fn drop(&mut self) {
        self.selector.unregister(self.fd.try_clone().unwrap());
    }
}

struct ReadLine<'a> {
    reader: &'a mut AsyncReader,
}

impl<'a> Future for ReadLine<'a> {
    type Output = Option<String>;

    fn poll(mut self: Pin<&mut Self>,
    cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut line = String::new();
        match self.reader.reader.read_line(&mut line) {
            Ok(0) => {
                print!("0");
                Poll::Ready(None)
            },
            Ok(n) => {
                print!("{}", n);
                Poll::Ready(Some(line))
            },
            Err(err) => {
                print!("!");
                if err.kind() == std::io::ErrorKind::WouldBlock {
                    self.reader.selector.register(EpollFlags::EPOLLIN, self.reader.fd.try_clone().unwrap(), cx.waker().clone());
                    Poll::Pending
                } else {
                    Poll::Ready(None)
                }
            }
        }
    }
}

struct Task {
    future: Mutex<BoxFuture<'static, ()>>,
    sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let self0 = arc_self.clone();
        arc_self.sender.send(self0).unwrap(); // send this ref to Executor
    }
}

struct Executor {
    sender: SyncSender<Arc<Task>>,
    receiver: Receiver<Arc<Task>>,
}

impl Executor {
    fn new() -> Self {
        let (sender, receiver) = sync_channel(1024); // queue size <= 1024
        Executor {
            sender: sender.clone(),
            receiver,
        }
    }

    fn get_spawner(&self) -> Spawner {
        Spawner {
            sender: self.sender.clone(),
        }
    }

    fn run(&self) {
        while let Ok(task) = self.receiver.recv() {
            let mut future = task.future.lock().unwrap();
            let waker = waker_ref(&task);
            let mut ctx = Context::from_waker(&waker);
            let _ = future.as_mut().poll(&mut ctx);
        }
    }
}

struct Spawner {
    sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(future),
            sender: self.sender.clone(),
        });

        self.sender.send(task).unwrap();
    }
}

fn main() {
    let executor = Executor::new();
    let selector = IOSelector::new();
    let spawner = executor.get_spawner();

    println!("telnet 127.0.0.1 10000");

    let server = async move {
        let listener = AsyncListener::listen("127.0.0.1:10000", selector.clone());
        loop {
            let (mut reader, mut writer, addr) = listener.accept().await;
            println!("accept: {}", addr);

            spawner.spawn(async move {
                while let Some(buf) = reader.read_line().await {
                    println!("recv: {}, {}", addr, buf);
                    writer.write(buf.as_bytes()).unwrap();
                    writer.flush().unwrap();
                }
                println!("close: {}", addr);
            });
        }
    };

    executor.get_spawner().spawn(server);
    executor.run();

    println!("Done.");
}

