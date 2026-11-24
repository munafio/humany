//! Chinese (Simplified) locale.

use core::fmt;

use crate::locale::helpers;
use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    past_prefix: "",
    ago: "前",
    in_prefix: "",
    in_suffix: "后",
    second_one: "1秒",
    second_other: "秒",
    minute_one: "1分钟",
    minute_other: "分钟",
    hour_one: "1小时",
    hour_other: "小时",
    day_one: "1天",
    day_other: "天",
    duration_second_short: "秒",
    duration_minute_short: "分",
    duration_hour_short: "时",
    duration_day_short: "天",
    duration_second_long_one: "秒",
    duration_second_long_other: "秒",
    duration_minute_long_one: "分钟",
    duration_minute_long_other: "分钟",
    duration_hour_long_one: "小时",
    duration_hour_long_other: "小时",
    duration_day_long_one: "天",
    duration_day_long_other: "天",
};

fn unit_phrase(unit: TimeUnit) -> &'static str {
    match unit {
        TimeUnit::Second => "秒",
        TimeUnit::Minute => "分钟",
        TimeUnit::Hour => "小时",
        TimeUnit::Day => "天",
    }
}

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    helpers::write_relative_east_asian(f, past, count, unit_phrase(unit), "前", "后")
}
