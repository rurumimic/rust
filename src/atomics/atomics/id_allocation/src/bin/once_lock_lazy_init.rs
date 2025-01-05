use std::sync::OnceLock;

fn get_key() -> u64 {
    static KEY: OnceLock<u64> = OnceLock::new();

    *KEY.get_or_init(|| generate_random_key())
}

fn generate_random_key() -> u64 {
    42
}

fn main() {
    dbg!(get_key());
    dbg!(get_key());
    dbg!(get_key());
}
