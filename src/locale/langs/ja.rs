//! Japanese locale.

use core::fmt;

use crate::locale::helpers;
use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    ago: "前",
    in_prefix: "",
    in_suffix: "後",
    second_one: "1秒",
    second_other: "秒",
    minute_one: "1分",
    minute_other: "分",
    hour_one: "1時間",
    hour_other: "時間",
    day_one: "1日",
    day_other: "日",
    duration_second_short: "秒",
    duration_minute_short: "分",
    duration_hour_short: "時間",
    duration_day_short: "日",
    duration_second_long_one: "秒",
    duration_second_long_other: "秒",
    duration_minute_long_one: "分",
    duration_minute_long_other: "分",
    duration_hour_long_one: "時間",
    duration_hour_long_other: "時間",
    duration_day_long_one: "日",
    duration_day_long_other: "日",
};

fn unit_phrase(unit: TimeUnit) -> &'static str {
    match unit {
        TimeUnit::Second => "秒",
        TimeUnit::Minute => "分",
        TimeUnit::Hour => "時間",
        TimeUnit::Day => "日",
    }
}

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    helpers::write_relative_east_asian(f, past, count, unit_phrase(unit), "前", "後")
}
