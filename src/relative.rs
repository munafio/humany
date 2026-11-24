//! Relative time from a signed second delta.

use core::fmt;

use crate::context::Humany;
use crate::locale::{write_relative, Locale, TimeUnit};
use crate::options::write_u64;

/// Human-readable relative time from a signed second delta (positive = past).
#[derive(Clone, Copy, Debug)]
pub struct Relative {
    seconds: u64,
    past: bool,
    locale: Locale,
    compact: bool,
}

/// Format a delta in seconds with [`Humany::DEFAULT`] (verbose English).
pub fn relative_secs(delta_secs: i64) -> Relative {
    Humany::DEFAULT.relative_secs(delta_secs)
}

/// Relative time between two UNIX timestamps in seconds (`later - earlier`).
pub fn relative_between(earlier_secs: u64, later_secs: u64) -> Relative {
    Relative::from_parts(
        later_secs.abs_diff(earlier_secs),
        later_secs >= earlier_secs,
        Locale::En,
    )
}

impl Relative {
    pub(crate) fn new(delta_secs: i64, locale: Locale) -> Self {
        Self::from_parts(delta_secs.unsigned_abs(), delta_secs >= 0, locale)
    }

    pub(crate) const fn from_parts(seconds: u64, past: bool, locale: Locale) -> Self {
        Self {
            seconds,
            past,
            locale,
            compact: false,
        }
    }

    /// Override locale for this value.
    pub const fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Use compact units (`2m ago` instead of `2 minutes ago`).
    pub const fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Use full unit phrases (`2 minutes ago`).
    pub const fn verbose(mut self) -> Self {
        self.compact = false;
        self
    }
}

impl fmt::Display for Relative {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (unit, count) = pick_unit(self.seconds);

        if self.compact {
            write_compact_relative(f, self.past, unit, count, self.locale)
        } else {
            write_relative(f, self.locale, self.past, unit, count)
        }
    }
}

fn pick_unit(abs_secs: u64) -> (TimeUnit, u64) {
    const MIN: u64 = 60;
    const HOUR: u64 = 3600;
    const DAY: u64 = 86400;

    if abs_secs >= DAY {
        (TimeUnit::Day, abs_secs / DAY)
    } else if abs_secs >= HOUR {
        (TimeUnit::Hour, abs_secs / HOUR)
    } else if abs_secs >= MIN {
        (TimeUnit::Minute, abs_secs / MIN)
    } else {
        (TimeUnit::Second, abs_secs)
    }
}

fn write_compact_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
    locale: Locale,
) -> fmt::Result {
    let tables = locale.tables();
    f.write_str(if past {
        tables.past_prefix
    } else {
        tables.in_prefix
    })?;
    write_u64(f, count)?;
    let suffix = match unit {
        TimeUnit::Second => tables.duration_second_short,
        TimeUnit::Minute => tables.duration_minute_short,
        TimeUnit::Hour => tables.duration_hour_short,
        TimeUnit::Day => tables.duration_day_short,
    };
    f.write_str(suffix)?;
    if past {
        f.write_str(tables.ago)
    } else {
        f.write_str(tables.in_suffix)
    }
}
