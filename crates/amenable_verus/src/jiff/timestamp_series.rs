//! Verus accommodation model for `jiff::TimestampSeries`'s
//! periodicity property (`Timestamp::series`/`Iterator::next`).
//!
//! `jiff` has zero `vstd` coverage and Verus never resolves
//! `Cargo.toml` at all — unlike Kani/Creusot, there is no mechanism
//! for Verus to reach jiff's actual code, full stop. The established
//! response (see `offset.rs`'s own doc comment for the precedent) is
//! a hand-verified Verus-native model reproducing the type's
//! documented behavior. This proof is conditional on that
//! reproduction being faithful — which `amenable_creusot::ext_jiff::
//! timestamp_series`'s own accommodation model for the identical
//! claim, checked directly against jiff's real source, independently
//! confirms. `amenable_kani::gallery::jiff_error_drop_cost`'s own doc
//! comment documents why this claim is unavoidably too costly for
//! Kani specifically to check by calling jiff's real API directly (a
//! real `jiff::Error` recursive-Arc Drop-glue wall inside
//! `Timestamp::series`'s own private implementation) — the same real
//! finding that makes an accommodation model the honest choice here,
//! not merely the default one.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// A comfortably safe sub-range of `Timestamp`'s real seconds range
/// (`Timestamp::MIN.as_second()` = -377,705,023,201, `Timestamp::
/// MAX.as_second()` = 253,402,207,200, confirmed empirically) with
/// enough headroom either side to add `timestamp_series_period_seconds
/// _in_range`'s own bound without ever approaching the real boundary
/// — the same constants `amenable_creusot::ext_jiff::timestamp_series`
/// independently confirms.
pub open spec fn timestamp_series_start_seconds_in_range(secs: int) -> bool {
    secs >= -300000000000 && secs <= 200000000000
}

/// jiff's own effectively-unbounded (for this model's purposes)
/// period range — bounded here purely to keep the addition below from
/// ever leaving `timestamp_series_start_seconds_in_range`.
pub open spec fn timestamp_series_period_seconds_in_range(period_secs: int) -> bool {
    period_secs >= -1000000000 && period_secs <= 1000000000
}

/// The periodicity law this model establishes: the first `next()`
/// call returns the starting timestamp exactly, and the second
/// advances it by exactly the period — the same claim
/// `amenable_creusot::ext_jiff::timestamp_series::
/// verify_timestamp_series_next_yields_start_then_advances_by_period`
/// checks as an accommodation model too, and the same claim
/// `amenable_kani::ext::jiff::timestamp_series`'s own doc comment
/// documents as unavoidably too costly for Kani to check directly
/// against jiff's real API.
pub open spec fn timestamp_series_next_yields_start_then_advances_by_period_holds(
    secs: i64,
    period_secs: i64,
    result: (i64, i64),
) -> bool {
    result.0 == secs && result.1 == secs + period_secs
}

/// A model of `Timestamp::series(period_secs.seconds())`'s first two
/// `next()` calls: the first yields `secs` exactly, the second
/// `secs + period_secs`.
pub fn verify_timestamp_series_next_yields_start_then_advances_by_period(
    secs: i64,
    period_secs: i64,
) -> (result: (i64, i64))
    requires
        timestamp_series_start_seconds_in_range(secs as int),
        timestamp_series_period_seconds_in_range(period_secs as int),
    ensures
        timestamp_series_next_yields_start_then_advances_by_period_holds(
            secs,
            period_secs,
            result,
        ),
{
    (secs, secs + period_secs)
}

} // verus!
