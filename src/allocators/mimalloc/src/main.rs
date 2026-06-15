use mimalloc::Metrics;

#[global_allocator]
static GLOBAL: mimallocator::MiMalloc = mimallocator::MiMalloc;

/// 4 MiB per step. mimalloc's process info is page-granular, so tiny `Vec<i32>`
/// pushes (as in the jemalloc demo) would not move the numbers; we allocate and
/// touch real pages instead.
const CHUNK: usize = 4 * 1024 * 1024;

fn main() {
    env_logger::init();

    let metrics = Metrics::new();
    log::info!("metrics backend: {}", metrics.backend());
    println!("Backend: {}", metrics.backend());

    let mut chunks: Vec<Vec<u8>> = Vec::new();

    for i in 0..10 {
        // Allocate and touch one byte per page so the pages actually fault in;
        // a zeroed `vec![0u8; CHUNK]` may otherwise stay lazily mapped.
        let mut chunk = vec![0u8; CHUNK];
        for byte in chunk.iter_mut().step_by(4096) {
            *byte = 1;
        }
        chunks.push(chunk);

        let stats = metrics.read();
        println!(
            "[{i:>2}] current_rss {:>10} | peak_rss {:>10} | current_commit {:>10} | peak_commit {:>10} | page_faults {:>8}",
            stats.current_rss,
            stats.peak_rss,
            stats.current_commit,
            stats.peak_commit,
            stats.page_faults,
        );
    }

    // Keep the allocations live until after the last read.
    drop(chunks);
}
