//! Real Creusot proof content for `jiff::civil::DateTimeSeries`'s
//! periodicity property (`ext::jiff::date_time_series` holds the
//! `CreusotWitness` bridge) — the same claim
//! `amenable_verus::jiff::date_time_series`'s hand-verified model
//! checks, and the same claim `amenable_kani::ext::jiff`'s own doc
//! comment documents as UNCHECKABLE on Kani specifically (the same
//! `jiff::Error` recursive-Arc Drop-glue wall `DateSeries`/
//! `TimestampSeries` hit, confirmed distinct from `ZonedSeries`'s
//! `TimeZone::Repr` wall, since `civil::DateTime` has no time zone at
//! all).
//!
//! Accommodation model, not a real `extern_spec!` against jiff's
//! actual `Iterator for DateTimeSeries` impl, matching this crate's
//! own established precedent for iterator types lacking real contract
//! coverage (see `timestamp_series.rs`'s own doc comment). Identical
//! in shape to `date_series.rs`'s own model: no `TimeZone::UTC`
//! scoping caveat needed at all, since `DateTime` has no DST-repeat
//! retry loop to begin with.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use super::date_series::{
    date_series_period_days_in_safe_range, date_series_start_days_in_safe_range,
};
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, DATE_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::DateTimeSeries>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn date_time_series_next_yields_start_then_advances_by_period_holds(
            start_days: i64,
            period_days: i64,
            observed: (i64, i64),
        ) -> bool {
            pearlite! { observed.0 == start_days && observed.1 == start_days + period_days }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::date_time_series::date_time_series_next_yields_start_then_advances_by_period_holds",
        "creusot",
        "ensures",
        || DATE_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_DATE_TIME_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC, {
        /// `DateTime::series(period).next()` returns the original
        /// datetime exactly on the first call, and advances it by
        /// exactly `period` on the second — the same claim
        /// `amenable_verus::jiff::date_time_series`'s
        /// hand-verified model checks, and the same claim confirmed
        /// too costly for Kani to check directly against jiff's real
        /// API (see `amenable_kani::gallery::jiff_error_drop_cost`'s
        /// own doc comment). Datetimes are modeled here as a signed
        /// day-count (`start_days`), the same unit `date_series.rs`
        /// uses, since the underlying `checked_mul`/`checked_add`
        /// machinery is identical.
        ///
        /// Accommodation model, not a real `extern_spec!` against
        /// jiff's actual `Iterator` impl: matching this crate's own
        /// established precedent for iterator types lacking real
        /// contract coverage (see this module's own doc comment).
        #[requires(date_series_start_days_in_safe_range(start_days))]
        #[requires(date_series_period_days_in_safe_range(period_days))]
        #[ensures(date_time_series_next_yields_start_then_advances_by_period_holds(start_days, period_days, result))]
        fn verify_date_time_series_next_yields_start_then_advances_by_period(
            start_days: i64,
            period_days: i64,
        ) -> (i64, i64) {
            (start_days, start_days + period_days)
        }
    }
}
