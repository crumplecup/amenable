//! Verus accommodation model for `chrono::NaiveDateTime::new`.
//!
//! `chrono` has zero `vstd` coverage and Verus never resolves `Cargo.toml`
//! at all — see `fixed_offset.rs`'s own doc comment for the established
//! response this mirrors. This proof is conditional on the model being
//! faithful — which `amenable_kani::chrono::civil_naive_date_time`'s own
//! harness for the identical claim, checked directly against chrono's
//! real `NaiveDateTime::new`, independently confirms.
//!
//! `NaiveDateTime` is exactly a `NaiveDate` paired with a `NaiveTime`,
//! with no interaction between the two beyond both needing to be
//! independently valid, the same relationship `amenable_kani::chrono::
//! civil_naive_date_time`'s own harness restates rather than calling
//! into its date/time siblings. Spec functions (`open spec fn`) are
//! ghost-only and erased under plain-rustc compatibility compilation
//! (confirmed: an earlier cross-file `use super::naive_date::
//! naive_date_fields_in_range` failed `cargo check` with "no
//! `naive_date_fields_in_range` in `chrono::naive_date`", even though
//! the real `verus` toolchain resolved it fine), so this restates
//! `naive_date.rs`'s and `naive_time.rs`'s own validity conditions
//! verbatim rather than importing them, matching the same restriction
//! every sibling cross-file spec reference in this crate already works
//! around the same way.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Gregorian leap year -- restated verbatim from `naive_date.rs`'s own
/// `gregorian_is_leap_year`; see this file's own doc comment for why.
pub open spec fn naive_date_time_is_leap_year(year: int) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Days in `month` of `year` -- restated verbatim from `naive_date.rs`'s
/// own `gregorian_days_in_month_spec`.
pub open spec fn naive_date_time_days_in_month(year: int, month: int) -> int {
    if month == 2 {
        if naive_date_time_is_leap_year(year) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else if month >= 1 && month <= 12 {
        31
    } else {
        0
    }
}

/// chrono's own documented supported year range -- restated verbatim
/// from `naive_date.rs`'s own `naive_date_fields_in_range`.
pub open spec fn naive_date_time_date_fields_in_range(year: int, month: int, day: int) -> bool {
    year >= -262143 && year <= 262142
        && month >= 1 && month <= 12
        && day >= 1 && day <= naive_date_time_days_in_month(year, month)
}

/// chrono's own documented valid range for the time fields, including
/// the leap-second exception -- restated verbatim from `naive_time.rs`'s
/// own `naive_time_fields_in_range`.
pub open spec fn naive_date_time_time_fields_in_range(hour: int, minute: int, second: int, nano: int) -> bool {
    hour >= 0 && hour < 24 && minute >= 0 && minute < 60 && second >= 0 && second < 60
        && nano >= 0 && nano < 2_000_000_000
        && (nano < 1_000_000_000 || second == 59)
}

/// The round-trip law this model establishes: within range (both the
/// date fields and the time fields independently valid), the modeled
/// constructor's result carries all seven fields back out exactly; out
/// of range, it reports none.
pub open spec fn naive_date_time_new_model_round_trip_holds(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    nano: u32,
    result: Option<(i32, u32, u32, u32, u32, u32, u32)>,
) -> bool {
    if naive_date_time_date_fields_in_range(year as int, month as int, day as int)
        && naive_date_time_time_fields_in_range(hour as int, minute as int, second as int, nano as int)
    {
        result == Some((year, month, day, hour, minute, second, nano))
    } else {
        result is None
    }
}

/// Executable days-in-month, proven equal to the specification --
/// restated verbatim from `naive_date.rs`'s own
/// `gregorian_days_in_month_exec`.
fn naive_date_time_days_in_month_exec(year: i32, month: u32) -> (result: u32)
    ensures
        result as int == naive_date_time_days_in_month(year as int, month as int),
{
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    if month == 2 {
        if leap { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else if (1..=12).contains(&month) {
        31
    } else {
        0
    }
}

/// A model of `NaiveDateTime::new(date, time)` built from
/// `NaiveDate::from_ymd_opt`/`NaiveTime::from_hms_nano_opt`, reporting
/// back all seven fields via its own date/time accessors: within
/// range, the values round-trip; out of range (either part invalid),
/// `None` — the same claim `amenable_kani::chrono::
/// civil_naive_date_time::
/// verify_naive_date_time_new_matches_date_and_time_validity` checks
/// against chrono's real API.
pub fn verify_naive_date_time_new_round_trips_model(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    nano: u32,
) -> (result: Option<(i32, u32, u32, u32, u32, u32, u32)>)
    ensures
        naive_date_time_new_model_round_trip_holds(
            year, month, day, hour, minute, second, nano, result,
        ),
{
    let date_valid = (-262143..=262142).contains(&year) && (1..=12).contains(&month)
        && (1..=naive_date_time_days_in_month_exec(year, month)).contains(&day);
    let time_valid = hour < 24 && minute < 60 && second < 60 && nano < 2_000_000_000
        && (nano < 1_000_000_000 || second == 59);
    if date_valid && time_valid {
        Some((year, month, day, hour, minute, second, nano))
    } else {
        None
    }
}

} // verus!
