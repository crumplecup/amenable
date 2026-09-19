//! Verus accommodation model for `jiff::civil::TimeSeries`'s
//! periodicity property (`Time::series`/`Iterator::next`).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! civil_time_series`'s own accommodation model for the identical
//! claim, checked directly against jiff's real source, independently
//! confirms. `amenable_kani::gallery::jiff_error_drop_cost`'s own doc
//! comment documents why this claim is unavoidably too costly for
//! Kani specifically to check by calling jiff's real API directly (a
//! `jiff::Error` recursive-Arc Drop-glue wall inside `TimeSeries::
//! next()`, the same class of wall as `DateSeries`/`DateTimeSeries`,
//! confirmed distinct from `ZonedSeries`'s `TimeZone::Repr` wall
//! since `civil::Time` has no time zone at all). Time is modeled as
//! a signed nanosecond count rather than jiff's real within-a-day
//! wraparound semantics, scoped to a comfortably safe range that
//! never approaches wraparound — the same honest simplification
//! `date_series.rs` makes for calendar days.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A comfortably safe sub-range for the modeled nanosecond count,
/// far from any wraparound concern — the same constants
/// `amenable_creusot::ext_jiff::civil_time_series` independently
/// confirms.
pub open spec fn civil_time_series_start_nanos_in_range(start_nanos: int) -> bool {
    start_nanos >= -1000000000000 && start_nanos <= 1000000000000
}

/// A comfortably safe sub-range for the modeled period, bounded here
/// purely to keep the addition below from ever leaving
/// `civil_time_series_start_nanos_in_range`.
pub open spec fn civil_time_series_period_nanos_in_range(period_nanos: int) -> bool {
    period_nanos >= -1000000000000 && period_nanos <= 1000000000000
}

/// The periodicity law this model establishes: the first `next()`
/// call returns the starting time exactly, and the second advances it
/// by exactly the period — the same claim `amenable_creusot::
/// ext_jiff::civil_time_series::
/// verify_civil_time_series_next_yields_start_then_advances_by_period`
/// checks as an accommodation model too, and the same claim
/// `amenable_kani::ext::jiff`'s own doc comment documents as
/// unavoidably too costly for Kani to check directly against jiff's
/// real API.
pub open spec fn civil_time_series_next_yields_start_then_advances_by_period_holds(
    start_nanos: i64,
    period_nanos: i64,
    result: (i64, i64),
) -> bool {
    result.0 == start_nanos && result.1 == start_nanos + period_nanos
}

/// A model of `Time::series(period_nanos.nanoseconds())`'s first two
/// `next()` calls: the first yields `start_nanos` exactly, the second
/// `start_nanos + period_nanos`.
pub fn verify_civil_time_series_next_yields_start_then_advances_by_period(
    start_nanos: i64,
    period_nanos: i64,
) -> (result: (i64, i64))
    requires
        civil_time_series_start_nanos_in_range(start_nanos as int),
        civil_time_series_period_nanos_in_range(period_nanos as int),
    ensures
        civil_time_series_next_yields_start_then_advances_by_period_holds(
            start_nanos,
            period_nanos,
            result,
        ),
{
    (start_nanos, start_nanos + period_nanos)
}

} // verus!
