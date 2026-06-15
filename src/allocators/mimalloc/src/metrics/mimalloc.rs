use crate::Stats;

pub(crate) struct Inner {}

impl Inner {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn backend(&self) -> &'static str {
        "mimalloc"
    }

    pub(crate) fn read(&self) -> Stats {
        // `mi_process_info` writes eight `usize` out-params. We only surface the
        // memory-related ones; the timing fields (elapsed/user/system) are read
        // into throwaways so the call stays well-formed.
        let mut elapsed_msecs = 0usize;
        let mut user_msecs = 0usize;
        let mut system_msecs = 0usize;
        let mut current_rss = 0usize;
        let mut peak_rss = 0usize;
        let mut current_commit = 0usize;
        let mut peak_commit = 0usize;
        let mut page_faults = 0usize;

        // SAFETY: every argument is a valid, unique, writable pointer to a live
        // `usize`. `mi_process_info` only writes through them during the call and
        // does not retain them afterwards, and it has no failure mode.
        unsafe {
            libmimalloc_sys::mi_process_info(
                &mut elapsed_msecs,
                &mut user_msecs,
                &mut system_msecs,
                &mut current_rss,
                &mut peak_rss,
                &mut current_commit,
                &mut peak_commit,
                &mut page_faults,
            );
        }

        Stats {
            current_rss,
            peak_rss,
            current_commit,
            peak_commit,
            page_faults,
        }
    }
}
