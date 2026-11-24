//! German locale.

use core::fmt;

use crate::locale::helpers;
use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    past_prefix: "vor ",
    ago: "",
    in_prefix: "in ",
    in_suffix: "",
    second_one: "1 Sekunde",
    second_other: " Sekunden",
    minute_one: "1 Minute",
    minute_other: " Minuten",
    hour_one: "1 Stunde",
    hour_other: " Stunden",
    day_one: "1 Tag",
    day_other: " Tage",
    duration_second_short: "s",
    duration_minute_short: "m",
    duration_hour_short: "h",
    duration_day_short: "T",
    duration_second_long_one: "Sekunde",
    duration_second_long_other: "Sekunden",
    duration_minute_long_one: "Minute",
    duration_minute_long_other: "Minuten",
    duration_hour_long_one: "Stunde",
    duration_hour_long_other: "Stunden",
    duration_day_long_one: "Tag",
    duration_day_long_other: "Tage",
};

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    helpers::write_relative_in_out(f, &TABLES, past, unit, count)
}
