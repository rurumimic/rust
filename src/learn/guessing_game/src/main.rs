use rand::Rng;
use std::cmp::Ordering;
use std::io;

// thread 'main' panicked at 'The secret number will be between 1 and 100.'
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        /*
        // if value < 1 || value > 100 { // should panic ok
        if value < 1 { // should panic failed
            panic!("The secret number will be between 1 and 100.");
        }
        */

        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!!!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse() {
            Ok(num) => Guess { value: num },
            Err(_) => continue,
        };

        // if guess < 1 || guess > 100 {
        //     println!("The secret number will be between 1 and 100.");
        //     continue;
        // }

        println!("You guessed: {}", guess.value);

        match guess.value.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 999")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
