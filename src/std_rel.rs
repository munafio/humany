//! `std::time::SystemTime` relative helpers.

use core::fmt;

use crate::locale::Locale;
use crate::options::FormatOptions;
use crate::relative::Relative;

/// Relative time from a wall-clock timestamp (`std` feature).
#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug)]
pub struct RelativeTime {
    timestamp: std::time::SystemTime,
    reference: Option<std::time::SystemTime>,
    options: FormatOptions,
    locale: Locale,
    compact: bool,
}

/// Relative time from `timestamp` compared to now ([`Humany::DEFAULT`]).
#[cfg(feature = "std")]
pub fn relative(timestamp: std::time::SystemTime) -> RelativeTime {
    crate::Humany::DEFAULT.relative(timestamp)
}

#[cfg(feature = "std")]
impl RelativeTime {
    pub(crate) fn new(
        timestamp: std::time::SystemTime,
        options: FormatOptions,
        locale: Locale,
    ) -> Self {
        Self {
            timestamp,
            reference: None,
            options,
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
        let delta = seconds_between(self.timestamp, reference);
        let inner = Relative::new(delta, self.options, self.locale);
        let inner = if self.compact {
            inner.compact()
        } else {
            inner.verbose()
        };
        inner.fmt(f)
    }
}

#[cfg(feature = "std")]
pub(crate) fn seconds_between(a: std::time::SystemTime, b: std::time::SystemTime) -> i64 {
    match b.duration_since(a) {
        Ok(d) => d.as_secs() as i64,
        Err(e) => -(e.duration().as_secs() as i64),
    }
}
