use std::time::Duration;

#[derive(Clone, Debug)]
/// The policy of a cache.
pub struct Policy {
    max_capacity: Option<u64>,
    time_to_live: Option<Duration>,
    time_to_idle: Option<Duration>,
    time_to_exist: Option<Duration>,
}

impl Policy {
    pub(crate) fn new(
        max_capacity: Option<u64>,
        time_to_live: Option<Duration>,
        time_to_idle: Option<Duration>,
        time_to_exist: Option<Duration>,
    ) -> Self {
        Self {
            max_capacity,
            time_to_live,
            time_to_idle,
            time_to_exist,
        }
    }

    /// Returns the `max_capacity` of the cache.
    pub fn max_capacity(&self) -> Option<u64> {
        self.max_capacity
    }

    /// Returns the `time_to_live` of the cache.
    pub fn time_to_live(&self) -> Option<Duration> {
        self.time_to_live
    }

    /// Returns the `time_to_idle` of the cache.
    pub fn time_to_idle(&self) -> Option<Duration> {
        self.time_to_idle
    }

    /// Returns the `time_to_exist` of the cache.
    pub fn time_to_exist(&self) -> Option<Duration> {
        self.time_to_exist
    }
}
