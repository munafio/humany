//! Ratio-to-percentage formatting.

use core::fmt;

use crate::context::Humany;
use crate::locale::Locale;
use crate::options::{write_scaled_f64, FormatOptions, RoundingMode};

/// Human-readable percentage from a ratio (`0.5` → `50%`).
#[derive(Clone, Copy, Debug)]
pub struct Percent {
    ratio: f64,
    options: FormatOptions,
    locale: Locale,
    symbol: Option<&'static str>,
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
            symbol: None,
        }
    }

    /// Set fractional digits after multiplying by 100 (maximum `18`).
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

    /// Override the locale's percent suffix.
    pub const fn symbol(mut self, symbol: &'static str) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

impl fmt::Display for Percent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_scaled_f64(
            f,
            self.ratio,
            2,
            self.options,
            self.locale.decimal_separator(),
        )?;
        f.write_str(self.symbol.unwrap_or(self.locale.tables().percent_symbol))
    }
}
