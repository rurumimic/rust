# Simple Shell

- denoland/rusty_v8: [examples/shell.rs](https://github.com/denoland/rusty_v8/blob/ab019251a4c45ee4edb4f7415eec762a589d87f8/examples/shell.rs)

## Run

```bash
cargo build
cargo run
```

```bash
V8 version 12.9.202.18-rusty [sample shell]
> 2 + 2;
4
> let x = 10; x * 2;
20
> function greet(name) { return "Hello, " + name + "!"; } greet("World");
Hello, World!
> let person = { name: "John", age: 30 }; person.name;
John
> let arr = [1, 2, 3]; arr.push(4); arr;
1,2,3,4
> let num = 5; if (num > 3) "greater than 3"; else "less than 3";
greater than 3
> for (let i = 0; i < 3; i++) console.log(i);
undefined
> try { throw new Error("Test Error"); } catch (e) { e.message; }
Test Error
> "Hello".toUpperCase();
HELLO
> let today = new Date(); today.toDateString();
Thu Sep 26 2024
> ^D
```

