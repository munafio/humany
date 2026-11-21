//! Optional [`time`] integration.

use time::OffsetDateTime;

use crate::std_rel::RelativeTime;
use crate::Humany;

/// Relative time for an [`OffsetDateTime`].
pub fn relative_time(timestamp: OffsetDateTime) -> RelativeTime {
    Humany::DEFAULT.relative_time(timestamp)
}

impl Humany {
    /// Relative time for an [`OffsetDateTime`].
    pub fn relative_time(self, timestamp: OffsetDateTime) -> RelativeTime {
        self.relative(to_system_time(timestamp))
    }
}

fn to_system_time(dt: OffsetDateTime) -> std::time::SystemTime {
    let secs = dt.unix_timestamp().max(0) as u64;
    let nanos = dt.nanosecond() as u64;
    std::time::UNIX_EPOCH + std::time::Duration::new(secs, nanos as u32)
}
