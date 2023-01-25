use add_one;
use add_two;
use rand::Rng;

fn main() {
    let num = 10;
    println!("{num} plus one is {}!", add_one::add_one(num));
    println!("{num} plus two is {}!", add_two::add_two(num));
    println!("{num} plus random number is {}!", add_one::add_rand(num));
    println!("a random number: {}", rand::thread_rng().gen_range(0..10));
}
