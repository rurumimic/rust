use crate::{Stats, metrics::MetricsError};

pub(crate) struct Inner {}

impl Inner {
    pub(crate) fn try_new() -> Result<Self, MetricsError> {
        Ok(Self {})
    }

    pub(crate) fn backend(&self) -> &'static str {
        "system"
    }

    pub(crate) fn read(&self) -> Result<Stats, MetricsError> {
        Ok(Stats::default())
    }
}
