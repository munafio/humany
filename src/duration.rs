use core::fmt::{self, Write};
use core::time::Duration as CoreDuration;

use crate::context::Humany;
use crate::locale::{Locale, LocaleTables};
use crate::options::{write_u64, FormatOptions, RoundingMode};

/// Input to the duration formatter.
#[derive(Clone, Copy, Debug)]
pub enum DurationInput {
    /// Whole seconds.
    Secs(u64),
    /// Sub-second precision.
    Std(CoreDuration),
}

/// Human-readable duration.
#[derive(Clone, Copy, Debug)]
pub struct DurationFmt {
    input: DurationInput,
    options: FormatOptions,
    locale: Locale,
    compact: bool,
    max_units: Option<u8>,
    min_unit: DurationUnit,
    separator: char,
}

/// Smallest unit to include when formatting a duration.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DurationUnit {
    /// Days.
    Day,
    /// Hours.
    Hour,
    /// Minutes.
    Minute,
    /// Seconds.
    Second,
    /// Milliseconds.
    Millisecond,
    /// Microseconds.
    Microsecond,
    /// Nanoseconds.
    Nanosecond,
}

/// Format a duration using [`Humany::DEFAULT`] (compact style).
pub fn duration(input: impl Into<DurationInput>) -> DurationFmt {
    Humany::DEFAULT.duration(input)
}

impl From<u64> for DurationInput {
    fn from(secs: u64) -> Self {
        DurationInput::Secs(secs)
    }
}

impl From<CoreDuration> for DurationInput {
    fn from(d: CoreDuration) -> Self {
        DurationInput::Std(d)
    }
}

impl DurationFmt {
    pub(crate) fn new(input: DurationInput, options: FormatOptions, locale: Locale) -> Self {
        Self {
            input,
            options,
            locale,
            compact: true,
            max_units: None,
            min_unit: DurationUnit::Nanosecond,
            separator: ' ',
        }
    }

    /// Set fractional digits (used when scaling sub-second parts).
    pub const fn decimals(mut self, decimals: u8) -> Self {
        self.options = self.options.decimals(decimals);
        self
    }

    /// Override locale for this value.
    pub const fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Limit how many non-zero components are shown (largest first).
    pub const fn max_units(mut self, n: u8) -> Self {
        self.max_units = Some(n);
        self
    }

    /// Smallest unit to emit; smaller components are dropped.
    pub const fn min_unit(mut self, unit: DurationUnit) -> Self {
        self.min_unit = unit;
        self
    }

    /// Use long unit names (`5 seconds` instead of `5s`).
    pub const fn long(mut self) -> Self {
        self.compact = false;
        self
    }

    /// Use compact unit suffixes (`5s`, `1h 1m`).
    pub const fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    /// Separator between components in multi-part output.
    pub const fn separator(mut self, separator: char) -> Self {
        self.separator = separator;
        self
    }

    /// Set rounding for any scaled sub-second parts.
    pub const fn rounding(mut self, rounding: RoundingMode) -> Self {
        self.options = self.options.rounding(rounding);
        self
    }
}

