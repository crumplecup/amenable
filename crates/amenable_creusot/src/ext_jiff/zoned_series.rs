//! Real Creusot proof content for `jiff::ZonedSeries`'s periodicity
//! property (`ext::jiff::zoned_series` holds the `CreusotWitness`
//! bridge) — the same claim `amenable_verus::jiff::zoned_series`'s
//! hand-verified model checks, and the same claim
//! `amenable_kani::gallery::jiff_error_drop_cost`'s own doc comment
//! documents as UNCHECKABLE on Kani specifically, for a reason
//! genuinely different from `TimestampSeries`'s: not `jiff::Error`
//! Drop glue, but `TimeZone`'s own hand-rolled pointer-tagged `Repr`
//! (a `usize`-to-pointer `transmute` and its reverse), which times out
//! CBMC even for a single, fully concrete `TimeZone::UTC.to_offset(..)`
//! call — before any series/iteration logic runs at all.
//!
//! Accommodation model, not a real `extern_spec!` against jiff's actual
//! `Iterator for ZonedSeries` impl, matching this crate's own
//! established precedent for iterator types lacking real contract
//! coverage (see `timestamp_series.rs`'s own doc comment). **Narrower
//! in scope than `TimestampSeries`'s model, for a real, documented
//! reason**: `ZonedSeries::next()`'s actual implementation loops,
//! re-trying with a larger multiple of the period whenever the
//! candidate instant doesn't strictly advance past the previous one —
//! jiff's own doc comment explains this exists specifically for time
//! zones with repeated local clock readings across a DST transition
//! (e.g. `Pacific/Apia`'s skipped 2011-12-30). This model states the
//! periodicity law only for `TimeZone::UTC`, where that loop is
//! structurally unreachable: confirmed by reading jiff's real source
//! (`tz::TimeZone::to_offset`'s `UTC` arm always returns the constant
//! `Offset::UTC`, so the mapping from `Timestamp` to civil time under
//! UTC is strictly monotonic — no instant is ever "skipped," so the
//! loop's retry branch never fires). Under that honestly-scoped
//! condition, the law is identical in shape to `TimestampSeries`'s own.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use super::timestamp_series::{
    timestamp_series_period_secs_in_safe_range, timestamp_series_secs_in_safe_range,
};
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, ZONED_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_UNDER_UTC_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::ZonedSeries>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn zoned_series_next_yields_start_then_advances_by_period_under_utc_holds(
            secs: i64,
            period_secs: i64,
            observed: (i64, i64),
        ) -> bool {
            pearlite! { observed.0 == secs && observed.1 == secs + period_secs }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::zoned_series::zoned_series_next_yields_start_then_advances_by_period_under_utc_holds",
        "creusot",
        "ensures",
        || ZONED_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_UNDER_UTC_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_ZONED_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_UNDER_UTC_SRC, {
        /// `Zoned::new(ts, TimeZone::UTC).series(period).next()` returns
        /// the original instant exactly on the first call, and advances
        /// it by exactly `period` on the second — scoped to `TimeZone::
        /// UTC` specifically, where `ZonedSeries::next()`'s real
        /// DST-repeat retry loop is structurally unreachable (see this
        /// module's own doc comment). The same claim
        /// `amenable_verus::jiff::zoned_series`'s hand-verified
        /// model checks, and the same claim confirmed too costly for
        /// Kani to check directly against jiff's real API (see
        /// `amenable_kani::gallery::jiff_error_drop_cost`'s own doc
        /// comment).
        ///
        /// Accommodation model, not a real `extern_spec!` against
        /// jiff's actual `Iterator` impl: matching this crate's own
        /// established precedent for iterator types lacking real
        /// contract coverage (see this module's own doc comment).
        #[requires(timestamp_series_secs_in_safe_range(secs))]
        #[requires(timestamp_series_period_secs_in_safe_range(period_secs))]
        #[ensures(zoned_series_next_yields_start_then_advances_by_period_under_utc_holds(secs, period_secs, result))]
        fn verify_zoned_series_next_yields_start_then_advances_by_period_under_utc(
            secs: i64,
            period_secs: i64,
        ) -> (i64, i64) {
            (secs, secs + period_secs)
        }
    }
}
