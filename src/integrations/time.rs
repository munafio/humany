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
        RelativeTime::from_unix_nanos(timestamp.unix_timestamp_nanos(), self.locale_tag())
    }
}
