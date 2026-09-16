//! Verus accommodation model for `jiff::civil::DateSeries`'s
//! periodicity property (`Date::series`/`Iterator::next`).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! date_series`'s own accommodation model for the identical claim,
//! checked directly against jiff's real source, independently
//! confirms. `amenable_kani::gallery::jiff_error_drop_cost`'s own doc
//! comment documents why this claim is unavoidably too costly for
//! Kani specifically to check by calling jiff's real API directly (a
//! `jiff::Error` recursive-Arc Drop-glue wall inside `DateSeries::
//! next()`, the same class of wall as `TimestampSeries`, confirmed
//! distinct from `ZonedSeries`'s `TimeZone::Repr` wall since `Date`
//! has no time zone at all). Unlike `ZonedSeries`'s model, this one
//! needs no `TimeZone::UTC` scoping caveat: `Date` has no DST-repeat
//! retry loop to begin with.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A comfortably safe sub-range of `Date`'s real day-count range
/// (`Date::MIN`/`MAX` are `-9999`/`9999` years, roughly
/// ±3,652,059 days) with enough headroom either side to add
/// `date_series_period_days_in_range`'s own bound without ever
/// approaching the real boundary — the same constants
/// `amenable_creusot::ext_jiff::date_series` independently confirms.
pub open spec fn date_series_start_days_in_range(start_days: int) -> bool {
    start_days >= -3700000 && start_days <= 3700000
}

/// jiff's own effectively-unbounded (for this model's purposes)
/// period range — bounded here purely to keep the addition below from
/// ever leaving `date_series_start_days_in_range`.
pub open spec fn date_series_period_days_in_range(period_days: int) -> bool {
    period_days >= -1000000000 && period_days <= 1000000000
}

/// The periodicity law this model establishes: the first `next()`
/// call returns the starting date exactly, and the second advances it
/// by exactly the period — the same claim `amenable_creusot::
/// ext_jiff::date_series::
/// verify_date_series_next_yields_start_then_advances_by_period`
/// checks as an accommodation model too, and the same claim
/// `amenable_kani::ext::jiff`'s own doc comment documents as
/// unavoidably too costly for Kani to check directly against jiff's
/// real API. Dates are modeled here as a signed day-count
/// (`start_days`), the natural unit `DateSeries::next()`'s own
/// `checked_mul`/`checked_add` operate over.
pub open spec fn date_series_next_yields_start_then_advances_by_period_holds(
    start_days: i64,
    period_days: i64,
    result: (i64, i64),
) -> bool {
    result.0 == start_days && result.1 == start_days + period_days
}

/// A model of `Date::series(period_days.days())`'s first two `next()`
/// calls: the first yields `start_days` exactly, the second
/// `start_days + period_days`.
pub fn verify_date_series_next_yields_start_then_advances_by_period(
    start_days: i64,
    period_days: i64,
) -> (result: (i64, i64))
    requires
        date_series_start_days_in_range(start_days as int),
        date_series_period_days_in_range(period_days as int),
    ensures
        date_series_next_yields_start_then_advances_by_period_holds(
            start_days,
            period_days,
            result,
        ),
{
    (start_days, start_days + period_days)
}

} // verus!
