fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    println!("{}", apply_twice(add_one, 3));
}
