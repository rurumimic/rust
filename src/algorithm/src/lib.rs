pub mod sort;

#[cfg(test)]
mod tests {

    use super::sort;

    #[test]
    fn bubble_sort() {
        let mut vec1 = vec![6, 5, 4, 3, 2, 1];
        sort::bubble_sort(&mut vec1);


    }
}
