//! Shared formatting helpers for locales with similar phrasing patterns.

use core::fmt;

use super::tables::LocaleTables;
use super::time_unit::TimeUnit;
use crate::options::write_u64;

/// Relative time using a prefix and suffix around a counted unit.
pub(crate) fn write_relative_in_out(
    f: &mut fmt::Formatter<'_>,
    tables: &LocaleTables,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    f.write_str(if past {
        tables.past_prefix
    } else {
        tables.in_prefix
    })?;
    if count != 1 {
        write_u64(f, count)?;
    }
    f.write_str(tables.unit_label(unit, count))?;
    f.write_str(if past { tables.ago } else { tables.in_suffix })
}

/// `{count}{unit}{suffix}` without spaces (Chinese, Japanese).
pub(crate) fn write_relative_east_asian(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    count: u64,
    unit_phrase: &'static str,
    past_suffix: &'static str,
    future_suffix: &'static str,
) -> fmt::Result {
    write_u64(f, count)?;
    f.write_str(unit_phrase)?;
    if past {
        f.write_str(past_suffix)
    } else {
        f.write_str(future_suffix)
    }
}
