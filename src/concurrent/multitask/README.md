# Multitask with Green Threads

- github: [oreilly-japan/conc_ytakano/chap6](https://github.com/oreilly-japan/conc_ytakano/tree/main/chap6)

## Run

### Rust

Run:

```bash
./main

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

