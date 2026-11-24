//! Spanish locale.

use core::fmt;

use crate::locale::helpers;
use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    past_prefix: "hace ",
    ago: "",
    in_prefix: "en ",
    in_suffix: "",
    second_one: "1 segundo",
    second_other: " segundos",
    minute_one: "1 minuto",
    minute_other: " minutos",
    hour_one: "1 hora",
    hour_other: " horas",
    day_one: "1 día",
    day_other: " días",
    duration_second_short: "s",
    duration_minute_short: "m",
    duration_hour_short: "h",
    duration_day_short: "d",
    duration_second_long_one: "segundo",
    duration_second_long_other: "segundos",
    duration_minute_long_one: "minuto",
    duration_minute_long_other: "minutos",
    duration_hour_long_one: "hora",
    duration_hour_long_other: "horas",
    duration_day_long_one: "día",
    duration_day_long_other: "días",
};

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    helpers::write_relative_in_out(f, &TABLES, past, unit, count)
}
