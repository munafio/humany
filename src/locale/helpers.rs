//! Shared formatting helpers for locales with similar phrasing patterns.

use core::fmt;

use super::tables::LocaleTables;
use super::time_unit::TimeUnit;
use crate::options::write_u64;

/// English-style: `2 minutes ago` / `in 2 days`.
pub(crate) fn write_relative_in_out(
    f: &mut fmt::Formatter<'_>,
    tables: &LocaleTables,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    if past {
        if count == 1 {
            f.write_str(tables.unit_label(unit, 1))?;
        } else {
            write_u64(f, count)?;
            f.write_str(tables.unit_label(unit, count))?;
        }
        f.write_str(tables.ago)
    } else {
        f.write_str(tables.in_prefix)?;
        write_u64(f, count)?;
        f.write_str(unit.unit_other_label(tables))?;
        f.write_str(tables.in_suffix)
    }
}

/// `{prefix}{count}{unit_phrase}` for past; future uses table `in_prefix` / `in_suffix`.
pub(crate) fn write_relative_past_prefix(
    f: &mut fmt::Formatter<'_>,
    tables: &LocaleTables,
    past_prefix: &'static str,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    if past {
        f.write_str(past_prefix)?;
        if count == 1 {
            f.write_str(tables.unit_label(unit, 1))?;
        } else {
            write_u64(f, count)?;
            f.write_str(tables.unit_label(unit, count))?;
        }
        Ok(())
    } else {
        f.write_str(tables.in_prefix)?;
        write_u64(f, count)?;
        f.write_str(unit.unit_other_label(tables))?;
        f.write_str(tables.in_suffix)
    }
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
