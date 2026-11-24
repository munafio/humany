//! Locale definitions and relative-time phrasing.
//!
//! Shared logic lives in this directory; translations live in [`langs`].

mod helpers;
mod langs;
mod tables;
mod time_unit;

pub(crate) use tables::LocaleTables;
pub(crate) use time_unit::TimeUnit;

use core::fmt;

use crate::duration::DurationUnit;

/// Supported locales.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Locale {
    /// English.
    #[default]
    En,
    /// Arabic.
    Ar,
    /// French.
    Fr,
    /// Spanish.
    Es,
    /// German.
    De,
    /// Chinese (Simplified).
    Zh,
    /// Japanese.
    Ja,
    /// Portuguese.
    Pt,
    /// Russian.
    Ru,
}

impl Locale {
    pub(crate) const fn decimal_separator(self) -> char {
        match self {
            Locale::Fr | Locale::Es | Locale::De | Locale::Pt | Locale::Ru => ',',
            Locale::Ar => '٫',
            Locale::En | Locale::Zh | Locale::Ja => '.',
        }
    }

    pub(crate) fn short_duration_label(self, unit: DurationUnit) -> &'static str {
        let tables = self.tables();
        match unit {
            DurationUnit::Day => tables.duration_day_short,
            DurationUnit::Hour => tables.duration_hour_short,
            DurationUnit::Minute => tables.duration_minute_short,
            DurationUnit::Second => tables.duration_second_short,
            DurationUnit::Millisecond => "ms",
            DurationUnit::Microsecond => "µs",
            DurationUnit::Nanosecond => "ns",
        }
    }

    pub(crate) fn long_duration_label(self, unit: DurationUnit, count: u64) -> &'static str {
        let time_unit = match unit {
            DurationUnit::Day => TimeUnit::Day,
            DurationUnit::Hour => TimeUnit::Hour,
            DurationUnit::Minute => TimeUnit::Minute,
            DurationUnit::Second => TimeUnit::Second,
            _ => return self.long_subsecond_label(unit, count),
        };

        match self {
            Self::Ru => return langs::ru::unit_label(time_unit, count, false),
            Self::Ar => return langs::ar::unit_label(time_unit, count),
            _ => {}
        }
        let tables = self.tables();
        let labels = match time_unit {
            TimeUnit::Day => [tables.duration_day_long_one, tables.duration_day_long_other],
            TimeUnit::Hour => [
                tables.duration_hour_long_one,
                tables.duration_hour_long_other,
            ],
            TimeUnit::Minute => [
                tables.duration_minute_long_one,
                tables.duration_minute_long_other,
            ],
            TimeUnit::Second => [
                tables.duration_second_long_one,
                tables.duration_second_long_other,
            ],
        };
        labels[usize::from(count != 1)]
    }

    fn long_subsecond_label(self, unit: DurationUnit, count: u64) -> &'static str {
        let index = unit as usize - DurationUnit::Millisecond as usize;
        let labels = match self {
            Self::En => [
                ["millisecond", "milliseconds"],
                ["microsecond", "microseconds"],
                ["nanosecond", "nanoseconds"],
            ],
            Self::Ar => [
                ["مللي ثانية", "مللي ثانية"],
                ["ميكروثانية", "ميكروثانية"],
                ["نانوثانية", "نانوثانية"],
            ],
            Self::Fr => [
                ["milliseconde", "millisecondes"],
                ["microseconde", "microsecondes"],
                ["nanoseconde", "nanosecondes"],
            ],
            Self::Es => [
                ["milisegundo", "milisegundos"],
                ["microsegundo", "microsegundos"],
                ["nanosegundo", "nanosegundos"],
            ],
            Self::De => [
                ["Millisekunde", "Millisekunden"],
                ["Mikrosekunde", "Mikrosekunden"],
                ["Nanosekunde", "Nanosekunden"],
            ],
            Self::Zh => [["毫秒"; 2], ["微秒"; 2], ["纳秒"; 2]],
            Self::Ja => [["ミリ秒"; 2], ["マイクロ秒"; 2], ["ナノ秒"; 2]],
            Self::Pt => [
                ["milissegundo", "milissegundos"],
                ["microssegundo", "microssegundos"],
                ["nanossegundo", "nanossegundos"],
            ],
            Self::Ru => {
                let labels = [
                    ["миллисекунда", "миллисекунды", "миллисекунд"],
                    ["микросекунда", "микросекунды", "микросекунд"],
                    ["наносекунда", "наносекунды", "наносекунд"],
                ];
                return labels[index][langs::ru::plural_form(count)];
            }
        };
        labels[index][usize::from(count != 1)]
    }

    /// Static string tables for duration, compact relative suffixes, and symbols.
    pub(crate) fn tables(self) -> &'static LocaleTables {
        match self {
            Locale::En => &langs::en::TABLES,
            Locale::Ar => &langs::ar::TABLES,
            Locale::Fr => &langs::fr::TABLES,
            Locale::Es => &langs::es::TABLES,
            Locale::De => &langs::de::TABLES,
            Locale::Zh => &langs::zh::TABLES,
            Locale::Ja => &langs::ja::TABLES,
            Locale::Pt => &langs::pt::TABLES,
            Locale::Ru => &langs::ru::TABLES,
        }
    }
}

/// Verbose relative phrase (`5 minutes ago`, `il y a 5 minutes`, …).
pub(crate) fn write_relative(
    f: &mut fmt::Formatter<'_>,
    locale: Locale,
    past: bool,
    unit: TimeUnit,
    count: u64,
) -> fmt::Result {
    match locale {
        Locale::En => langs::en::write_relative(f, past, unit, count),
        Locale::Ar => langs::ar::write_relative(f, past, unit, count),
        Locale::Fr => langs::fr::write_relative(f, past, unit, count),
        Locale::Es => langs::es::write_relative(f, past, unit, count),
        Locale::De => langs::de::write_relative(f, past, unit, count),
        Locale::Zh => langs::zh::write_relative(f, past, unit, count),
        Locale::Ja => langs::ja::write_relative(f, past, unit, count),
        Locale::Pt => langs::pt::write_relative(f, past, unit, count),
        Locale::Ru => langs::ru::write_relative(f, past, unit, count),
    }
}
