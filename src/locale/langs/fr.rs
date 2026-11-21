//! French locale.

use core::fmt;

use crate::locale::helpers;
use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    ago: "",
    in_prefix: "dans ",
    in_suffix: "",
    second_one: "1 seconde",
    second_other: " secondes",
    minute_one: "1 minute",
    minute_other: " minutes",
    hour_one: "1 heure",
    hour_other: " heures",
    day_one: "1 jour",
    day_other: " jours",
    duration_second_short: "s",
    duration_minute_short: "m",
    duration_hour_short: "h",
    duration_day_short: "j",
    duration_second_long_one: "seconde",
    duration_second_long_other: "secondes",
    duration_minute_long_one: "minute",
    duration_minute_long_other: "minutes",
    duration_hour_long_one: "heure",
    duration_hour_long_other: "heures",
    duration_day_long_one: "jour",
    duration_day_long_other: "jours",
};

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    helpers::write_relative_past_prefix(f, &TABLES, "il y a ", past, unit, count)
}
