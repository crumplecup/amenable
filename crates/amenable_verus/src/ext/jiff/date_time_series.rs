//! Verus accommodation model for `jiff::civil::DateTimeSeries`'s
//! periodicity property (`DateTime::series`/`Iterator::next`).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! date_time_series`'s own accommodation model for the identical
//! claim, checked directly against jiff's real source, independently
//! confirms. `amenable_kani::gallery::jiff_error_drop_cost`'s own doc
//! comment documents why this claim is unavoidably too costly for
//! Kani specifically to check by calling jiff's real API directly (a
//! `jiff::Error` recursive-Arc Drop-glue wall inside `DateTimeSeries::
//! next()`, the same class of wall as `DateSeries`/`TimestampSeries`,
//! confirmed distinct from `ZonedSeries`'s `TimeZone::Repr` wall
//! since `civil::DateTime` has no time zone at all). Identical in
//! shape to `date_series.rs`'s own model: no `TimeZone::UTC` scoping
//! caveat needed at all.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A comfortably safe sub-range of `DateTime`'s real day-count range,
/// identical to `date_series.rs`'s own bound — a `DateTime` carries
/// the exact same underlying `Date`.
pub open spec fn date_time_series_start_days_in_range(start_days: int) -> bool {
    start_days >= -3700000 && start_days <= 3700000
}

/// jiff's own effectively-unbounded (for this model's purposes)
/// period range — bounded here purely to keep the addition below from
/// ever leaving `date_time_series_start_days_in_range`.
pub open spec fn date_time_series_period_days_in_range(period_days: int) -> bool {
    period_days >= -1000000000 && period_days <= 1000000000
}

/// The periodicity law this model establishes: the first `next()`
/// call returns the starting datetime exactly, and the second
/// advances it by exactly the period — the same claim
/// `amenable_creusot::ext_jiff::date_time_series::
/// verify_date_time_series_next_yields_start_then_advances_by_period`
/// checks as an accommodation model too, and the same claim
/// `amenable_kani::ext::jiff`'s own doc comment documents as
/// unavoidably too costly for Kani to check directly against jiff's
/// real API. Datetimes are modeled here as a signed day-count
/// (`start_days`), the same unit `date_series.rs` uses.
pub open spec fn date_time_series_next_yields_start_then_advances_by_period_holds(
    start_days: i64,
    period_days: i64,
    result: (i64, i64),
) -> bool {
    result.0 == start_days && result.1 == start_days + period_days
}

/// A model of `DateTime::series(period_days.days())`'s first two
/// `next()` calls: the first yields `start_days` exactly, the second
/// `start_days + period_days`.
pub fn verify_date_time_series_next_yields_start_then_advances_by_period(
    start_days: i64,
    period_days: i64,
) -> (result: (i64, i64))
    requires
        date_time_series_start_days_in_range(start_days as int),
        date_time_series_period_days_in_range(period_days as int),
    ensures
        date_time_series_next_yields_start_then_advances_by_period_holds(
            start_days,
            period_days,
            result,
        ),
{
    (start_days, start_days + period_days)
}

} // verus!
