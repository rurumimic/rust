fn main() {
    if_let();
    while_let();
    for_loops();
    function_parameters();
    matching_named_variables();
    matching_ranges();
    destructuring_structs();
    destructuring_nested_structs_and_enums();
}

#[allow(dead_code)]
fn if_let() {
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
}

#[allow(dead_code)]
fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

#[allow(dead_code)]
fn for_loops() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index); // a is at index 0, b is at index 1, c is at index 2
    }
}

#[allow(dead_code)]
fn function_parameters() {
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

#[allow(dead_code)]
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // Matched, y = 5
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y); // at the end: x = Some(5), y = 10
}

#[allow(dead_code)]
fn matching_ranges() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

#[allow(dead_code)]
fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // a = 0, b = 7
    println!("a: {}, b: {}", a, b);

    let Point { x, y } = p; // x = 0, y = 7
    println!("x: {}, y: {}", x, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y), // On the y axis at 7
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

struct Point {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

#[allow(dead_code)]
fn destructuring_nested_structs_and_enums() {
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
}
