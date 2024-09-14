#[derive(Debug, Default)]
pub struct TimeoutError(());

impl TimeoutError {
    pub fn new() -> Self {
        TimeoutError(())
    }
}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad("request timed out")
    }
}

impl std::error::Error for TimeoutError {}

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;
