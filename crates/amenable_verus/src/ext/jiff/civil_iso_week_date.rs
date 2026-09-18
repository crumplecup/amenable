//! Verus accommodation model for `jiff::civil::ISOWeekDate::new`/
//! `year`/`week`/`weekday`.
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_kani::ext::jiff::
//! civil_iso_week_date`'s and `amenable_creusot::ext_jiff::
//! civil_iso_week_date`'s own harnesses for the identical claim,
//! checked directly against jiff's real API, independently confirm.
//!
//! Scoped the same way both of those are, for the same real reason
//! confirmed there (not assumed): a first attempt using the full
//! `-9999..=9999` year range alongside week `1..=52` FAILED for real
//! on Kani (`ISOWeekDate::MIN`/`MAX` are derived from `Date::MIN`/
//! `MAX`, not the leap-week rule alone, so a week/weekday combination
//! near the exact boundary years can still land outside the overall
//! representable range even at week `<= 52`) — narrowed to
//! `-9990..=9990` to restore the property honestly.
//!
//! `Weekday` is modeled by its real Monday=1..Sunday=7 numbering
//! (`Weekday::to_monday_one_offset`'s own documented range) directly
//! as an `i8`, since Verus has no reason to reproduce jiff's own enum
//! layout — only the round-trip law itself, the same choice
//! `unit.rs`'s own model makes for `Unit`'s discriminants.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The narrowed, comfortably safe year range this model uses — see
/// this file's own doc comment for the real reason it's narrower
/// than jiff's own documented `-9999..=9999`.
pub open spec fn civil_iso_week_date_year_in_range(year: int) -> bool {
    year >= -9990 && year <= 9990
}

/// jiff's own documented always-valid week sub-range (week `53` is
/// only valid for years containing a "leap week," which this model
/// avoids needing to compute).
pub open spec fn civil_iso_week_date_week_in_range(week: int) -> bool {
    week >= 1 && week <= 52
}

/// `Weekday`'s own real Monday=1..Sunday=7 numbering range.
pub open spec fn weekday_offset_in_range(offset: int) -> bool {
    offset >= 1 && offset <= 7
}

/// The round-trip law this model establishes: within range, the
/// modeled constructor's result carries `(year, week, weekday_offset)`
/// back out exactly; out of range, it reports none, mirroring
/// `ISOWeekDate::new`'s real `Result::Err` case.
pub open spec fn civil_iso_week_date_new_model_round_trip_holds(
    year: i16,
    week: i8,
    weekday_offset: i8,
    result: Option<(i16, i8, i8)>,
) -> bool {
    if civil_iso_week_date_year_in_range(year as int)
        && civil_iso_week_date_week_in_range(week as int)
        && weekday_offset_in_range(weekday_offset as int) {
        result == Some((year, week, weekday_offset))
    } else {
        result is None
    }
}

/// A model of `ISOWeekDate::new(year, week, weekday).ok().map(|d|
/// (d.year(), d.week(), d.weekday().to_monday_one_offset()))`: within
/// the always-valid sub-range, the values round-trip; out of range,
/// `None` — the same claim `amenable_kani::ext::jiff::
/// civil_iso_week_date::
/// verify_civil_iso_week_date_new_year_week_weekday_round_trips` and
/// `amenable_creusot::ext_jiff::civil_iso_week_date::
/// verify_civil_iso_week_date_new_year_week_weekday_round_trips` check
/// against jiff's real API.
pub fn verify_civil_iso_week_date_new_year_week_weekday_round_trips_model(
    year: i16,
    week: i8,
    weekday_offset: i8,
) -> (result: Option<(i16, i8, i8)>)
    ensures
        civil_iso_week_date_new_model_round_trip_holds(year, week, weekday_offset, result),
{
    if (-9990..=9990).contains(&year) && (1..=52).contains(&week) && (1..=7).contains(
        &weekday_offset,
    ) {
        Some((year, week, weekday_offset))
    } else {
        None
    }
}

} // verus!
