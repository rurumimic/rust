fn compose_two<FIRST, SECOND, THIRD, F, G>(f: F, g: G) -> impl Fn(FIRST) -> THIRD
where
    F: Fn(FIRST) -> SECOND,
    G: Fn(SECOND) -> THIRD,
{
    move |x| g(f(x))
}

macro_rules! compose {
    ($last:expr) => { $last };
    ($head:expr, $($tail:expr),+) => {
        compose_two($head, compose!($($tail), +))
    };
    ($head:expr => $($tail:expr)=>+) => {
        compose_two($head, compose!($($tail)=>+))
    };
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn stringify(x: i32) -> String {
    x.to_string()
}

fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
    move |x| format!("{}{}", prefix, x)
}

fn main() {
    let composed = compose!(add_one, stringify, prefix_with("Result: "));
    println!("{}", composed(5));

    let composed = compose!(add_one => stringify => prefix_with("Result: "));
    println!("{}", composed(7));
}
