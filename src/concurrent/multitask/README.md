# Multitask with Green Threads

- github: [oreilly-japan/conc_ytakano/chap6](https://github.com/oreilly-japan/conc_ytakano/tree/main/chap6)

## Run

### Rust

#### Greenthread

```bash
cargo run --bin greenthread

Gaia!
Ortega!
Mash!
Gaia!

# ...

Mash!
Gaia!
Ortega!
Mash!
```

#### Actors

1 producer and 2 consumers:

```bash
cargo run --bin actors

consumer: 9e53acc32685513c - 0
received: count = 0
consumer: f1b9fd8abcc7a035 - 1
received: count = 1
consumer: 9e53acc32685513c - 2
received: count = 2
consumer: f1b9fd8abcc7a035 - 3
received: count = 3
consumer: 9e53acc32685513c - 4
received: count = 4
consumer: f1b9fd8abcc7a035 - 5
received: count = 5
consumer: 9e53acc32685513c - 6
received: count = 6
consumer: f1b9fd8abcc7a035 - 7
received: count = 7
consumer: 9e53acc32685513c - 8
received: count = 8
consumer: f1b9fd8abcc7a035 - 9
received: count = 9
```

### C

Compile:

```bash
gcc asm/context.S -c -fPIC -ggdb -o asm/context.o
ar cruUs asm/libcontext.a asm/context.o # linux amd64

# gcc main.c -Lasm -lcontext -o main
gcc -S main.c -o main.s
gcc -c main.s -o main.o
gcc main.s -Lasm -lcontext -o main
```

Run:

```bash
./main

result: 2
```

### LLDB

```bash
lldb main
break set -n main
break set -n set_context
run
s
register read
memory read -s8 -fx -c16 $rsp
```

### Rust

```bash
lldb target/debug/multitask
break set -n main
run
```

