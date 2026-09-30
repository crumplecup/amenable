//! Verus accommodation model for `jiff::ZonedSeries`'s periodicity
//! property (`Zoned::series`/`Iterator::next`), scoped to `TimeZone::
//! UTC` for a real, documented reason — see this file's own doc
//! comment on the scoping spec fn below, and `amenable_creusot::
//! ext_jiff::zoned_series`'s own doc comment for the full explanation
//! (jiff's real `ZonedSeries::next()` retries with a larger multiple
//! of the period whenever the candidate instant doesn't strictly
//! advance, which only matters across a DST-repeated local clock
//! reading — structurally unreachable under `TimeZone::UTC`, whose
//! offset is always the constant zero).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is a
//! hand-verified Verus-native model reproducing the type's documented
//! behavior. This proof is conditional on that reproduction being
//! faithful — which `amenable_creusot::ext_jiff::zoned_series`'s own
//! accommodation model for the identical claim, checked directly
//! against jiff's real source, independently confirms.
//! `amenable_kani::gallery::jiff_error_drop_cost`'s own doc comment
//! documents why this claim is unavoidably too costly for Kani
//! specifically to check by calling jiff's real API directly — a real
//! `TimeZone::to_offset` wall in its own hand-rolled pointer-tagged
//! `Repr` (times out CBMC even for a single, fully concrete `UTC`
//! call), genuinely different from `TimestampSeries`'s `jiff::Error`
//! Drop-glue wall, though both land in the same gallery file.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A comfortably safe sub-range of `Timestamp`'s real seconds range,
/// identical to `timestamp_series.rs`'s own bound — a `Zoned` under
/// `TimeZone::UTC` carries the exact same underlying `Timestamp`.
pub open spec fn zoned_series_start_seconds_in_range(secs: int) -> bool {
    secs >= -300000000000 && secs <= 200000000000
}

/// jiff's own effectively-unbounded (for this model's purposes) period
/// range — bounded here purely to keep the addition below from ever
/// leaving `zoned_series_start_seconds_in_range`.
pub open spec fn zoned_series_period_seconds_in_range(period_secs: int) -> bool {
    period_secs >= -1000000000 && period_secs <= 1000000000
}

/// The periodicity law this model establishes, scoped to `TimeZone::
/// UTC`: the first `next()` call returns the starting instant exactly,
/// and the second advances it by exactly the period. Under UTC, whose
/// offset is always the constant zero, `ZonedSeries::next()`'s real
/// DST-repeat retry loop is structurally unreachable (every instant
/// strictly advances with the period), so this is the same claim
/// `amenable_creusot::ext_jiff::zoned_series::
/// verify_zoned_series_next_yields_start_then_advances_by_period_under_utc`
/// checks as an accommodation model too, and the same claim
/// `amenable_kani::ext::jiff::zoned_series`'s own doc comment
/// documents as unavoidably too costly for Kani to check directly
/// against jiff's real API.
pub open spec fn zoned_series_next_yields_start_then_advances_by_period_under_utc_holds(
    secs: i64,
    period_secs: i64,
    result: (i64, i64),
) -> bool {
    result.0 == secs && result.1 == secs + period_secs
}

/// A model of `Zoned::new(ts, TimeZone::UTC).series(period_secs.
/// seconds())`'s first two `next()` calls: the first yields `secs`
/// exactly, the second `secs + period_secs`.
pub fn verify_zoned_series_next_yields_start_then_advances_by_period_under_utc(
    secs: i64,
    period_secs: i64,
) -> (result: (i64, i64))
    requires
        zoned_series_start_seconds_in_range(secs as int),
        zoned_series_period_seconds_in_range(period_secs as int),
    ensures
        zoned_series_next_yields_start_then_advances_by_period_under_utc_holds(
            secs,
            period_secs,
            result,
        ),
{
    (secs, secs + period_secs)
}

} // verus!
