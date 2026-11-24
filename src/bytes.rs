//! Byte size formatting (binary and decimal).
use core::fmt::{self, Write};

use crate::context::Humany;
use crate::locale::Locale;
use crate::options::{write_scaled_decimal, FormatOptions, RoundingMode};

const BINARY_UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];
const DECIMAL_UNITS: &[&str] = &["B", "kB", "MB", "GB", "TB", "PB", "EB"];

/// Human-readable byte size (binary units by default).
#[derive(Clone, Copy, Debug)]
pub struct Bytes {
    value: u64,
    options: FormatOptions,
    locale: Locale,
    decimal: bool,
    units: &'static [&'static str],
    separator: char,
}

/// Format bytes using [`Humany::DEFAULT`] (binary IEC).
pub fn bytes(value: u64) -> Bytes {
    Humany::DEFAULT.bytes(value)
}

impl Bytes {
    pub(crate) fn new(value: u64, options: FormatOptions, locale: Locale) -> Self {
        Self {
            value,
            options,
            locale,
            decimal: false,
            units: BINARY_UNITS,
            separator: ' ',
        }
    }

    /// Set fractional digits for the scaled value (maximum `18`).
    pub const fn decimals(mut self, decimals: u8) -> Self {
        self.options = self.options.decimals(decimals);
        self
    }

    /// Set whether to trim trailing fractional zeros.
    pub const fn trim_trailing_zeros(mut self, trim: bool) -> Self {
        self.options = self.options.trim_trailing_zeros(trim);
        self
    }

    /// Set rounding when scaling.
    pub const fn rounding(mut self, rounding: RoundingMode) -> Self {
        self.options = self.options.rounding(rounding);
        self
    }

    /// Override locale for this value.
    pub const fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Use decimal (SI, base 1000) units instead of binary IEC.
    pub const fn decimal(mut self) -> Self {
        self.decimal = true;
        self.units = DECIMAL_UNITS;
        self
    }

    /// Character between the numeric part and unit label (default space).
    pub const fn separator(mut self, separator: char) -> Self {
        self.separator = separator;
        self
    }

    /// Replace the unit label table, starting with the label for bytes.
    /// An empty table uses the default labels for the current base.
    pub const fn units(mut self, units: &'static [&'static str]) -> Self {
        self.units = units;
        self
    }
}

impl fmt::Display for Bytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base = if self.decimal { 1000u64 } else { 1024u64 };
        let units = if self.units.is_empty() {
            if self.decimal {
                DECIMAL_UNITS
            } else {
                BINARY_UNITS
            }
        } else {
            self.units
        };

        if self.value < base {
            crate::options::write_u64(f, self.value)?;
            f.write_char(self.separator)?;
            f.write_str(units[0])?;
            return Ok(());
        }

        let mut tier = 0usize;
        let mut div = 1u64;
        while tier + 1 < units.len() && self.value / div >= base {
            tier += 1;
            div *= base;
        }

        write_scaled_decimal(
            f,
            self.value,
            false,
            div,
            self.options,
            self.locale.decimal_separator(),
        )?;
        f.write_char(self.separator)?;
        f.write_str(units[tier])
    }
}
