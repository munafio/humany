//! Compact number formatting with SI suffixes.

use core::fmt;

use crate::context::Humany;
use crate::locale::Locale;
use crate::options::{
    write_fixed_f64, write_i64, write_scaled_decimal, FormatOptions, RoundingMode,
};

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
pub(crate) enum NumberBits {
    I64(i64),
    F64(f64),
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
    ($($ty:ty),+ $(,)?) => {
        $(
            impl IntoNumber for $ty {
                fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number {
                    Number::from_bits(NumberBits::I64(self as i64), defaults, locale)
                }
            }
        )+
    };
}

impl_into_number_int!(i8, i16, i32, i64, u8, u16, u32);

impl IntoNumber for u64 {
    fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number {
        let bits = if self <= i64::MAX as u64 {
            NumberBits::I64(self as i64)
        } else {
            NumberBits::F64(self as f64)
        };
        Number::from_bits(bits, defaults, locale)
    }
}

impl IntoNumber for f32 {
    fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number {
        Number::from_bits(NumberBits::F64(self as f64), defaults, locale)
    }
}

impl IntoNumber for f64 {
    fn into_humany_number(self, defaults: FormatOptions, locale: Locale) -> Number {
        Number::from_bits(NumberBits::F64(self), defaults, locale)
    }
}

impl Number {
    pub(crate) fn from_bits(bits: NumberBits, options: FormatOptions, locale: Locale) -> Self {
        Self {
            bits,
            options,
            locale,
            suffixes: DEFAULT_SUFFIXES,
        }
    }

    /// Set the number of fractional digits when scaling (default `2`).
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

    /// Custom unit suffixes (`""`, `K`, `M`, …); must include at least five entries.
    pub const fn suffixes(mut self, suffixes: &'static [&'static str]) -> Self {
        self.suffixes = suffixes;
        self
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = self.locale;
        match self.bits {
            NumberBits::I64(v) => fmt_i64(f, v, self.options, self.suffixes),
            NumberBits::F64(v) => fmt_f64(f, v, self.options, self.suffixes),
        }
    }
}

fn fmt_i64(
    f: &mut fmt::Formatter<'_>,
    value: i64,
    options: FormatOptions,
    suffixes: &[&str],
) -> fmt::Result {
    let negative = value < 0;
    let abs = if negative {
        if value == i64::MIN {
            u128::from(i64::MAX as u64) + 1
        } else {
            (-value) as u128
        }
    } else {
        value as u128
    };

    if abs < 1000 {
        return write_i64(f, value);
    }

    let tier = tier_for(abs);
    if tier >= suffixes.len() {
        return write_i64(f, value);
    }

    if negative {
        f.write_str("-")?;
    }

    let divisor = 1000u128.saturating_pow(tier as u32);
    write_scaled_decimal(f, abs, divisor, options)?;
    f.write_str(suffixes[tier])
}

fn fmt_f64(
    f: &mut fmt::Formatter<'_>,
    value: f64,
    options: FormatOptions,
    suffixes: &[&str],
) -> fmt::Result {
    let negative = value < 0.0;
    let abs = value.abs();

    if abs < 1000.0 {
        return write_fixed_f64(
            f,
            value,
            FormatOptions {
                decimals: 0,
                trim_trailing_zeros: true,
                rounding: options.rounding,
            },
        );
    }

    let tier = tier_for(abs as u128);
    if tier >= suffixes.len() {
        return write_fixed_f64(f, value, options);
    }

    if negative {
        f.write_str("-")?;
    }

    let scaled = abs / libm::pow(1000.0, tier as f64);
    write_fixed_f64(f, scaled, options)?;
    f.write_str(suffixes[tier])
}

fn tier_for(abs: u128) -> usize {
    let mut tier = 0usize;
    let mut threshold = 1000u128;
    while abs >= threshold && tier + 1 < DEFAULT_SUFFIXES.len() {
        threshold = threshold.saturating_mul(1000);
        tier += 1;
    }
    tier
}
