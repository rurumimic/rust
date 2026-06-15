#[cfg(feature = "stats")]
mod mimalloc;
#[cfg(feature = "stats")]
use mimalloc::Inner;

#[cfg(not(feature = "stats"))]
mod noop;
#[cfg(not(feature = "stats"))]
use noop::Inner;

/// Metrics collection handle.
///
/// `Inner` is selected at compile time (mimalloc vs. noop) and kept private.
///
/// Unlike jemalloc's `tikv-jemalloc-ctl` — whose MIB lookups and epoch reads can
/// fail, forcing a fallible API and runtime degradation — mimalloc's
/// `mi_process_info` is infallible. There is therefore no error type, no
/// `Option`, and no epoch here: `read()` always returns live values. That gap is
/// the point, not an oversight; mimalloc's Rust binding simply exposes far less
/// introspection than jemalloc does.
pub struct Metrics {
    inner: Inner,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            inner: Inner::new(),
        }
    }

    pub fn backend(&self) -> &'static str {
        self.inner.backend()
    }

    pub fn read(&self) -> Stats {
        self.inner.read()
    }
}

/// Process-level memory figures from `mi_process_info`. All values are in bytes
/// except `page_faults`, which is a count.
///
/// These are coarser than jemalloc's per-size-class accounting: there is no
/// `allocated`/`active` byte-accurate breakdown, only what the OS and mimalloc
/// report for the whole process.
#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    /// Current resident set size (maps to RSS).
    pub current_rss: usize,
    /// Peak resident set size observed so far.
    pub peak_rss: usize,
    /// Currently committed memory.
    pub current_commit: usize,
    /// Peak committed memory observed so far.
    pub peak_commit: usize,
    /// Hard page-fault count.
    pub page_faults: usize,
}
