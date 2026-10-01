//! The Gregorian leap-year rule and the year-length contracts built on it.
//!
//! Split further since the combined 724-line file exceeded cordial's
//! 500-line limit: `rule` (the divisibility rule itself, plus the
//! shared `is_gregorian_leap_year`/`days_in_year`/`days_in_month`/
//! `is_valid_calendar_day` day-counting helpers every other contract
//! here and in `super::month_day_bounds` builds on) and `year_length`
//! (the 365-vs-366-day consequences).

mod rule;
mod year_length;

pub(crate) use rule::{days_in_month, days_in_year, is_gregorian_leap_year, is_valid_calendar_day};
