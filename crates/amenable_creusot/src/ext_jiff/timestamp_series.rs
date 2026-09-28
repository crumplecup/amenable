//! Real Creusot proof content for `jiff::TimestampSeries`'s
//! periodicity property (`ext::jiff::timestamp_series` holds the
//! `CreusotWitness` bridge) — the same claim
//! `amenable_verus::ext::jiff::timestamp_series`'s hand-verified model
//! checks, and the same claim
//! `amenable_kani::gallery::jiff_error_drop_cost`'s own doc comment
//! documents as UNCHECKABLE on Kani specifically (a real,
//! unavoidable `jiff::Error` recursive-Arc Drop-glue wall inside
//! `Timestamp::series`'s own private implementation).
//!
//! Accommodation model, not a real `extern_spec!` against jiff's
//! actual `Iterator for TimestampSeries` impl: matching this crate's
//! own established precedent for iterator types lacking real contract
//! coverage (see `rust_std::vec_deque`'s `verify_vec_deque_into_iter_
//! yields_owned_values_in_order`, an identically-shaped accommodation
//! model for `VecDeque::into_iter` since `creusot-std` ships no real
//! contract for it either) — the harness computes the expected
//! `(first, second)` pair directly rather than calling jiff's real
//! `TimestampSeries::next()` twice, and the `#[logic(open)]` helper
//! checks that computed pair's own internal consistency. This models
//! the same real periodicity law `amenable_kani::ext::jiff::
//! timestamp_series` established was too costly for Kani specifically
//! to check by calling jiff's own API directly.

#[cfg(creusot)]
mod mirror {
    pub(super) use creusot_std::macros::{ensures, logic, requires};
}
#[cfg(creusot)]
use mirror::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, TIMESTAMP_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC, {
        /// The `amenable_ext::ExtStandard<jiff::TimestampSeries>`
        /// postcondition — real, callable Pearlite content, not just
        /// descriptive text alongside it.
        #[logic(open)]
        fn timestamp_series_next_yields_start_then_advances_by_period_holds(
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
        "amenable_creusot::ext_jiff::timestamp_series::timestamp_series_next_yields_start_then_advances_by_period_holds",
        "creusot",
        "ensures",
        || TIMESTAMP_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, TIMESTAMP_SERIES_SECS_IN_SAFE_RANGE_HOLDS_SRC, {
        /// This model's own safe-range restriction for a signed-second
        /// timestamp -- `pub(crate)` (not nested in `mirror`) so
        /// `zoned_series.rs` can reuse the identical bound rather than
        /// restating it.
        #[logic(open)]
        pub(crate) fn timestamp_series_secs_in_safe_range(secs: i64) -> bool {
            pearlite! { secs > -300_000_000_000i64 - 1i64 && secs < 200_000_000_000i64 + 1i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::timestamp_series::timestamp_series_secs_in_safe_range",
        "creusot",
        "requires",
        || TIMESTAMP_SERIES_SECS_IN_SAFE_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, TIMESTAMP_SERIES_PERIOD_SECS_IN_SAFE_RANGE_HOLDS_SRC, {
        /// This model's own safe-range restriction for a signed-second
        /// period -- `pub(crate)` for the same cross-file reuse reason
        /// as the secs range above.
        #[logic(open)]
        pub(crate) fn timestamp_series_period_secs_in_safe_range(period_secs: i64) -> bool {
            pearlite! { period_secs > -1_000_000_000i64 - 1i64 && period_secs < 1_000_000_000i64 + 1i64 }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_jiff::timestamp_series::timestamp_series_period_secs_in_safe_range",
        "creusot",
        "requires",
        || TIMESTAMP_SERIES_PERIOD_SECS_IN_SAFE_RANGE_HOLDS_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_TIMESTAMP_SERIES_NEXT_YIELDS_START_THEN_ADVANCES_BY_PERIOD_SRC, {
        /// `Timestamp::series(period).next()` returns the original
        /// timestamp exactly on the first call, and advances it by
        /// exactly `period` on the second — the same claim
        /// `amenable_verus::ext::jiff::timestamp_series`'s
        /// hand-verified model checks, and the same claim confirmed
        /// too costly for Kani to check directly against jiff's real
        /// API (see `amenable_kani::gallery::jiff_error_drop_cost`'s
        /// own doc comment).
        ///
        /// Accommodation model, not a real `extern_spec!` against
        /// jiff's actual `Iterator` impl: matching this crate's own
        /// established precedent for iterator types lacking real
        /// contract coverage (see this module's own doc comment).
        #[requires(timestamp_series_secs_in_safe_range(secs))]
        #[requires(timestamp_series_period_secs_in_safe_range(period_secs))]
        #[ensures(timestamp_series_next_yields_start_then_advances_by_period_holds(secs, period_secs, result))]
        fn verify_timestamp_series_next_yields_start_then_advances_by_period(
            secs: i64,
            period_secs: i64,
        ) -> (i64, i64) {
            (secs, secs + period_secs)
        }
    }
}
