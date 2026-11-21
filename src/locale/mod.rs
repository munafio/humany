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