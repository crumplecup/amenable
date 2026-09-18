//! Verus accommodation model for `jiff::civil::Time::new`/`hour`/
//! `minute`/`second`/`subsec_nanosecond`.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! civil_time`'s and `amenable_creusot::ext_jiff::civil_time`'s own
//! harnesses for the identical claim, checked directly against
//! jiff's real API, independently confirm.
//!
//! Unlike `civil_date.rs`/`civil_iso_week_date.rs`, `Time::new`'s
//! validity is a fully rectangular, unconditional domain with no
//! interdependency between fields at all, so this states jiff's FULL
//! documented validity condition, not a narrowed sufficient
//! sub-range.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's own documented FULL valid range for `civil::Time::new`'s
/// four fields — no narrowing needed, unlike the other `civil::*`
/// models, since each field's validity is fully independent.
pub open spec fn civil_time_fields_in_range(hour: int, minute: int, second: int, subsec_nanosecond: int) -> bool {
    hour >= 0 && hour <= 23 && minute >= 0 && minute <= 59 && second >= 0 && second <= 59
        && subsec_nanosecond >= 0 && subsec_nanosecond <= 999999999
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `(hour, minute, second,
/// subsec_nanosecond)` back out exactly; out of range, it reports
/// none, mirroring `Time::new`'s real `Result::Err` case.
pub open spec fn civil_time_new_model_round_trip_holds(
    hour: i8,
    minute: i8,
    second: i8,
    subsec_nanosecond: i32,
    result: Option<(i8, i8, i8, i32)>,
) -> bool {
    if civil_time_fields_in_range(hour as int, minute as int, second as int, subsec_nanosecond as int) {
        result == Some((hour, minute, second, subsec_nanosecond))
    } else {
        result is None
    }
}

/// A model of `Time::new(hour, minute, second,
/// subsec_nanosecond).ok().map(|t| (t.hour(), t.minute(), t.second(),
/// t.subsec_nanosecond()))`: within range, the values round-trip; out
/// of range, `None` — the same claim `amenable_kani::ext::jiff::
/// civil_time::verify_civil_time_new_hour_minute_second_subsec_round_trips`
/// and `amenable_creusot::ext_jiff::civil_time::
/// verify_civil_time_new_hour_minute_second_subsec_round_trips` check
/// against jiff's real API.
pub fn verify_civil_time_new_hour_minute_second_subsec_round_trips_model(
    hour: i8,
    minute: i8,
    second: i8,
    subsec_nanosecond: i32,
) -> (result: Option<(i8, i8, i8, i32)>)
    ensures
        civil_time_new_model_round_trip_holds(hour, minute, second, subsec_nanosecond, result),
{
    if (0..=23).contains(&hour) && (0..=59).contains(&minute) && (0..=59).contains(&second)
        && (0..=999999999).contains(&subsec_nanosecond) {
        Some((hour, minute, second, subsec_nanosecond))
    } else {
        None
    }
}

} // verus!
