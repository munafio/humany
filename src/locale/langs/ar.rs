//! Arabic locale.

use core::fmt;

use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;
use crate::options::write_u64;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
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
    if past {
        if count == 1 {
            f.write_str(match unit {
                TimeUnit::Second => "منذ ثانية واحدة",
                TimeUnit::Minute => "منذ دقيقة واحدة",
                TimeUnit::Hour => "منذ ساعة واحدة",
                TimeUnit::Day => "منذ يوم واحد",
            })
        } else if count == 2 {
            f.write_str(match unit {
                TimeUnit::Second => "منذ ثانيتين",
                TimeUnit::Minute => "منذ دقيقتين",
                TimeUnit::Hour => "منذ ساعتين",
                TimeUnit::Day => "منذ يومين",
            })
        } else if count <= 10 {
            write_u64(f, count)?;
            f.write_str(match unit {
                TimeUnit::Second => " منذ ثوان",
                TimeUnit::Minute => " منذ دقائق",
                TimeUnit::Hour => " ساعات",
                TimeUnit::Day => " أيام",
            })
        } else {
            f.write_str("منذ ")?;
            write_u64(f, count)?;
            f.write_str(match unit {
                TimeUnit::Second => " ثانية",
                TimeUnit::Minute => " دقيقة",
                TimeUnit::Hour => " ساعة",
                TimeUnit::Day => " يوم",
            })
        }
    } else {
        f.write_str("خلال ")?;
        write_u64(f, count)?;
        f.write_str(match unit {
            TimeUnit::Second if count == 1 => " ثانية",
            TimeUnit::Second if count == 2 => " ثانيتين",
            TimeUnit::Second => " ثوان",
            TimeUnit::Minute if count == 1 => " دقيقة",
            TimeUnit::Minute if count == 2 => " دقيقتين",
            TimeUnit::Minute => " دقائق",
            TimeUnit::Hour if count == 1 => " ساعة",
            TimeUnit::Hour if count == 2 => " ساعتين",
            TimeUnit::Hour => " ساعات",
            TimeUnit::Day if count == 1 => " يوم",
            TimeUnit::Day if count == 2 => " يومين",
            TimeUnit::Day => " أيام",
        })
    }
}
