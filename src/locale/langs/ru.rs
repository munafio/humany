//! Russian locale.

use core::fmt;

use crate::locale::tables::LocaleTables;
use crate::locale::time_unit::TimeUnit;
use crate::options::write_u64;

pub(crate) const TABLES: LocaleTables = LocaleTables {
    percent_symbol: "%",
    past_prefix: "",
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
    if !past {
        f.write_str(TABLES.in_prefix)?;
    }
    write_u64(f, count)?;
    f.write_str(" ")?;
    f.write_str(unit_label(unit, count, true))?;
    if past {
        f.write_str(TABLES.ago)?;
    }
    Ok(())
}

pub(crate) fn unit_label(unit: TimeUnit, count: u64, relative: bool) -> &'static str {
    let form = plural_form(count);
    if relative && form == 0 {
        match unit {
            TimeUnit::Second => return "секунду",
            TimeUnit::Minute => return "минуту",
            TimeUnit::Hour | TimeUnit::Day => {}
        }
    }
    let labels = match unit {
        TimeUnit::Second => ["секунда", "секунды", "секунд"],
        TimeUnit::Minute => ["минута", "минуты", "минут"],
        TimeUnit::Hour => ["час", "часа", "часов"],
        TimeUnit::Day => ["день", "дня", "дней"],
    };
    labels[form]
}

pub(crate) fn plural_form(count: u64) -> usize {
    match (count % 10, count % 100) {
        (1, 11) | (2..=4, 12..=14) => 2,
        (1, _) => 0,
        (2..=4, _) => 1,
        _ => 2,
    }
}
