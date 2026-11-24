//! Time units for relative and duration formatters.

/// Unit bucket for relative-time phrasing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
}
