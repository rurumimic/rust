use declarative::greeting;
use declarative::greeting::base_greeting_fn;

fn main() {
    let greet = greeting!("Sam", "Heya");
    println!("{}", greet);

    let greet_with_default = greeting!("Sam");
    println!("{}", greet_with_default);
}
