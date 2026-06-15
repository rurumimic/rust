use jemalloc::Metrics;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() {
    env_logger::init();

    let metrics = Metrics::new();
    println!("Backend: {}", metrics.backend());

    let mut v = Vec::<i32>::new();

    for i in 0..10 {
        v.push(i);

        if let Some(stats) = metrics.read() {
            println!(
                "[{i:>2}] allocated {:>9} | active {:>9} | resident {:>9} | mapped {:>9} | retained {:>9} | metadata {:>9}",
                stats.allocated,
                stats.active,
                stats.resident,
                stats.mapped,
                stats.retained,
                stats.metadata,
            );
        }
    }
}
