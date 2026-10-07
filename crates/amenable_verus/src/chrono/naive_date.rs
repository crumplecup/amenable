//! Verus accommodation model for `chrono::NaiveDate::from_ymd_opt`.
//!
//! `chrono` has zero `vstd` coverage and Verus never resolves `Cargo.toml`
//! at all — see `fixed_offset.rs`'s own doc comment for the established
//! response this mirrors. This proof is conditional on the model being
//! faithful — which `amenable_kani::chrono::civil_naive_date`'s and
//! `amenable_creusot::ext_chrono::civil_naive_date`'s own harnesses for
//! the identical claim, checked directly against chrono's real
//! `from_ymd_opt`, independently confirm.
//!
//! Unlike jiff's own `civil_date.rs` model (which narrows day to
//! `1..=28`, always valid regardless of month or leap year, to avoid
//! modeling days-in-month at all), this model reproduces the real
//! proleptic Gregorian leap-year rule in full — the same rule
//! `amenable_creusot::ext_chrono::civil_naive_date`'s own Pearlite model
//! already proved out, restated here in Verus's own spec language. day
//! `1..=28` would have been simpler, but chrono's own Kani and Creusot
//! witnesses for this type are already full-domain, not narrowed; Verus
//! should not be the one backend that settles for less when the real
//! invariant is this tractable.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// Gregorian leap year: divisible by 4, except centuries not divisible
/// by 400.
pub open spec fn gregorian_is_leap_year(year: int) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// Days in `month` of `year` in the proleptic Gregorian calendar, or `0`
/// for a `month` outside `1..=12`.
pub open spec fn gregorian_days_in_month_spec(year: int, month: int) -> int {
    if month == 2 {
        if gregorian_is_leap_year(year) { 29 } else { 28 }
    } else if month == 4 || month == 6 || month == 9 || month == 11 {
        30
    } else if month >= 1 && month <= 12 {
        31
    } else {
        0
    }
}

/// Executable days-in-month, proven equal to the specification.
pub fn gregorian_days_in_month_exec(year: i32, month: u32) -> (result: u32)
    ensures
        result as int == gregorian_days_in_month_spec(year as int, month as int),
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

/// chrono's own documented supported year range (`-262143..=262142`,
/// `chrono` 0.4.45's `MIN_YEAR`/`MAX_YEAR`) — the same constant the
/// Kani and Creusot proofs for this type independently confirm.
pub open spec fn naive_date_fields_in_range(year: int, month: int, day: int) -> bool {
    year >= -262143 && year <= 262142
        && month >= 1 && month <= 12
        && day >= 1 && day <= gregorian_days_in_month_spec(year, month)
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `(year, month, day)` back out
/// exactly; out of range, it reports none, mirroring `from_ymd_opt`'s
/// real `Option::None` case.
pub open spec fn naive_date_from_ymd_model_round_trip_holds(
    year: i32,
    month: u32,
    day: u32,
    result: Option<(i32, u32, u32)>,
) -> bool {
    if naive_date_fields_in_range(year as int, month as int, day as int) {
        result == Some((year, month, day))
    } else {
        result is None
    }
}

/// A model of `NaiveDate::from_ymd_opt(year, month, day).map(|d|
/// (d.year(), d.month(), d.day()))`: within range, the values round-
/// trip; out of range, `None` — the same claim `amenable_kani::chrono::
/// civil_naive_date::verify_naive_date_from_ymd_round_trips` and
/// `amenable_creusot::ext_chrono::civil_naive_date::
/// verify_naive_date_model_round_trips` check (the latter against its
/// own Pearlite model, the former against chrono's real API directly).
pub fn verify_naive_date_from_ymd_round_trips_model(
    year: i32,
    month: u32,
    day: u32,
) -> (result: Option<(i32, u32, u32)>)
    ensures
        naive_date_from_ymd_model_round_trip_holds(year, month, day, result),
{
    if (-262143..=262142).contains(&year) && (1..=12).contains(&month) {
        let days = gregorian_days_in_month_exec(year, month);
        if (1..=days).contains(&day) {
            return Some((year, month, day));
        }
    }
    None
}

} // verus!
