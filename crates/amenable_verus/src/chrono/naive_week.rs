//! Verus accommodation model for `chrono::NaiveWeek`'s span claim: when
//! both `checked_first_day()` and `checked_last_day()` exist, they span
//! exactly six days.
//!
//! `chrono` has zero `vstd` coverage and Verus never resolves `Cargo.toml`
//! at all — see `fixed_offset.rs`'s own doc comment for the established
//! response this mirrors. This proof is conditional on the model being
//! faithful — which `amenable_kani::chrono::week_span_partitions`'s own
//! 100 year-partitioned harnesses, checked directly against chrono's
//! real `checked_first_day`/`checked_last_day`, independently confirm.
//!
//! Unlike that Kani witness, this needs none of its year-partitioning.
//! CBMC's state-space explosion there is a property of symbolic
//! execution over concrete bit-level values across chrono's whole
//! ~262,000-year range (see `week_span_partitions.rs`'s own doc comment
//! for the full investigation). Verus's SMT-based reasoning instead
//! works directly over the linear relationship the claim actually
//! rests on. A date is represented abstractly by its signed day-count,
//! `day_count: int` (position on the calendar's own number line, with
//! no Gregorian or leap-year arithmetic needed at all). `first_day_count`
//! equals `day_count` minus `back`, and `last_day_count` equals
//! `day_count` plus `forward`, where `back` (days from the week's start
//! to this date) and `forward` (days from this date to the week's end)
//! are the two complementary halves of a 7-day week. The sum of `back`
//! and `forward` is always six, by construction: the real reason
//! chrono's own claim holds, independently of which calendar system
//! assigns `back` and `forward` their value for a given weekday and
//! week-start day.
//!
//! Stated for any representable range (`min_day`/`max_day` are
//! parameters, not narrowed to chrono's specific numeric bounds), since
//! the argument holds regardless of where those bounds fall. Bounded
//! only to a hundred million in magnitude, so `i64` arithmetic cannot
//! overflow near the edges; chrono's own day-count range is roughly
//! ninety-five million six hundred thousand in magnitude, comfortably
//! inside that margin, not narrowed by it.

use verus_builtin_macros::verus;
#[allow(
    unused_imports,
    reason = "vstd::prelude::* is unused under plain rustc (verus! {} erases real spec content); needed only when the real verus toolchain parses this file directly"
)]
use vstd::prelude::*;

verus! {

/// The claim: when both ends exist, they span exactly six days. Holds
/// vacuously when either end is absent — the same shape chrono's own
/// claim takes (nothing is asserted about a week whose checked end
/// falls outside the representable range).
pub open spec fn naive_week_span_holds(first: Option<i64>, last: Option<i64>) -> bool {
    match (first, last) {
        (Some(f), Some(l)) => l - f == 6,
        _ => true,
    }
}

/// A model of `(week.checked_first_day(), week.checked_last_day())`:
/// `day_count` stands for the date's position on the calendar's own
/// number line, `back` for how many days separate it from the week's
/// start (`0..=6`, one of the seven possible weekday-to-week-start
/// offsets), and `min_day`/`max_day` for the representable range's own
/// bounds, at any value. `first`/`last` exist exactly when stepping
/// `back` days back, respectively `6 - back` days forward, stays
/// within `min_day..=max_day` — the same shape
/// `checked_first_day`/`checked_last_day`'s own `Option` result takes.
pub fn verify_naive_week_span_model(
    day_count: i64,
    back: i64,
    min_day: i64,
    max_day: i64,
) -> (result: (Option<i64>, Option<i64>))
    requires
        back >= 0 && back <= 6,
        min_day <= max_day,
        day_count >= -100_000_000 && day_count <= 100_000_000,
        min_day >= -100_000_000 && min_day <= 100_000_000,
        max_day >= -100_000_000 && max_day <= 100_000_000,
    ensures
        naive_week_span_holds(result.0, result.1),
{
    let forward = 6 - back;
    let first = if day_count - back >= min_day { Some(day_count - back) } else { None };
    let last = if day_count + forward <= max_day { Some(day_count + forward) } else { None };
    (first, last)
}

} // verus!
