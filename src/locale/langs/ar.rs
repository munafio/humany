//! Arabic locale.

use core::fmt;

use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;
use crate::options::write_u64;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "٪",
    past_prefix: "منذ ",
    ago: "",
    in_prefix: "خلال ",
    in_suffix: "",
    second_one: "منذ ثانية واحدة",
    second_other: "منذ ثانية",
    minute_one: "منذ دقيقة واحدة",
    minute_other: "منذ دقيقة",
    hour_one: "منذ ساعة واحدة",
    hour_other: "منذ ساعة",
    day_one: "منذ يوم واحد",
    day_other: "منذ يوم",
    duration_second_short: "s",
    duration_minute_short: "m",
    duration_hour_short: "h",
    duration_day_short: "d",
    duration_second_long_one: "ثانية",
    duration_second_long_other: "ثوان",
    duration_minute_long_one: "دقيقة",
    duration_minute_long_other: "دقائق",
    duration_hour_long_one: "ساعة",
    duration_hour_long_other: "ساعات",
    duration_day_long_one: "يوم",
    duration_day_long_other: "أيام",
};

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    if past && count == 1 {
        return f.write_str(TABLES.unit_label(unit, count));
    }

    f.write_str(if past {
        TABLES.past_prefix
    } else {
        TABLES.in_prefix
    })?;
    if count == 2 {
        return f.write_str(match unit {
            TimeUnit::Second => "ثانيتين",
            TimeUnit::Minute => "دقيقتين",
            TimeUnit::Hour => "ساعتين",
            TimeUnit::Day => "يومين",
        });
    }

    write_u64(f, count)?;
    f.write_str(" ")?;
    f.write_str(unit_label(unit, count))
}

pub(crate) fn unit_label(unit: TimeUnit, count: u64) -> &'static str {
    let (singular, plural) = match unit {
        TimeUnit::Second => ("ثانية", "ثوان"),
        TimeUnit::Minute => ("دقيقة", "دقائق"),
        TimeUnit::Hour => ("ساعة", "ساعات"),
        TimeUnit::Day => ("يوم", "أيام"),
    };
    if count == 0 || matches!(count % 100, 3..=10) {
        plural
    } else {
        singular
    }
}
