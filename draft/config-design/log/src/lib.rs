#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    };
}
