use crate::Stats;

pub(crate) struct Inner {}

impl Inner {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn backend(&self) -> &'static str {
        // mimalloc is still the global allocator here; only introspection (the
        // `stats` feature) is off, so we cannot report `system`.
        "mimalloc (stats disabled)"
    }

    pub(crate) fn read(&self) -> Stats {
        Stats::default()
    }
}
