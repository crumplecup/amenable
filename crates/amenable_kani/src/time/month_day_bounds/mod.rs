//! Month-duration and within-month day bounds — built on
//! `leap_year_core`'s leap-year rule.
//!
//! Split further since the combined 515-line file exceeded cordial's
//! 500-line limit: `month_duration` (the 28-31-day range contract) and
//! `day_bounds` (within-month day validity + leap-day-only-in-leap-
//! year).

mod day_bounds;
mod month_duration;
