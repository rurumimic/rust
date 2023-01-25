//! # Add One
//!
//! `add_one` is a collection of utilities to make performing certain
//! calculations more convenient.
use rand::Rng;

/// Adds one to the number given.
///
/// # Examples
///
/// ```rs
/// let arg = 5;
/// let answer = add_one::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_rand(x: i32) -> i32 {
    x + rand::thread_rng().gen_range(0..10)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
