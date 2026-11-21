//! Russian locale.

use core::fmt;

use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;
use crate::options::write_u64;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    ago: " назад",
    in_prefix: "через ",
    in_suffix: "",
    second_one: "1 секунду",
    second_other: " секунд",
    minute_one: "1 минуту",
    minute_other: " минут",
    hour_one: "1 час",
    hour_other: " часов",
    day_one: "1 день",
    day_other: " дней",
    duration_second_short: "s",
    duration_minute_short: "m",
    duration_hour_short: "h",
    duration_day_short: "d",
    duration_second_long_one: "секунду",
    duration_second_long_other: "секунд",
    duration_minute_long_one: "минуту",
    duration_minute_long_other: "минут",
    duration_hour_long_one: "час",
    duration_hour_long_other: "часов",
    duration_day_long_one: "день",
    duration_day_long_other: "дней",
};

pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    if past {
        if count == 1 {
            f.write_str(TABLES.unit_label(unit, 1))?;
        } else {
            write_u64(f, count)?;
            f.write_str(TABLES.unit_label(unit, count))?;
        }
        f.write_str(TABLES.ago)
    } else {
        f.write_str(TABLES.in_prefix)?;
        write_u64(f, count)?;
        f.write_str(unit.unit_other_label(&TABLES))?;
        f.write_str(TABLES.in_suffix)
    }
}
