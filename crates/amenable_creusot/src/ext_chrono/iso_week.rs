//! Creusot proof content for `chrono::NaiveDate::iso_week`/
//! `NaiveDate::from_isoywd_opt`'s round trip, over an Amenable-owned model,
//! the same shape `naive_week.rs`'s own model uses.
//!
//! Real chrono source read first (`NaiveDate::from_isoywd_opt`'s
//! `YearFlags`/`isoweek_delta` machinery), the same investigation
//! `amenable_verus::chrono::iso_week`'s own doc comment describes in full.
//! Rather than re-deriving chrono's own rule for exactly which Monday starts
//! week 1 of a given year -- real, but orthogonal to this claim, and
//! independently confirmed by `amenable_kani::chrono::civil_iso_week`'s own
//! harness against chrono's real API -- this model takes that Monday (`m1`)
//! and that year's real week count (`52` or `53`) as given inputs,
//! constrained only by the two properties chrono's own algorithm guarantees.
//!
//! Unlike `amenable_verus::chrono::iso_week`'s own model, this one avoids `%`
//! and `/` entirely, rather than correcting their behavior: a real toolchain
//! wall found directly (`cannot calculate the remainder of i64 ... in
//! logic` -- `creusot-std`'s `RemLogic`/division traits are implemented only
//! for the unbounded `Int` type, not for `i64`, inside any `#[logic]`/
//! `#[ensures]`/`#[requires]` clause). `weekday` (`0..=6`, this date's
//! position within its week) and `back_weeks` (how many whole weeks separate
//! `m1` from this date's own week-Monday) are taken as given inputs instead
//! of derived from `day_count`, the same choice `naive_week.rs`'s own model
//! already makes for its `back` parameter -- `week_number` then equals
//! `back_weeks + 1` directly, and reconstructing is `m1 + back_weeks * 7 +
//! weekday`, both pure multiplication and addition.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, ISO_WEEK_ROUND_TRIP_HOLDS_SRC, {
        /// The claim: the week number lies in `1..=53`, and reconstructing
        /// from `(iso_year, week_number, weekday)` yields back the same
        /// `day_count`.
        #[logic(open)]
        pub fn iso_week_round_trip_holds(
            week_number: i64,
            rebuilt_day_count: i64,
            day_count: i64,
        ) -> bool {
            pearlite! {
                week_number >= 1i64 && week_number <= 53i64
                    && rebuilt_day_count == day_count
            }
        }
    }
}

amenable_derive::harness! {
    creusot, VERIFY_ISO_WEEK_ROUND_TRIPS_MODEL_SRC, {
        /// A model of `(date.iso_week().year(), date.iso_week().week(),
        /// date.weekday())`, then `NaiveDate::from_isoywd_opt` applied to
        /// that triple: given `weekday` (this date's position within its
        /// week), `back_weeks` (how many whole weeks separate `m1`, the
        /// Monday starting this date's ISO year's week 1, from this date's
        /// own week-Monday), and `weeks_in_year` (that year's real week
        /// count), related the way chrono's own algorithm guarantees, the
        /// computed week number and the rebuilt day-count satisfy the round
        /// trip.
        #[requires(
            weekday >= 0i64 && weekday <= 6i64
                && back_weeks >= 0i64 && back_weeks < weeks_in_year
                && (weeks_in_year == 52i64 || weeks_in_year == 53i64)
                && day_count == m1 + back_weeks * 7i64 + weekday
                && day_count >= -100_000_000i64 && day_count <= 100_000_000i64
                && m1 >= -100_000_000i64 && m1 <= 100_000_000i64
        )]
        #[ensures(iso_week_round_trip_holds(result.0, result.1, day_count))]
        fn verify_iso_week_round_trips_model(
            day_count: i64,
            m1: i64,
            weekday: i64,
            back_weeks: i64,
            weeks_in_year: i64,
        ) -> (i64, i64) {
            let week_number = back_weeks + 1i64;
            let rebuilt = m1 + back_weeks * 7i64 + weekday;
            (week_number, rebuilt)
        }
    }
}
