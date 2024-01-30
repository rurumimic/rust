# Interactive Server

## Simple Server

```bash
telnet localhost 10000

Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello
hello
Connection closed by foreign host.
```

## Epoll Server

```bash
accept: fd = 5
read: fd = 5, buf = hello
read: fd = 5, buf = world
closed: fd = 5
```

```bash
telnet localhost 10000

Trying 127.0.0.1...
Connected to localhost.
Escape character is '^]'.
hello
hello
world
world
^]
telnet> quit
Connection closed.
```