impl fmt::Display for DurationFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tables = self.locale.tables();
        let total_nanos = total_nanoseconds(self.input);
        if total_nanos == 0 {
            return if self.compact {
                f.write_str("0s")
            } else {
                write_long_single(f, tables, 0, DurationUnit::Second)
            };
        }

        let mut parts = decompose(total_nanos);

        trim_below_min(&mut parts, self.min_unit);
        if let Some(max) = self.max_units {
            truncate_max_units(&mut parts, max);
        }

        if self.compact {
            write_compact(f, &parts, tables, self.separator)
        } else {
            write_long(f, &parts, tables, self.separator)
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Parts {
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
    millis: u64,
    micros: u64,
    nanos: u64,
}

fn total_nanoseconds(input: DurationInput) -> u128 {
    match input {
        DurationInput::Secs(s) => (s as u128) * 1_000_000_000,
        DurationInput::Std(d) => (d.as_secs() as u128) * 1_000_000_000 + d.subsec_nanos() as u128,
    }
}

fn decompose(total_nanos: u128) -> Parts {
    const DAY: u128 = 86_400_000_000_000;
    const HOUR: u128 = 3_600_000_000_000;
    const MIN: u128 = 60_000_000_000;
    const SEC: u128 = 1_000_000_000;
    const MS: u128 = 1_000_000;
    const US: u128 = 1_000;

    let mut n = total_nanos;
    let days = (n / DAY) as u64;
    n %= DAY;
    let hours = (n / HOUR) as u64;
    n %= HOUR;
    let minutes = (n / MIN) as u64;
    n %= MIN;
    let seconds = (n / SEC) as u64;
    n %= SEC;
    let millis = (n / MS) as u64;
    n %= MS;
    let micros = (n / US) as u64;
    n %= US;
    let nanos = n as u64;
    Parts {
        days,
        hours,
        minutes,
        seconds,
        millis,
        micros,
        nanos,
    }
}

fn trim_below_min(parts: &mut Parts, min: DurationUnit) {
    match min {
        DurationUnit::Day => {}
        DurationUnit::Hour => parts.millis = 0,
        DurationUnit::Minute => {
            parts.millis = 0;
            parts.seconds = 0;
        }
        DurationUnit::Second => {
            parts.millis = 0;
            parts.micros = 0;
            parts.nanos = 0;
        }
        DurationUnit::Millisecond => {
            parts.micros = 0;
            parts.nanos = 0;
        }
        DurationUnit::Microsecond => parts.nanos = 0,
        DurationUnit::Nanosecond => {}
    }
    if min > DurationUnit::Second {
        // no op for day/hour/minute trimming of subcomponents
    }
    if matches!(min, DurationUnit::Minute) {
        parts.seconds = 0;
        parts.millis = 0;
        parts.micros = 0;
        parts.nanos = 0;
    }
    if matches!(min, DurationUnit::Hour) {
        parts.minutes = 0;
        parts.seconds = 0;
        parts.millis = 0;
        parts.micros = 0;
        parts.nanos = 0;
    }
    if matches!(min, DurationUnit::Day) {
        parts.hours = 0;
        parts.minutes = 0;
        parts.seconds = 0;
        parts.millis = 0;
        parts.micros = 0;
        parts.nanos = 0;
    }
}

fn truncate_max_units(parts: &mut Parts, max: u8) {
    let values = [
        parts.days,
        parts.hours,
        parts.minutes,
        parts.seconds,
        parts.millis,
        parts.micros,
        parts.nanos,
    ];
    let mut seen = 0u8;
    let mut keep = [false; 7];
    for (i, v) in values.iter().enumerate() {
        if *v > 0 {
            seen += 1;
            keep[i] = seen <= max;
        }
    }
    if !keep[0] {
        parts.days = 0;
    }
    if !keep[1] {
        parts.hours = 0;
    }
    if !keep[2] {
        parts.minutes = 0;
    }
    if !keep[3] {
        parts.seconds = 0;
    }
    if !keep[4] {
        parts.millis = 0;
    }
    if !keep[5] {
        parts.micros = 0;
    }
    if !keep[6] {
        parts.nanos = 0;
    }
}

#[allow(unused_assignments)]
fn write_compact(
    f: &mut fmt::Formatter<'_>,
    parts: &Parts,
    tables: &LocaleTables,
    sep: char,
) -> fmt::Result {
    let mut first = true;
    macro_rules! emit {
        ($val:expr, $suffix:expr) => {
            if $val > 0 {
                if !first {
                    f.write_char(sep)?;
                }
                write_u64(f, $val)?;
                f.write_str($suffix)?;
                first = false;
            }
        };
    }
    emit!(parts.days, tables.duration_day_short);
    emit!(parts.hours, tables.duration_hour_short);
    emit!(parts.minutes, tables.duration_minute_short);
    emit!(parts.seconds, tables.duration_second_short);
    if first && parts.millis == 0 && parts.micros == 0 && parts.nanos == 0 {
        f.write_str("0")?;
        f.write_str(tables.duration_second_short)?;
    } else {
        emit!(parts.millis, "ms");
        emit!(parts.micros, "µs");
        emit!(parts.nanos, "ns");
    }
    Ok(())
}

fn write_long(
    f: &mut fmt::Formatter<'_>,
    parts: &Parts,
    tables: &LocaleTables,
    sep: char,
) -> fmt::Result {
    let mut first = true;
    let mut wrote = false;

    let emit = |f: &mut fmt::Formatter<'_>,
                first: &mut bool,
                count: u64,
                one: &str,
                other: &str|
     -> fmt::Result {
        if count == 0 {
            return Ok(());
        }
        if !*first {
            f.write_char(sep)?;
        }
        write_u64(f, count)?;
        f.write_char(' ')?;
        f.write_str(if count == 1 { one } else { other })?;
        *first = false;
        Ok(())
    };

    emit(
        f,
        &mut first,
        parts.days,
        tables.duration_day_long_one,
        tables.duration_day_long_other,
    )?;
    wrote |= parts.days > 0;
    emit(
        f,
        &mut first,
        parts.hours,
        tables.duration_hour_long_one,
        tables.duration_hour_long_other,
    )?;
    wrote |= parts.hours > 0;
    emit(
        f,
        &mut first,
        parts.minutes,
        tables.duration_minute_long_one,
        tables.duration_minute_long_other,
    )?;
    wrote |= parts.minutes > 0;
    emit(
        f,
        &mut first,
        parts.seconds,
        tables.duration_second_long_one,
        tables.duration_second_long_other,
    )?;
    wrote |= parts.seconds > 0;

    if !wrote {
        write_long_single(f, tables, 0, DurationUnit::Second)?;
    }
    Ok(())
}

fn write_long_single(
    f: &mut fmt::Formatter<'_>,
    tables: &LocaleTables,
    count: u64,
    unit: DurationUnit,
) -> fmt::Result {
    write_u64(f, count)?;
    f.write_char(' ')?;
    match unit {
        DurationUnit::Second => f.write_str(if count == 1 {
            tables.duration_second_long_one
        } else {
            tables.duration_second_long_other
        }),
        DurationUnit::Minute => f.write_str(if count == 1 {
            tables.duration_minute_long_one
        } else {
            tables.duration_minute_long_other
        }),
        _ => f.write_str(tables.duration_second_long_other),
    }
}
