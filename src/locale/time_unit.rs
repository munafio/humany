//! Time units for relative and duration formatters.

/// Unit bucket for relative-time phrasing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
}

impl TimeUnit {
    pub(crate) fn unit_other_label(self, tables: &super::tables::LocaleTables) -> &'static str {
        match self {
            TimeUnit::Second => tables.second_other,
            TimeUnit::Minute => tables.minute_other,
            TimeUnit::Hour => tables.hour_other,
            TimeUnit::Day => tables.day_other,
        }
    }
}
