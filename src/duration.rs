use core::fmt::{self, Write};
use core::time::Duration as CoreDuration;

use crate::context::Humany;
use crate::locale::Locale;
use crate::options::{write_u64, RoundingMode};

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
    input: CoreDuration,
    locale: Locale,
    compact: bool,
    max_units: u8,
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
    pub(crate) fn new(input: DurationInput, locale: Locale) -> Self {
        let input = match input {
            DurationInput::Secs(seconds) => CoreDuration::from_secs(seconds),
            DurationInput::Std(duration) => duration,
        };
        Self {
            input,
            locale,
            compact: true,
            max_units: 7,
            min_unit: DurationUnit::Nanosecond,
            separator: ' ',
        }
    }

    /// Duration output uses whole units. This compatibility method has no effect.
    #[deprecated(note = "duration output uses whole units; use min_unit to limit precision")]
    pub const fn decimals(self, _decimals: u8) -> Self {
        self
    }

    /// Override locale for this value.
    pub const fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Limit how many non-zero components are shown (largest first).
    /// A limit of zero displays a single zero-valued unit.
    pub const fn max_units(mut self, n: u8) -> Self {
        self.max_units = n;
        self
    }

    /// Smallest unit to emit. If all components are dropped, the result is
    /// zero in this unit or in seconds, whichever unit is larger.
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

    /// Duration output drops units below the configured limit.
    /// This compatibility method has no effect.
    #[deprecated(note = "duration output uses whole units and drops smaller components")]
    pub const fn rounding(self, _rounding: RoundingMode) -> Self {
        self
    }
}

impl DurationUnit {
    const ALL: [Self; 7] = [
        Self::Day,
        Self::Hour,
        Self::Minute,
        Self::Second,
        Self::Millisecond,
        Self::Microsecond,
        Self::Nanosecond,
    ];
}

impl fmt::Display for DurationFmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let seconds = self.input.as_secs();
        let nanos = u64::from(self.input.subsec_nanos());
        let values = [
            seconds / 86_400,
            seconds / 3_600 % 24,
            seconds / 60 % 60,
            seconds % 60,
            nanos / 1_000_000,
            nanos / 1_000 % 1_000,
            nanos % 1_000,
        ];
        let mut remaining = self.max_units;
        for (unit, count) in DurationUnit::ALL.into_iter().zip(values) {
            if unit > self.min_unit || remaining == 0 {
                break;
            }
            if count == 0 {
                continue;
            }
            if remaining != self.max_units {
                f.write_char(self.separator)?;
            }
            self.write_component(f, unit, count)?;
            remaining -= 1;
        }
        if remaining == self.max_units {
            self.write_component(f, self.min_unit.min(DurationUnit::Second), 0)?;
        }
        Ok(())
    }
}

impl DurationFmt {
    fn write_component(
        &self,
        f: &mut fmt::Formatter<'_>,
        unit: DurationUnit,
        count: u64,
    ) -> fmt::Result {
        write_u64(f, count)?;
        if self.compact {
            f.write_str(self.locale.short_duration_label(unit))
        } else {
            f.write_char(' ')?;
            f.write_str(self.locale.long_duration_label(unit, count))
        }
    }
}
