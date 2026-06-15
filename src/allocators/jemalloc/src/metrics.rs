#[cfg(not(target_env = "msvc"))]
mod jemalloc;
#[cfg(not(target_env = "msvc"))]
use jemalloc::Inner;

#[cfg(target_env = "msvc")]
mod noop;
#[cfg(target_env = "msvc")]
use noop::Inner;

/// Metrics collection handle.
///
/// `Inner` is selected at compile time (jemalloc vs. noop) and kept private;
/// this type layers runtime degradation on top: if initialization fails, we
/// log once and carry on disabled (`inner == None`) instead of taking the
/// whole app down. `Inner` and `MetricsError` never leak into the public API.
pub struct Metrics {
    inner: Option<Inner>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    pub fn new() -> Self {
        let inner = match Inner::try_new() {
            Ok(inner) => Some(inner),
            Err(error) => {
                log::warn!("metrics disabled: {error}");
                None
            }
        };

        Self { inner }
    }

    pub fn backend(&self) -> &'static str {
        match &self.inner {
            Some(inner) => inner.backend(),
            None => "disabled",
        }
    }

    pub fn read(&self) -> Option<Stats> {
        self.inner
            .as_ref()?
            .read()
            .inspect_err(|error| log::warn!("metrics read failed: {error}"))
            .ok()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub allocated: usize,
    pub active: usize,
    pub resident: usize,
    pub mapped: usize,
    pub retained: usize,
    pub metadata: usize,
}

#[derive(Debug)]
pub(crate) enum MetricsError {
    Jemalloc(String),
}

impl std::fmt::Display for MetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetricsError::Jemalloc(message) => write!(f, "jemalloc metrics error: {message}"),
        }
    }
}

impl std::error::Error for MetricsError {}
