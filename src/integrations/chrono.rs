//! Optional [`chrono`] integration.

use chrono::{DateTime, Utc};

use crate::std_rel::RelativeTime;
use crate::Humany;

/// Relative time for a UTC [`DateTime`].
pub fn relative_chrono(timestamp: DateTime<Utc>) -> RelativeTime {
    Humany::DEFAULT.relative_chrono(timestamp)
}

impl Humany {
    /// Relative time for a UTC [`DateTime`].
    pub fn relative_chrono(self, timestamp: DateTime<Utc>) -> RelativeTime {
        self.relative(to_system_time(timestamp))
    }
}

fn to_system_time(dt: DateTime<Utc>) -> std::time::SystemTime {
    std::time::UNIX_EPOCH
        + std::time::Duration::from_secs(dt.timestamp().max(0) as u64)
        + std::time::Duration::from_nanos(dt.timestamp_subsec_nanos() as u64)
}
