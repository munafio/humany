//! Shared formatting options and decimal helpers.

use core::fmt::{self, Write};

/// How fractional digits are rounded when scaling values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RoundingMode {
    /// Round halves away from zero.
    #[default]
    HalfAwayFromZero,
    /// Round toward negative infinity.
    Floor,
    /// Round toward positive infinity.
    Ceil,
    /// Round toward zero.
    Trunc,
}

/// Options shared across formatters.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormatOptions {
    /// Number of digits after the decimal separator when scaling (at most 18).
    pub decimals: u8,
    /// Whether to drop trailing fractional zeros (e.g. `1.50` → `1.5`).
    pub trim_trailing_zeros: bool,
    /// Rounding rule for scaled output.
    pub rounding: RoundingMode,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatOptions {
    /// Maximum supported number of fractional digits.
    pub const MAX_DECIMALS: u8 = 18;

    /// Default shared options (`2` decimals, trim zeros, half away from zero).
    pub const fn new() -> Self {
        Self {
            decimals: 2,
            trim_trailing_zeros: true,
            rounding: RoundingMode::HalfAwayFromZero,
        }
    }

    /// Set fractional digit count, clamped to [`Self::MAX_DECIMALS`].
    pub const fn decimals(self, decimals: u8) -> Self {
        Self {
            decimals: if decimals > Self::MAX_DECIMALS {
                Self::MAX_DECIMALS
            } else {
                decimals
            },
            ..self
        }
    }

    /// Set whether to trim trailing fractional zeros.
    pub const fn trim_trailing_zeros(self, trim: bool) -> Self {
        Self {
            trim_trailing_zeros: trim,
            ..self
        }
    }

    /// Set rounding mode.
    pub const fn rounding(self, rounding: RoundingMode) -> Self {
        Self { rounding, ..self }
    }
}

/// Write a non-negative integer.
pub(crate) fn write_u64(f: &mut fmt::Formatter<'_>, n: u64) -> fmt::Result {
    write!(f, "{n}")
}

/// Write a signed magnitude divided by a positive unit size.
pub(crate) fn write_scaled_decimal(
    f: &mut fmt::Formatter<'_>,
    magnitude: u64,
    negative: bool,
    divisor: u64,
    options: FormatOptions,
    separator: char,
) -> fmt::Result {
    let places = options.decimals.min(FormatOptions::MAX_DECIMALS);
    let scale = 10u128.pow(places.into());
    let mode = magnitude_rounding(options.rounding, negative);
    let rounded = round_quotient(u128::from(magnitude) * scale, divisor.into(), mode);
    write_fixed(
        f,
        rounded,
        negative,
        scale,
        places,
        options.trim_trailing_zeros,
        separator,
    )
}

/// Scale a float's shortest round-trippable decimal representation by a power of ten.
pub(crate) fn write_scaled_f64(
    f: &mut fmt::Formatter<'_>,
    value: f64,
    power: i32,
    options: FormatOptions,
    separator: char,
) -> fmt::Result {
    if !value.is_finite() {
        return write!(f, "{value}");
    }

    let places = options.decimals.min(FormatOptions::MAX_DECIMALS);
    if value == 0.0 {
        f.write_char('0')?;
        return write_fraction(f, 0, places, options.trim_trailing_zeros, separator);
    }

    // The shortest scientific notation needs at most 17 significant digits,
    // a decimal point, and an exponent such as e-324.
    let mut buffer = ScientificBuffer {
        bytes: [0; 24],
        len: 0,
    };
    write!(&mut buffer, "{:e}", value.abs())?;
    let decimal = core::str::from_utf8(&buffer.bytes[..buffer.len]).map_err(|_| fmt::Error)?;
    let (digits, exponent) = decimal.split_once('e').ok_or(fmt::Error)?;
    let exponent = exponent.parse::<i32>().map_err(|_| fmt::Error)?;
    let fractional_digits = digits
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    let coefficient = digits
        .bytes()
        .filter(|&byte| byte != b'.')
        .fold(0u128, |n, digit| n * 10 + u128::from(digit - b'0'));

    let negative = value.is_sign_negative();
    let exponent = exponent - fractional_digits as i32 + power;
    if exponent >= 0 {
        if negative {
            f.write_char('-')?;
        }
        write!(f, "{coefficient}")?;
        for _ in 0..exponent {
            f.write_char('0')?;
        }
        return write_fraction(f, 0, places, options.trim_trailing_zeros, separator);
    }

    let mode = magnitude_rounding(options.rounding, negative);
    let shift = exponent + i32::from(places);
    let rounded = match shift {
        0.. => coefficient * 10u128.pow(shift as u32),
        -18..=-1 => round_quotient(coefficient, 10u128.pow(shift.unsigned_abs()), mode),
        _ => u128::from(mode == RoundingMode::Ceil),
    };
    let scale = 10u128.pow(places.into());
    write_fixed(
        f,
        rounded,
        negative,
        scale,
        places,
        options.trim_trailing_zeros,
        separator,
    )
}

struct ScientificBuffer {
    bytes: [u8; 24],
    len: usize,
}

impl Write for ScientificBuffer {
    fn write_str(&mut self, text: &str) -> fmt::Result {
        let end = self.len + text.len();
        self.bytes
            .get_mut(self.len..end)
            .ok_or(fmt::Error)?
            .copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

fn magnitude_rounding(mode: RoundingMode, negative: bool) -> RoundingMode {
    match (mode, negative) {
        (RoundingMode::Floor, true) => RoundingMode::Ceil,
        (RoundingMode::Ceil, true) => RoundingMode::Floor,
        _ => mode,
    }
}

fn round_quotient(num: u128, divisor: u128, mode: RoundingMode) -> u128 {
    let quotient = num / divisor;
    let remainder = num % divisor;
    let increment = match mode {
        RoundingMode::HalfAwayFromZero => remainder >= divisor - remainder,
        RoundingMode::Ceil => remainder != 0,
        RoundingMode::Floor | RoundingMode::Trunc => false,
    };
    quotient + u128::from(increment)
}

fn write_fixed(
    f: &mut fmt::Formatter<'_>,
    rounded: u128,
    negative: bool,
    scale: u128,
    places: u8,
    trim: bool,
    separator: char,
) -> fmt::Result {
    if negative && rounded != 0 {
        f.write_char('-')?;
    }
    let whole = rounded / scale;
    write!(f, "{whole}")?;
    write_fraction(f, (rounded % scale) as u64, places, trim, separator)
}

fn write_fraction(
    f: &mut fmt::Formatter<'_>,
    mut fraction: u64,
    mut places: u8,
    trim: bool,
    separator: char,
) -> fmt::Result {
    if places == 0 || (trim && fraction == 0) {
        return Ok(());
    }

    if trim {
        while fraction % 10 == 0 {
            fraction /= 10;
            places -= 1;
        }
    }
    f.write_char(separator)?;
    let width = usize::from(places);
    write!(f, "{fraction:0width$}")
}
