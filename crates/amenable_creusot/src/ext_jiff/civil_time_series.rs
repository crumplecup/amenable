//! Real Creusot proof content for `jiff::civil::TimeSeries`'s
//! periodicity property (`ext::jiff::civil_time_series` holds the
//! `CreusotWitness` bridge) — the same claim
//! `amenable_verus::ext::jiff::civil_time_series`'s hand-verified
//! model checks, and the same claim `amenable_kani::ext::jiff`'s own
//! doc comment documents as UNCHECKABLE on Kani specifically (the
//! same `jiff::Error` recursive-Arc Drop-glue wall `DateSeries`/
//! `DateTimeSeries`/`TimestampSeries` hit, confirmed distinct from
//! `ZonedSeries`'s `TimeZone::Repr` wall, since `civil::Time` has no
//! time zone at all).
//!
//! Accommodation model, not a real `extern_spec!` against jiff's
//! actual `Iterator for TimeSeries` impl, matching this crate's own
//! established precedent for iterator types lacking real contract
//! coverage (see `timestamp_series.rs`'s own doc comment). Identical
//! in shape to `date_series.rs`'s/`date_time_series.rs`'s own models:
//! no `TimeZone::UTC` scoping caveat needed at all, since `Time` has
//! no DST-repeat retry loop to begin with. Time is modeled as a
//! signed nanosecond count (`start_nanos`) rather than jiff's real
//! `checked_add_span`'s wraparound-within-a-day semantics — this
//! model states the abstract periodicity law over a comfortably safe
//! range that never approaches wraparound, the same honest
//! simplification `date_series.rs` makes for calendar days.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, CIVIL_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::TimeSeries>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn civil_time_series_next_yields_start_then_advances_by_period_holds(
            start_nanos: i64,
            period_nanos: i64,
            observed: (i64, i64),
        ) -> bool {
            pearlite! { observed.0 == start_nanos && observed.1 == start_nanos + period_nanos }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_time_series::civil_time_series_next_yields_start_then_advances_by_period_holds",
        "creusot",
        "ensures",
        || CIVIL_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, CIVIL_TIME_SERIES_NANOS_IN_SAFE_RANGE_HOLDS_SRC, {
        /// This model's own safe-range restriction for a signed
        /// nanosecond count -- comfortably clear of wraparound, the
        /// same bound this file's own doc comment describes -- named
        /// so both `#[requires(..)]` clauses point at one real
        /// predicate instead of restating the bound twice.
        #[logic(open)]
        fn civil_time_series_nanos_in_safe_range(nanos: i64) -> bool {
            pearlite! { nanos > -1_000_000_000_000i64 - 1i64 && nanos < 1_000_000_000_000i64 + 1i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::civil_time_series::civil_time_series_nanos_in_safe_range",
        "creusot",
        "requires",
        || CIVIL_TIME_SERIES_NANOS_IN_SAFE_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_CIVIL_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC, {
        /// `Time::series(period).next()` returns the original time
        /// exactly on the first call, and advances it by exactly
        /// `period` on the second — the same claim
        /// `amenable_verus::ext::jiff::civil_time_series`'s
        /// hand-verified model checks, and the same claim confirmed
        /// too costly for Kani to check directly against jiff's real
        /// API (see `amenable_kani::gallery::jiff_error_drop_cost`'s
        /// own doc comment).
        ///
        /// Accommodation model, not a real `extern_spec!` against
        /// jiff's actual `Iterator` impl: matching this crate's own
        /// established precedent for iterator types lacking real
        /// contract coverage (see this module's own doc comment).
        #[requires(civil_time_series_nanos_in_safe_range(start_nanos))]
        #[requires(civil_time_series_nanos_in_safe_range(period_nanos))]
        #[ensures(civil_time_series_next_yields_start_then_advances_by_period_holds(start_nanos, period_nanos, result))]
        fn verify_civil_time_series_next_yields_start_then_advances_by_period(
            start_nanos: i64,
            period_nanos: i64,
        ) -> (i64, i64) {
            (start_nanos, start_nanos + period_nanos)
        }
    }
}
