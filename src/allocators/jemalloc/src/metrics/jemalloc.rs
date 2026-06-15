use tikv_jemalloc_ctl::{epoch, stats};

use crate::{Stats, metrics::MetricsError};

pub(crate) struct Inner {
    epoch: tikv_jemalloc_ctl::epoch_mib,
    allocated: tikv_jemalloc_ctl::stats::allocated_mib,
    active: tikv_jemalloc_ctl::stats::active_mib,
    resident: tikv_jemalloc_ctl::stats::resident_mib,
    mapped: tikv_jemalloc_ctl::stats::mapped_mib,
    retained: tikv_jemalloc_ctl::stats::retained_mib,
    metadata: tikv_jemalloc_ctl::stats::metadata_mib,
}

fn to_error(error: tikv_jemalloc_ctl::Error) -> MetricsError {
    MetricsError::Jemalloc(error.to_string())
}

impl Inner {
    pub(crate) fn try_new() -> Result<Self, MetricsError> {
        Ok(Self {
            epoch: epoch::mib().map_err(to_error)?,
            allocated: stats::allocated::mib().map_err(to_error)?,
            active: stats::active::mib().map_err(to_error)?,
            resident: stats::resident::mib().map_err(to_error)?,
            mapped: stats::mapped::mib().map_err(to_error)?,
            retained: stats::retained::mib().map_err(to_error)?,
            metadata: stats::metadata::mib().map_err(to_error)?,
        })
    }

    pub(crate) fn backend(&self) -> &'static str {
        "jemalloc"
    }

    pub(crate) fn read(&self) -> Result<Stats, MetricsError> {
        self.epoch.advance().map_err(to_error)?;

        Ok(Stats {
            allocated: self.allocated.read().map_err(to_error)?,
            active: self.active.read().map_err(to_error)?,
            resident: self.resident.read().map_err(to_error)?,
            mapped: self.mapped.read().map_err(to_error)?,
            retained: self.retained.read().map_err(to_error)?,
            metadata: self.metadata.read().map_err(to_error)?,
        })
    }
}
