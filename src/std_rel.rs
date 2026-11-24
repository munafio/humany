//! `std::time::SystemTime` relative helpers.

use core::fmt;

use crate::locale::Locale;
use crate::relative::Relative;

/// Relative time from a wall-clock timestamp (`std` feature).
#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug)]
pub struct RelativeTime {
    timestamp_nanos: i128,
    reference: Option<std::time::SystemTime>,
    locale: Locale,
    compact: bool,
}

/// Relative time from `timestamp` compared to now ([`crate::Humany::DEFAULT`]).
#[cfg(feature = "std")]
pub fn relative(timestamp: std::time::SystemTime) -> RelativeTime {
    crate::Humany::DEFAULT.relative(timestamp)
}

#[cfg(feature = "std")]
impl RelativeTime {
    pub(crate) fn new(timestamp: std::time::SystemTime, locale: Locale) -> Self {
        Self::from_unix_nanos(unix_nanoseconds(timestamp), locale)
    }

    pub(crate) fn from_unix_nanos(timestamp_nanos: i128, locale: Locale) -> Self {
        Self {
            timestamp_nanos,
            reference: None,
            locale,
            compact: false,
        }
    }

    /// Fixed reference instant instead of `SystemTime::now()` when displayed.
    pub fn reference(mut self, reference: std::time::SystemTime) -> Self {
        self.reference = Some(reference);
        self
    }

    /// Override locale when displayed.
    pub const fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Compact output (`2m ago`).
    pub const fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Verbose output (`2 minutes ago`).
    pub const fn verbose(mut self) -> Self {
        self.compact = false;
        self
    }
}

#[cfg(feature = "std")]
impl fmt::Display for RelativeTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reference = self.reference.unwrap_or_else(std::time::SystemTime::now);
        let delta = unix_nanoseconds(reference) - self.timestamp_nanos;
        let seconds = (delta.abs() / 1_000_000_000).try_into().unwrap_or(u64::MAX);
        let inner = Relative::from_parts(seconds, delta >= 0, self.locale);
        let inner = if self.compact {
            inner.compact()
        } else {
            inner.verbose()
        };
        inner.fmt(f)
    }
}

#[cfg(feature = "std")]
fn unix_nanoseconds(timestamp: std::time::SystemTime) -> i128 {
    match timestamp.duration_since(std::time::UNIX_EPOCH) {
        Ok(duration) => duration.as_nanos() as i128,
        Err(error) => -(error.duration().as_nanos() as i128),
    }
}
