use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

use nix::sys::epoll::{
    epoll_create1, epoll_ctl, epoll_wait, EpollCreateFlags, EpollEvent, EpollFlags, EpollOp,
};

fn main() {
    let epoll_in = EpollFlags::EPOLLIN;

    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    while let Ok((socket, _addr)) = listener.accept() {
        let socket0 = socket.try_clone().unwrap();
        let mut reader = BufReader::new(socket0);
        let mut writer = BufWriter::new(socket);

        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        writer.write(buf.as_bytes()).unwrap();
        writer.flush().unwrap();
    }
}
