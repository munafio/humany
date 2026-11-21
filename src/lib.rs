//! Human-readable formatting for Rust (`#![no_std]` by default).
//!
//! ```rust
//! use humany::{bytes, duration, number, percent, Humany};
//!
//! assert_eq!(format!("{}", number(1_250_000)), "1.25M");
//! assert_eq!(format!("{}", bytes(1_024)), "1 KiB");
//! assert_eq!(format!("{}", duration(3665_u64)), "1h 1m 5s");
//! assert_eq!(format!("{}", percent(0.85623).decimals(2)), "85.62%");
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

pub mod options;

mod bytes;
mod context;
mod duration;
mod locale;
mod number;
mod percent;
mod relative;

#[cfg(feature = "std")]
mod std_rel;

#[cfg(any(feature = "chrono", feature = "time"))]
mod integrations;

pub use bytes::{bytes, Bytes};
pub use context::Humany;
pub use duration::{duration, DurationFmt, DurationInput, DurationUnit};
pub use locale::Locale;
pub use number::{number, IntoNumber, Number};
pub use options::{FormatOptions, RoundingMode};
pub use percent::{percent, Percent};
pub use relative::{relative_between, relative_secs, Relative};

#[cfg(feature = "std")]
pub use std_rel::{relative, RelativeTime};

#[cfg(feature = "chrono")]
pub use integrations::chrono::relative_chrono;

#[cfg(feature = "time")]
pub use integrations::time::relative_time;
