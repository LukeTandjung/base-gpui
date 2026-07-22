use std::time::Duration;

use crate::toast::{TOAST_DEFAULT_LIMIT, TOAST_DEFAULT_TIMEOUT};

/// Provider configuration: the default auto-dismiss timeout (5000 ms) and the
/// visible-toast limit (3) synced into the runtime every render.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ToastProviderProps {
    timeout: Duration,
    limit: usize,
}

impl Default for ToastProviderProps {
    fn default() -> Self {
        Self {
            timeout: TOAST_DEFAULT_TIMEOUT,
            limit: TOAST_DEFAULT_LIMIT,
        }
    }
}

impl ToastProviderProps {
    pub fn new(timeout: Duration, limit: usize) -> Self {
        Self { timeout, limit }
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }

    pub fn limit(&self) -> usize {
        self.limit
    }
}
