//! English locale.

use core::fmt;

use crate::locale::helpers;
use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    past_prefix: "",
    ago: " ago",
    in_prefix: "in ",
    in_suffix: "",
    second_one: "1 second",
    second_other: " seconds",
    minute_one: "1 minute",
    minute_other: " minutes",
    hour_one: "1 hour",
    hour_other: " hours",
    day_one: "1 day",
    day_other: " days",
    duration_second_short: "s",
    duration_minute_short: "m",
    duration_hour_short: "h",
    duration_day_short: "d",
    duration_second_long_one: "second",
    duration_second_long_other: "seconds",
    duration_minute_long_one: "minute",
    duration_minute_long_other: "minutes",
    duration_hour_long_one: "hour",
    duration_hour_long_other: "hours",
    duration_day_long_one: "day",
    duration_day_long_other: "days",
};

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    helpers::write_relative_in_out(f, &TABLES, past, unit, count)
}
