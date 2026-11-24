//! Compact number formatting with SI suffixes.

use core::fmt;

use crate::context::Humany;
use crate::locale::Locale;
use crate::options::{write_scaled_decimal, write_scaled_f64, FormatOptions, RoundingMode};

const DEFAULT_SUFFIXES: &[&str] = &["", "K", "M", "B", "T"];

/// Scaled human-readable number.
#[derive(Clone, Copy, Debug)]
pub struct Number {
    bits: NumberBits,
    options: FormatOptions,
    locale: Locale,
    suffixes: &'static [&'static str],
}

#[derive(Clone, Copy, Debug)]
enum NumberBits {
    Signed(i64),
    Unsigned(u64),
    Float(f64),
}

/// Format a number using [`Humany::DEFAULT`] (English).
pub fn number<T: IntoNumber>(value: T) -> Number {
    Humany::DEFAULT.number(value)
}

/// Types accepted by [`number`] and [`Humany::number`].
pub trait IntoNumber {
    #[doc(hidden)]
    fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number;
}

macro_rules! impl_into_number_int {
    ($variant:ident, $storage:ty; $($ty:ty),+ $(,)?) => {
        $(
            impl IntoNumber for $ty {
                fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number {
                    Number::from_bits(NumberBits::$variant(self as $storage), defaults, locale)
                }
            }
        )+
    };
}

impl_into_number_int!(Signed, i64; i8, i16, i32, i64, isize);
impl_into_number_int!(Unsigned, u64; u8, u16, u32, u64, usize);

impl IntoNumber for f32 {
    fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number {
        Number::from_bits(NumberBits::Float(self as f64), defaults, locale)
    }
}

impl IntoNumber for f64 {
    fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number {
        Number::from_bits(NumberBits::Float(self), defaults, locale)
    }
}

impl Number {
    fn from_bits(bits: NumberBits, options: FormatOptions, locale: Locale) -> Self {
        Self {
            bits,
            options,
            locale,
            suffixes: DEFAULT_SUFFIXES,
        }
    }

    /// Set the number of fractional digits when scaling (default `2`, maximum `18`).
    pub const fn decimals(mut self, decimals: u8) -> Self {
        self.options = self.options.decimals(decimals);
        self
    }

    /// Control trimming of trailing fractional zeros (default `true`).
    pub const fn trim_trailing_zeros(mut self, trim: bool) -> Self {
        self.options = self.options.trim_trailing_zeros(trim);
        self
    }

    /// Set rounding used when scaling.
    pub const fn rounding(mut self, rounding: RoundingMode) -> Self {
        self.options = self.options.rounding(rounding);
        self
    }

    /// Override locale for this value (wins over [`Humany`] preset).
    pub const fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Custom unit suffixes (`""`, `K`, `M`, …). A missing tier is printed unscaled.
    pub const fn suffixes(mut self, suffixes: &'static [&'static str]) -> Self {
        self.suffixes = suffixes;
        self
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let separator = self.locale.decimal_separator();
        match self.bits {
            NumberBits::Signed(v) => self.fmt_integer(f, v.unsigned_abs(), v < 0, separator),
            NumberBits::Unsigned(v) => self.fmt_integer(f, v, false, separator),
            NumberBits::Float(v) => self.fmt_float(f, v, separator),
        }
    }
}

impl Number {
    fn fmt_integer(
        &self,
        f: &mut fmt::Formatter<'_>,
        magnitude: u64,
        negative: bool,
        separator: char,
    ) -> fmt::Result {
        let tier = tier_for(magnitude);
        if tier == 0 || tier >= self.suffixes.len() {
            return if negative {
                write!(f, "-{magnitude}")
            } else {
                write!(f, "{magnitude}")
            };
        }

        write_scaled_decimal(
            f,
            magnitude,
            negative,
            DIVISORS[tier],
            self.options,
            separator,
        )?;
        f.write_str(self.suffixes[tier])
    }

    fn fmt_float(&self, f: &mut fmt::Formatter<'_>, value: f64, separator: char) -> fmt::Result {
        if !value.is_finite() {
            return write!(f, "{value}");
        }

        let tier = tier_for(value.abs() as u64);
        if tier == 0 {
            return write_scaled_f64(f, value, 0, self.options.decimals(0), separator);
        }
        if tier >= self.suffixes.len() {
            return write_scaled_f64(f, value, 0, self.options, separator);
        }

        write_scaled_f64(f, value, -3 * tier as i32, self.options, separator)?;
        f.write_str(self.suffixes[tier])
    }
}

const DIVISORS: [u64; 5] = [1, 1_000, 1_000_000, 1_000_000_000, 1_000_000_000_000];

fn tier_for(magnitude: u64) -> usize {
    DIVISORS
        .partition_point(|&divisor| divisor <= magnitude)
        .saturating_sub(1)
}
