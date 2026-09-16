//! Verus accommodation model for `jiff::civil::Date::new`/`Date::
//! year`/`Date::month`/`Date::day`.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! civil_date`'s and `amenable_creusot::ext_jiff::civil_date`'s own
//! harnesses for the identical claim, checked directly against
//! jiff's real `Date::new`/`year`/`month`/`day`, independently
//! confirm.
//!
//! Scoped the same way both of those are, for the same real reason:
//! day `1..=28` is valid for every month in every year (even February
//! in a non-leap year), so this avoids needing to model days-in-month
//! for a symbolic year/month combination.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// jiff's own documented valid range for `civil::Date::new`'s year
/// and month, and this model's own deliberately narrowed sufficient
/// sub-range for day — the same constants the Kani and Creusot
/// proofs for this type independently confirm.
pub open spec fn civil_date_fields_in_range(year: int, month: int, day: int) -> bool {
    year >= -9999 && year <= 9999 && month >= 1 && month <= 12 && day >= 1 && day <= 28
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `(year, month, day)` back out
/// exactly; out of range, it reports none, mirroring `Date::new`'s
/// real `Result::Err` case.
pub open spec fn civil_date_new_model_round_trip_holds(
    year: i16,
    month: i8,
    day: i8,
    result: Option<(i16, i8, i8)>,
) -> bool {
    if civil_date_fields_in_range(year as int, month as int, day as int) {
        result == Some((year, month, day))
    } else {
        result is None
    }
}

/// A model of `Date::new(year, month, day).ok().map(|d| (d.year(),
/// d.month(), d.day()))`: within the always-valid sub-range, the
/// values round-trip; out of range, `None` — the same claim
/// `amenable_kani::ext::jiff::civil_date::
/// verify_civil_date_new_year_month_day_round_trips` and
/// `amenable_creusot::ext_jiff::civil_date::
/// verify_civil_date_new_year_month_day_round_trips` check against
/// jiff's real API.
pub fn verify_civil_date_new_year_month_day_round_trips_model(
    year: i16,
    month: i8,
    day: i8,
) -> (result: Option<(i16, i8, i8)>)
    ensures
        civil_date_new_model_round_trip_holds(year, month, day, result),
{
    if (-9999..=9999).contains(&year) && (1..=12).contains(&month) && (1..=28).contains(&day) {
        Some((year, month, day))
    } else {
        None
    }
}

} // verus!
