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
        let nanos = i128::from(timestamp.timestamp()) * 1_000_000_000
            + i128::from(timestamp.timestamp_subsec_nanos());
        RelativeTime::from_unix_nanos(nanos, self.locale_tag())
    }
}
