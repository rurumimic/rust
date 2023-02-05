# Patterns and Matching

- book: [Patterns and Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)

Patterns with:

- Literals
- Destructured arrays, enums, structs, or tuples
- Variables
- Wildcards
- Placeholders

---

## All Patterns

- book: [All the Places Patterns Can Be Used](https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html)

### match Arms

```rs
match VALUE {
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
  PATTERN => EXPRESSION,
}
```

```rs
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

### Conditional if let Expressions

a shorter way to write the equivalent of a match that only matches one case

```rs
let favorite_color: Option<&str> = None; // Some("Red")
let is_tuesday = false;
let age: Result<u8, _> = "34".parse();

if let Some(color) = favorite_color {
    println!("Using your favorite color, {color}, as the background");
} else if is_tuesday {
    println!("Tuesday is green day!");
} else if let Ok(age) = age {
    if age > 30 {
        println!("Using purple as the background color");
    } else {
        println!("Using oragne as the background color");
    }
} else {
    println!("Using blue as the background color"); // print
}
```

### while let Conditional Loops

```rs
while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

### for Loops

```rs
for (index, value) in v.iter().enumerate() {
    println!("{} is at index {}", value, index);
    // a is at index 0
    // b is at index 1
    // c is at index 2
}
```

### let Statements

```rs
let PATTERN = EXPRESSION;
```

```rs
let x = 5;
let (x, y, z) = (1, 2, 3);
```

### Function Parameters

```rs
fn foo(PATTERN: TYPE) {}
```

```rs
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

let point = (3, 5);
print_coordinates(&point);
```

---

## Refutability

- book: [Refutability: Whether a Pattern Might Fail to Match](https://doc.rust-lang.org/book/ch18-02-refutability.html)

refutable vs irrefutable:

- refutable: `if let`, `while let`, `Some(x)`
  - intend to handle possible failure
- irrefutable: function parameters, let statement, for loops

```rs
match value {
  PATTERN => refutable EXPRESSION,
  PATTERN => refutable EXPRESSION,
  PATTERN => refutable EXPRESSION,
  PATTERN => irrefutable EXPRESSION,
}
```

```rs
match value {
  PATTERN => irrefutable EXPRESSION,
}

// == sames as
let PATTERN = EXPRESSION;
```

---

## Pattern Syntax

- book: [Pattern Syntax](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)

### Matching Literals

```rs
let x = 1;

match x {
  1 => println!("one");
  2 => println!("two");
  _ => println!("anything");
}
```

### Matching Named Variables

- Named variables = irrefutable patterns
- match starts a new scope
- match expression will shadow those with the same name outside the match construct

compare: [Extra Conditionals with Match Guards](#extra-conditionals-with-match-guards)

```rs
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {:?}", y),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {:?}", x, y);
```

```bash
Matched, y = 5
at the end: x = Some(5), y = 10
```

### Multiple Patterns

```rs
let x = 1;

match x {
  1 | 2 => println!("one or two");
  3 => println!("three"),
  _ => println!("anything"),
}
```

### Matching Ranges of Values with ..=

```rs
let x = 5;

match x {
    1..=5 => println!("one through five"), // 1 | 2 | 3 | 4 | 5
    _ => println!("something else"),
}

let x = 'c';

match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

### Destructuring to Break Apart Values

#### Destructuring Structs

```rs
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };

let Point { x: a, y: b } = p; // a = 0, b = 7
let Point { x, y } = p; // x = 0, y = 7
match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y), // On the y axis at 7
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

#### Destructuring Enums

```rs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::ChangeColor(0, 160, 255);

match msg {
    Message::Quit => {
        println!("The Quit variant has no data to destructure.");
    }
    Message::Move { x, y } => {
        println!("Move in the x direction {x} and in the y direction {y}");
    }
    Message::Write(text) => {
        println!("Text message: {text}");
    }
    Message::ChangeColor(r, g, b) => {
        println!("Change the color to red {r}, green {g}, and blue {b}",)
    }
}
```

#### Destructuring Nested Structs and Enums

```rs
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

match msg {
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
        println!("Change color to red {r}, green {g}, and blue {b}");
    }
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
        println!("Change color to hue {h}, saturation {s}, value {v}")
    }
    _ => (),
}
```

#### Destructuring Structs and Tuples

```rs
struct Point {
    x: i32,
    y: i32,
}

let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
```

### Ignoring Values in a Pattern

#### Ignoring an Entire Value with _

```rs
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

foo(3, 4);
```

#### Ignoring Parts of a Value with a Nested _

```rs
let mut setting_value = Some(5);
let new_setting_value = Some(10);

match (setting_value, new_setting_value) {
    (Some(_), Some(_)) => {
        println!("Can't overwrite an existing customized value"); // print
    }
    _ => { // (None, Some(_)) / (Some(_), None) / (None, None)
        setting_value = new_setting_value;
    }
}

println!("setting is {:?}", setting_value); // Some(5)
```

```rs
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, _, third, _, fifth) => {
        println!("Some numbers: {first}, {third}, {fifth}") // 2, 8, 32
    }
}
```

#### Ignoring an Unused Variable by Starting Its Name with _

```rs
let s = Some(String::from("Hello!"));

// Error: 
// if let Some(_s) = s {
if let Some(_) = s {
    println!("found a string");
}

println!("{:?}", s);
```

#### Ignoring Remaining Parts of a Value with '..'

`..` must be unambiguous.

```rs
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

```rs
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {first}, {last}");
    }
}
```

### Extra Conditionals with Match Guards

```rs
let num = Some(4);

match num {
    Some(x) if x % 2 == 0 => println!("The number {} is even", x), // The number 4 is even
    Some(x) => println!("The number {} is odd", x),
    None => (),
}
```

compare: [Matching Named Variables](#matching-named-variables)

```rs
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(n) if n == y => println!("Matched, n = {:?}", n),
    _ => println!("Default case, x = {:?}", x), // Default case, x = Some(5)
}

println!("at the end: x = {:?}, y = {:?}", x, y);
```

- `if n == y` is not a pattern
- doens't introduce new variables
- `y` is the outer `y`

```rs
let x = 4;
let y = false;

match x {
    4 | 5 | 6 if y => println!("yes"), // print
    _ => println!("no"),
}

match x {
    (4 | 5 | 6) if y => println!("yes"), // print
    _ => println!("no"),
}
```

### @ Bindings

- *at* operator: `@`
- create a variable
  - holds a value at the same time as we're testing that value for a pattern match
- test a value and save it in a variable within one pattern

```rs
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };

match msg {
    Message::Hello {
        id: id_variable @ 3..=7,
    } => println!("Found an id in range: {}", id_variable), // Found an id in range: 5
    Message::Hello { id: 10..=12 } => {
        println!("Found an id in another range") // unable to use `id`
    }
    Message::Hello { id } => println!("Found some other id: {}", id),
}
```
