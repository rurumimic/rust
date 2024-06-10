# Multitask with Green Threads

- github: [oreilly-japan/conc_ytakano/chap6](https://github.com/oreilly-japan/conc_ytakano/tree/main/chap6)

## Run

### C

Compile:

```bash
gcc asm/context.S -c -fPIC -ggdb -o asm/context.o
ar cruUs asm/libcontext.a asm/context.o

# gcc main.c -Lasm -lcontext -o main
gcc -S main.c -o main.s
gcc -c main.s -o main.o
gcc main.s -Lasm -lcontext -o main
```

Run:

```bash
./main
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

