mod bubble_sort;

pub use self::bubble_sort::bubble_sort;

#[cfg(test)]
use std::cmp;

#[cfg(test)]
pub fn is_sorted<T>(arr: &[T]) -> bool
where T: cmp::PartialOrd {
    arr.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
pub fn is_same<T>(a: &[T], b: &[T]) -> bool
where T: cmp::PartialOrd + cmp::Eq + std::hash::Hash {
    use std::collections::HashSet;
    match a.len() == b.len() {
        true => {
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();
            set_a == set_b
        }
        false => false,
    }
}

