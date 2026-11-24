//! Shared string tables for each locale.

use super::time_unit::TimeUnit;

/// Static labels used by duration, relative, and percent formatters.
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocaleTables {
    pub(crate) percent_symbol: &'static str,
    pub(crate) past_prefix: &'static str,
    pub(crate) ago: &'static str,
    pub(crate) in_prefix: &'static str,
    pub(crate) in_suffix: &'static str,
    pub(crate) second_one: &'static str,
    pub(crate) second_other: &'static str,
    pub(crate) minute_one: &'static str,
    pub(crate) minute_other: &'static str,
    pub(crate) hour_one: &'static str,
    pub(crate) hour_other: &'static str,
    pub(crate) day_one: &'static str,
    pub(crate) day_other: &'static str,
    pub(crate) duration_second_short: &'static str,
    pub(crate) duration_minute_short: &'static str,
    pub(crate) duration_hour_short: &'static str,
    pub(crate) duration_day_short: &'static str,
    pub(crate) duration_second_long_one: &'static str,
    pub(crate) duration_second_long_other: &'static str,
    pub(crate) duration_minute_long_one: &'static str,
    pub(crate) duration_minute_long_other: &'static str,
    pub(crate) duration_hour_long_one: &'static str,
    pub(crate) duration_hour_long_other: &'static str,
    pub(crate) duration_day_long_one: &'static str,
    pub(crate) duration_day_long_other: &'static str,
}

impl LocaleTables {
    pub(crate) fn unit_label(&self, unit: TimeUnit, count: u64) -> &'static str {
        if count == 1 {
            match unit {
                TimeUnit::Second => self.second_one,
                TimeUnit::Minute => self.minute_one,
                TimeUnit::Hour => self.hour_one,
                TimeUnit::Day => self.day_one,
            }
        } else {
            match unit {
                TimeUnit::Second => self.second_other,
                TimeUnit::Minute => self.minute_other,
                TimeUnit::Hour => self.hour_other,
                TimeUnit::Day => self.day_other,
            }
        }
    }
}
