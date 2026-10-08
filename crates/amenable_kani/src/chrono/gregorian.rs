//! The proleptic Gregorian calendar rule, shared by the chrono date-type witnesses.
//!
//! Stated here independently of chrono, so each witness's validity claim is checked
//! against a rule chrono does not supply.

/// Days in `month` of `year` under the proleptic Gregorian calendar, or `0` for a
/// month outside `1..=12`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug"))]
pub(super) fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
            if leap { 29 } else { 28 }
        }
        _ => 0,
    }
}
