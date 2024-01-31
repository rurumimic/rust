use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};

use nix::sys::epoll::{Epoll, EpollCreateFlags, EpollEvent, EpollFlags};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    let epfd = Epoll::new(EpollCreateFlags::empty()).unwrap();

    let listen_fd = listener.as_raw_fd();
    let ev = EpollEvent::new(EpollFlags::EPOLLIN, listen_fd as u64);
    let listen_owned_fd: OwnedFd = unsafe { OwnedFd::from_raw_fd(listen_fd) };
    if let Err(err) = epfd.add(&listen_owned_fd, ev) {
        println!("epoll add: {:?}", err);
        return;
    }

    let mut fd2buf = HashMap::new();
    let mut events = vec![EpollEvent::empty(); 1024];

    while let Ok(nfds) = epfd.wait(&mut events, -1) {
        for n in 0..nfds {
            if events[n].data() == listen_fd as u64 {
                println!("accept: listen_fd = {}", listen_fd);
                if let Ok((stream, addr)) = listener.accept() {
                    println!("accept: addr = {:?}", addr);
                    let fd = stream.as_raw_fd();
                    let owned_fd: OwnedFd = unsafe { OwnedFd::from_raw_fd(fd) };
                    let stream0 = stream.try_clone().unwrap();
                    let reader = BufReader::new(stream0);
                    let writer = BufWriter::new(stream);

                    let ev = EpollEvent::new(EpollFlags::EPOLLIN, fd as u64);
                    epfd.add(&owned_fd, ev).unwrap();

                    fd2buf.insert(fd, (owned_fd, reader, writer));

                    println!("accept: fd = {}", fd);
                }
            } else {
                let fd = events[n].data() as RawFd;
                let (owned_fd, reader, writer) = fd2buf.get_mut(&fd).unwrap();

                let mut buf = String::new();
                let n = reader.read_line(&mut buf).unwrap();

                if n == 0 {
                    epfd.delete(&owned_fd).unwrap();
                    fd2buf.remove(&fd);
                    println!("closed: fd = {}", fd);
                    continue;
                }

                print!("read: fd = {}, buf = {}", fd, buf);

                writer.write(buf.as_bytes()).unwrap();
                writer.flush().unwrap();
            }
        }
    }
}

#[allow(dead_code)]
fn server(listener: TcpListener) {
    while let Ok((socket, _addr)) = listener.accept() {
        let socket0 = socket.try_clone().unwrap();
        let mut reader = BufReader::new(socket0);
        let mut writer = BufWriter::new(socket);

        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        print!("Read: {}", buf);
        writer.write(buf.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
