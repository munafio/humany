//! Ratio-to-percentage formatting.

use core::fmt;

use crate::context::Humany;
use crate::locale::Locale;
use crate::options::{write_fixed_f64, FormatOptions, RoundingMode};

/// Human-readable percentage from a ratio (`0.5` → `50%`).
#[derive(Clone, Copy, Debug)]
pub struct Percent {
    ratio: f64,
    options: FormatOptions,
    locale: Locale,
    symbol: &'static str,
}

/// Format a ratio as a percentage using [`Humany::DEFAULT`].
pub fn percent(ratio: f64) -> Percent {
    Humany::DEFAULT.percent(ratio)
}

impl Percent {
    pub(crate) fn new(ratio: f64, options: FormatOptions, locale: Locale) -> Self {
        Self {
            ratio,
            options,
            locale,
            symbol: "%",
        }
    }

    /// Set fractional digits after multiplying by 100.
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

    /// Override the suffix (default locale-aware `%`).
    pub const fn symbol(mut self, symbol: &'static str) -> Self {
        self.symbol = symbol;
        self
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pct = self.ratio * 100.0;
        write_fixed_f64(f, pct, self.options)?;
        f.write_str(if self.symbol == "%" {
            self.locale.tables().percent_symbol
        } else {
            self.symbol
        })
    }
}
