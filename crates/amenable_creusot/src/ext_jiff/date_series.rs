//! Real Creusot proof content for `jiff::civil::DateSeries`'s
//! periodicity property (`ext::jiff::date_series` holds the
//! `CreusotWitness` bridge) — the same claim
//! `amenable_verus::ext::jiff::date_series`'s hand-verified model
//! checks, and the same claim `amenable_kani::ext::jiff`'s own doc
//! comment documents as UNCHECKABLE on Kani specifically (the same
//! `jiff::Error` recursive-Arc Drop-glue wall `TimestampSeries` hits,
//! confirmed distinct from `ZonedSeries`'s `TimeZone::Repr` wall,
//! since `Date` has no time zone at all).
//!
//! Accommodation model, not a real `extern_spec!` against jiff's
//! actual `Iterator for DateSeries` impl, matching this crate's own
//! established precedent for iterator types lacking real contract
//! coverage (see `timestamp_series.rs`'s own doc comment). Unlike
//! `ZonedSeries`'s model, this one needs NO `TimeZone::UTC` scoping
//! caveat at all: `DateSeries::next()` has no DST-repeat retry loop
//! to begin with (confirmed by reading jiff's real source — `Date`
//! has no time zone, so there is no such thing as a repeated local
//! clock reading to retry past), so the periodicity law is exactly as
//! simple as `TimestampSeries`'s own.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, DATE_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::civil::DateSeries>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn date_series_next_yields_start_then_advances_by_period_holds(
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
        "amenable_creusot::ext_jiff::date_series::date_series_next_yields_start_then_advances_by_period_holds",
        "creusot",
        "ensures",
        || DATE_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, DATE_SERIES_START_DAYS_IN_SAFE_RANGE_HOLDS_SRC, {
        /// This model's own safe-range restriction for a signed
        /// day-count start point -- `pub(crate)` (not nested in
        /// `mirror`) so `date_time_series.rs` can reuse the identical
        /// bound rather than restating it.
        #[logic(open)]
        pub(crate) fn date_series_start_days_in_safe_range(days: i64) -> bool {
            pearlite! { days > -3_700_000i64 - 1i64 && days < 3_700_000i64 + 1i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::date_series::date_series_start_days_in_safe_range",
        "creusot",
        "requires",
        || DATE_SERIES_START_DAYS_IN_SAFE_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, DATE_SERIES_PERIOD_DAYS_IN_SAFE_RANGE_HOLDS_SRC, {
        /// This model's own safe-range restriction for a signed
        /// day-count period -- `pub(crate)` for the same cross-file
        /// reuse reason as the start-days range above.
        #[logic(open)]
        pub(crate) fn date_series_period_days_in_safe_range(days: i64) -> bool {
            pearlite! { days > -1_000_000_000i64 - 1i64 && days < 1_000_000_000i64 + 1i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::date_series::date_series_period_days_in_safe_range",
        "creusot",
        "requires",
        || DATE_SERIES_PERIOD_DAYS_IN_SAFE_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_DATE_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC, {
        /// `Date::series(period).next()` returns the original date
        /// exactly on the first call, and advances it by exactly
        /// `period` on the second — the same claim
        /// `amenable_verus::ext::jiff::date_series`'s hand-verified
        /// model checks, and the same claim confirmed too costly for
        /// Kani to check directly against jiff's real API (see
        /// `amenable_kani::gallery::jiff_error_drop_cost`'s own doc
        /// comment). Dates are modeled here as a signed day-count
        /// (`start_days`), the natural unit `DateSeries::next()`'s own
        /// `checked_mul`/`checked_add` operate over.
        ///
        /// Accommodation model, not a real `extern_spec!` against
        /// jiff's actual `Iterator` impl: matching this crate's own
        /// established precedent for iterator types lacking real
        /// contract coverage (see this module's own doc comment).
        #[requires(date_series_start_days_in_safe_range(start_days))]
        #[requires(date_series_period_days_in_safe_range(period_days))]
        #[ensures(date_series_next_yields_start_then_advances_by_period_holds(start_days, period_days, result))]
        fn verify_date_series_next_yields_start_then_advances_by_period(
            start_days: i64,
            period_days: i64,
        ) -> (i64, i64) {
            (start_days, start_days + period_days)
        }
    }
}
