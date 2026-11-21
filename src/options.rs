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
    /// Number of digits after the decimal separator when scaling.
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
    /// Default shared options (`2` decimals, trim zeros, half away from zero).
    pub const fn new() -> Self {
        Self {
            decimals: 2,
            trim_trailing_zeros: true,
            rounding: RoundingMode::HalfAwayFromZero,
        }
    }

    /// Set fractional digit count.
    pub const fn decimals(self, decimals: u8) -> Self {
        Self { decimals, ..self }
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
    if n >= 10 {
        write_u64(f, n / 10)?;
    }
    let d = (n % 10) as u8;
    f.write_char((b'0' + d) as char)
}

/// Write a signed integer.
pub(crate) fn write_i64(f: &mut fmt::Formatter<'_>, n: i64) -> fmt::Result {
    if n < 0 {
        f.write_str("-")?;
        if n == i64::MIN {
            return write_u64(f, 9223372036854775808);
        }
        write_u64(f, (-n) as u64)
    } else {
        write_u64(f, n as u64)
    }
}

/// Scale `numerator / divisor` with fixed decimal places into the formatter.
pub(crate) fn write_scaled_decimal(
    f: &mut fmt::Formatter<'_>,
    numerator: u128,
    divisor: u128,
    options: FormatOptions,
) -> fmt::Result {
    if divisor == 0 {
        return f.write_str("0");
    }

    let places = options.decimals as u32;
    if places == 0 {
        let rounded = round_quotient(numerator, divisor, options.rounding);
        return write_u128(f, rounded.quotient);
    }

    let scale = 10u128.saturating_pow(places);
    let int_part = numerator / divisor;
    let rem = numerator % divisor;
    let mut frac = round_quotient(rem * scale, divisor, options.rounding).quotient;
    let mut int_part = int_part;
    if frac >= scale {
        int_part += 1;
        frac -= scale;
    }

    write_u128(f, int_part)?;
    if frac > 0 || !options.trim_trailing_zeros {
        f.write_str(".")?;
        write_frac_padded(f, frac, places as usize, options.trim_trailing_zeros)?;
    }
    Ok(())
}

/// `f64` scaled value (already divided).
pub(crate) fn write_fixed_f64(
    f: &mut fmt::Formatter<'_>,
    value: f64,
    options: FormatOptions,
) -> fmt::Result {
    let places = options.decimals as i32;
    if places <= 0 {
        let rounded = round_f64(value, 0, options.rounding);
        return write_i64(f, rounded as i64);
    }

    let factor = libm::pow(10.0, places as f64);
    let scaled = round_f64(value * factor, 0, options.rounding);
    let int_part = libm::trunc(scaled / factor) as i64;
    let frac = libm::round(scaled.abs() - (int_part.abs() as f64) * factor) as u64;

    if int_part < 0 {
        f.write_str("-")?;
        write_u64(f, int_part.unsigned_abs())?;
    } else {
        write_u64(f, int_part as u64)?;
    }

    if options.decimals > 0 && (frac > 0 || !options.trim_trailing_zeros) {
        f.write_str(".")?;
        write_frac_padded(
            f,
            frac as u128,
            places as usize,
            options.trim_trailing_zeros,
        )?;
    }
    Ok(())
}

struct Quotient {
    quotient: u128,
}

fn round_quotient(num: u128, div: u128, mode: RoundingMode) -> Quotient {
    let q = num / div;
    let r = num % div;
    let quotient = match mode {
        RoundingMode::Trunc => q,
        RoundingMode::Floor => q,
        RoundingMode::Ceil => {
            if r == 0 {
                q
            } else {
                q + 1
            }
        }
        RoundingMode::HalfAwayFromZero => {
            if r * 2 >= div {
                q + 1
            } else {
                q
            }
        }
    };
    Quotient { quotient }
}

fn round_f64(value: f64, _places: i32, mode: RoundingMode) -> f64 {
    match mode {
        RoundingMode::HalfAwayFromZero => {
            let fract = libm::fabs(value - libm::trunc(value));
            if value >= 0.0 {
                libm::floor(value) + if fract >= 0.5 { 1.0 } else { 0.0 }
            } else {
                libm::ceil(value) - if fract >= 0.5 { 1.0 } else { 0.0 }
            }
        }
        RoundingMode::Floor => libm::floor(value),
        RoundingMode::Ceil => libm::ceil(value),
        RoundingMode::Trunc => libm::trunc(value),
    }
}

fn write_u128(f: &mut fmt::Formatter<'_>, n: u128) -> fmt::Result {
    let mut buf = [0u8; 39];
    let mut i = buf.len();
    let mut n = n;
    if n == 0 {
        return f.write_char('0');
    }
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    let s = core::str::from_utf8(&buf[i..]).unwrap_or("0");
    f.write_str(s)
}

fn write_frac_padded(
    f: &mut fmt::Formatter<'_>,
    mut frac: u128,
    places: usize,
    trim: bool,
) -> fmt::Result {
    if places == 0 {
        return Ok(());
    }
    let mut buf = [0u8; 32];
    for i in (0..places).rev() {
        buf[i] = b'0' + (frac % 10) as u8;
        frac /= 10;
    }
    let mut end = places;
    if trim {
        while end > 0 && buf[end - 1] == b'0' {
            end -= 1;
        }
        if end == 0 {
            return Ok(());
        }
    }
    let s = core::str::from_utf8(&buf[..end]).map_err(|_| fmt::Error)?;
    f.write_str(s)
}
