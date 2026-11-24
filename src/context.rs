use crate::bytes::Bytes;
use crate::duration::{DurationFmt, DurationInput};
use crate::locale::Locale;
use crate::number::{IntoNumber, Number};
use crate::options::FormatOptions;
use crate::percent::Percent;
use crate::relative::Relative;

/// Formatter preset and defaults. `Copy` — store in app state or use presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Humany {
    locale: Locale,
    defaults: FormatOptions,
}

impl Default for Humany {
    fn default() -> Self {
        Self::new()
    }
}

impl Humany {
    /// English preset with default formatting options.
    pub const DEFAULT: Humany = Humany::new();
    /// Arabic preset with default formatting options.
    pub const AR: Humany = Humany {
        locale: Locale::Ar,
        defaults: FormatOptions::new(),
    };
    /// French preset.
    pub const FR: Humany = Humany::with_locale(Locale::Fr);
    /// Spanish preset.
    pub const ES: Humany = Humany::with_locale(Locale::Es);
    /// German preset.
    pub const DE: Humany = Humany::with_locale(Locale::De);
    /// Chinese (Simplified) preset.
    pub const ZH: Humany = Humany::with_locale(Locale::Zh);
    /// Japanese preset.
    pub const JA: Humany = Humany::with_locale(Locale::Ja);
    /// Portuguese preset.
    pub const PT: Humany = Humany::with_locale(Locale::Pt);
    /// Russian preset.
    pub const RU: Humany = Humany::with_locale(Locale::Ru);

    /// English locale and default [`FormatOptions`].
    pub const fn new() -> Self {
        Self::with_locale(Locale::En)
    }

    /// Preset with the given locale and default [`FormatOptions`].
    pub const fn with_locale(locale: Locale) -> Self {
        Self {
            locale,
            defaults: FormatOptions::new(),
        }
    }

    /// Set locale for formatters created from this instance.
    pub const fn locale(mut self, locale: Locale) -> Self {
        self.locale = locale;
        self
    }

    /// Default fractional digits for formatters from this instance.
    pub const fn decimals(mut self, decimals: u8) -> Self {
        self.defaults = self.defaults.decimals(decimals);
        self
    }

    /// Replace the shared [`FormatOptions`] for this instance.
    pub const fn defaults(mut self, defaults: FormatOptions) -> Self {
        self.defaults = defaults;
        self
    }

    /// Active locale tag.
    pub const fn locale_tag(self) -> Locale {
        self.locale
    }

    /// Build a [`Number`] formatter with this instance's locale and defaults.
    pub fn number<T: IntoNumber>(self, value: T) -> Number {
        value.into_humany_number(self.defaults, self.locale)
    }

    /// Build a [`Bytes`] formatter with this instance's locale and defaults.
    pub fn bytes(self, value: u64) -> Bytes {
        Bytes::new(value, self.defaults, self.locale)
    }

    /// Build a [`DurationFmt`] formatter with this instance's locale.
    pub fn duration(self, input: impl Into<DurationInput>) -> DurationFmt {
        DurationFmt::new(input.into(), self.locale)
    }

    /// Build a [`Percent`] formatter with this instance's locale and defaults.
    pub fn percent(self, ratio: f64) -> Percent {
        Percent::new(ratio, self.defaults, self.locale)
    }

    /// Build a [`Relative`] formatter from a signed second delta.
    pub fn relative_secs(self, delta_secs: i64) -> Relative {
        Relative::new(delta_secs, self.locale)
    }

    /// Wall-clock relative time (requires `std` feature).
    #[cfg(feature = "std")]
    pub fn relative(self, timestamp: std::time::SystemTime) -> crate::std_rel::RelativeTime {
        crate::std_rel::RelativeTime::new(timestamp, self.locale)
    }
}
