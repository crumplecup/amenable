//! Creusot proof content for `chrono::NaiveWeek`'s span claim, over an
//! Amenable-owned model, the same shape `civil_naive_date.rs`'s own model
//! uses.
//!
//! `NaiveWeek` has no public accessor for its own inner date, so there is no
//! way to extern_spec `checked_first_day`/`checked_last_day` and relate their
//! results to each other directly, the way `FixedOffset`'s and `NaiveTime`'s
//! own witnesses extern_spec chrono's real functions. A model is the honest
//! alternative, the same choice `NaiveDate`'s own witness already makes for a
//! different real reason (chrono's internals aren't visible to Creusot at
//! all).
//!
//! The model is the same one `amenable_verus::chrono::naive_week`'s own
//! Verus model already validated: a date is represented abstractly by its
//! signed day-count, and `back`/`forward` (days from the week's start to this
//! date, and from this date to the week's end) are the two complementary
//! halves of a 7-day week, always summing to six by construction. The proof
//! holds for the model over every input; it holds for chrono's
//! `checked_first_day`/`checked_last_day` only under the refinement premise
//! stated on the witness bridge, which `amenable_kani::chrono::
//! week_span_partitions`'s own 100 year-partitioned harnesses, checked
//! directly against chrono's real API, independently confirm.

#[cfg(creusot)]
use creusot_std::macros::{ensures, logic, requires};

amenable_derive::harness! {
    creusot, NAIVE_WEEK_SPAN_HOLDS_SRC, {
        /// The claim: when both ends exist, they span exactly six days.
        /// Holds vacuously when either end is absent.
        #[logic(open)]
        pub fn naive_week_span_holds(first: Option<i64>, last: Option<i64>) -> bool {
            pearlite! {
                match (first, last) {
                    (Some(f), Some(l)) => l - f == 6i64,
                    _ => true,
                }
            }
        }
    }
}

amenable_derive::harness! {
    creusot, NAIVE_WEEK_SPAN_MODEL_INPUTS_VALID_SRC, {
        /// `verify_naive_week_span_model`'s own precondition: `back` is one of
        /// the seven possible weekday-to-week-start offsets, `min_day` is at
        /// most `max_day`, and all three day-count values stay within
        /// `±100_000_000` (so `i64` arithmetic can't overflow near the edges;
        /// chrono's own day-count range is roughly `±95_600_000`, comfortably
        /// inside that margin, not narrowed by it). Named so the harness's
        /// `requires` points at a real, registered contract fragment instead of
        /// a raw conjunction.
        #[logic(open)]
        pub fn naive_week_span_model_inputs_valid(day_count: i64, back: i64, min_day: i64, max_day: i64) -> bool {
            pearlite! {
                back >= 0i64 && back <= 6i64 && min_day <= max_day
                    && day_count >= -100_000_000i64 && day_count <= 100_000_000i64
                    && min_day >= -100_000_000i64 && min_day <= 100_000_000i64
                    && max_day >= -100_000_000i64 && max_day <= 100_000_000i64
            }
        }
    }
}

#[cfg(not(creusot))]
::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_creusot::ext_chrono::naive_week::naive_week_span_model_inputs_valid",
        "creusot",
        "requires",
        || NAIVE_WEEK_SPAN_MODEL_INPUTS_VALID_SRC,
    )
}

amenable_derive::harness! {
    creusot, VERIFY_NAIVE_WEEK_SPAN_MODEL_SRC, {
        /// A model of `(week.checked_first_day(), week.checked_last_day())`:
        /// `day_count` stands for the date's position on the calendar's own
        /// number line, `back` for how many days separate it from the
        /// week's start, and `min_day`/`max_day` for the representable
        /// range's own bounds — see `naive_week_span_model_inputs_valid`'s
        /// own doc comment for the exact precondition.
        #[requires(naive_week_span_model_inputs_valid(day_count, back, min_day, max_day))]
        #[ensures(naive_week_span_holds(result.0, result.1))]
        fn verify_naive_week_span_model(
            day_count: i64,
            back: i64,
            min_day: i64,
            max_day: i64,
        ) -> (Option<i64>, Option<i64>) {
            let forward = 6i64 - back;
            let first = if day_count - back >= min_day { Some(day_count - back) } else { None };
            let last = if day_count + forward <= max_day { Some(day_count + forward) } else { None };
            (first, last)
        }
    }
}
