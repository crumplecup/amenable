//! Verus accommodation model for `chrono::NaiveTime::from_hms_nano_opt`.
//!
//! `chrono` has zero `vstd` coverage and Verus never resolves `Cargo.toml`
//! at all — see `fixed_offset.rs`'s own doc comment for the established
//! response this mirrors. This proof is conditional on the model being
//! faithful — which `amenable_kani::chrono::civil_naive_time::
//! verify_naive_time_from_hms_nano_matches_time_of_day_validity`'s own
//! harness, checked directly against chrono's real `from_hms_nano_opt`,
//! independently confirms.
//!
//! Unlike jiff's own `civil_time.rs` model, chrono's `NaiveTime` allows
//! one documented exception to an otherwise fully rectangular domain:
//! `nano` may range up to `1_999_999_999`, not just `999_999_999`, but
//! only when `second == 59` — chrono's own leap-second representation.
//! States that exact condition, not a narrower always-valid sub-range.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// chrono's own documented valid range for `from_hms_nano_opt`'s four
/// fields: `hour < 24`, `minute < 60`, `second < 60`, and `nano <
/// 2_000_000_000` -- but a `nano` of `1_000_000_000` or more (the leap-
/// second range) is only valid when `second == 59`.
pub open spec fn naive_time_fields_in_range(hour: int, minute: int, second: int, nano: int) -> bool {
    hour >= 0 && hour < 24 && minute >= 0 && minute < 60 && second >= 0 && second < 60
        && nano >= 0 && nano < 2_000_000_000
        && (nano < 1_000_000_000 || second == 59)
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `(hour, minute, second, nano)`
/// back out exactly, including a leap-second `nano`; out of range, it
/// reports none, mirroring `from_hms_nano_opt`'s real `Option::None`
/// case.
pub open spec fn naive_time_from_hms_nano_model_round_trip_holds(
    hour: u32,
    minute: u32,
    second: u32,
    nano: u32,
    result: Option<(u32, u32, u32, u32)>,
) -> bool {
    if naive_time_fields_in_range(hour as int, minute as int, second as int, nano as int) {
        result == Some((hour, minute, second, nano))
    } else {
        result is None
    }
}

/// A model of `NaiveTime::from_hms_nano_opt(hour, minute, second,
/// nano).map(|t| (t.hour(), t.minute(), t.second(), t.nanosecond()))`:
/// within range, the values round-trip, including a leap-second `nano`;
/// out of range, `None` — the same claim `amenable_kani::chrono::
/// civil_naive_time::
/// verify_naive_time_from_hms_nano_matches_time_of_day_validity`
/// checks against chrono's real API.
pub fn verify_naive_time_from_hms_nano_round_trips_model(
    hour: u32,
    minute: u32,
    second: u32,
    nano: u32,
) -> (result: Option<(u32, u32, u32, u32)>)
    ensures
        naive_time_from_hms_nano_model_round_trip_holds(hour, minute, second, nano, result),
{
    if hour < 24 && minute < 60 && second < 60 && nano < 2_000_000_000
        && (nano < 1_000_000_000 || second == 59)
    {
        Some((hour, minute, second, nano))
    } else {
        None
    }
}

} // verus!
