//! Gallery cases for `chrono::NaiveWeek` under Kani.
//!
//! The chrono `NaiveDate` week witness claims, for every valid date and every
//! week-start day, that the week either fits inside `NaiveDate`'s range or reaches
//! out of it. The fitting branch asserts three things about the week: its first day
//! starts on the chosen weekday, the first and last days bracket the date, and the
//! last day minus the first day is six days. Probed piece by piece, with a symbolic
//! year, month, day, and week-start day:
//!
//! - the span relation `(last - first).num_days() == 6`, asserted: passes
//! - day stepping, `checked_add_days` with a step of `0..=6`, and the calls
//!   `checked_first_day()` and `checked_last_day()`: pass, but these probes asserted
//!   nothing about the values, so they show only that the calls do not time out
//! - all three assertions together, in the in-range branch: time out at three minutes
//!
//! The conjunction is therefore the expensive part. Decomposing the claim into its
//! separately verified pieces is a sound strategy, provided each piece is itself
//! asserted over the same symbolic inputs.
//!
//! chrono also documents that `first_day` and `last_day` panic when the week falls
//! outside `NaiveDate`'s range. The witness therefore uses the checked forms. That
//! precondition is a finding in its own right, and it is recorded here so it is not
//! rediscovered.
//!
//! Decomposition, adopted for the witness: five harnesses, each asserted over the same
//! symbolic inputs, whose conjunction is the claim. Four pass within the limit. The span
//! piece is borderline: it passed on one run and timed out on another, so its verdict
//! depends on solver timing near the three-minute limit. A reformulation as
//! `last == first + 6 days` timed out, so the direct `num_days` form is kept.
//!
//! This case is a false trail for the combined in-range assertion as written: it documents
//! the timeout, not a verdict on chrono. The witness is proven only as the decomposition,
//! and the span piece needs a stable result before the row is Complete. The witness stays unproven on Kani until the combined claim
//! resolves, by a model of the arithmetic or by a different decomposition.

#[cfg(kani)]
use chrono::{Datelike, Weekday};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_naive_week_span::in_range_week_claim_times_out".to_owned(),
            "gallery::chrono_naive_week_span::in_range_week_claim_times_out".to_owned(),
            "amenable_kani".to_owned(),
            "The combined in-range NaiveWeek claim times out, though each of its pieces passes alone".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::FalseTrail,
            ::amenable_kani::KaniGalleryExpectation::Timeout,
        ),
    )
}

/// Map any `u8` onto the seven weekdays, so a symbolic `u8` covers every weekday.
#[cfg(kani)]
fn weekday_of(index: u8) -> Weekday {
    match index % 7 {
        0 => Weekday::Mon,
        1 => Weekday::Tue,
        2 => Weekday::Wed,
        3 => Weekday::Thu,
        4 => Weekday::Fri,
        5 => Weekday::Sat,
        _ => Weekday::Sun,
    }
}

amenable_derive::gallery_harness! {
    kani, IN_RANGE_WEEK_CLAIM_TIMES_OUT_SRC, {
        /// The in-range branch of the `NaiveWeek` claim, combined: when both checked
        /// ends exist, the first day starts on the chosen weekday, contains the date,
        /// and the span is six days. Confirmed to time out at three minutes, while each
        /// piece passes alone.
        #[kani::proof]
        fn in_range_week_claim_times_out() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start_index: u8 = kani::any();
            let start = weekday_of(start_index);
            if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                let week = date.week(start);
                if let (Some(first), Some(last)) = (week.checked_first_day(), week.checked_last_day()) {
                    assert!(first.weekday() == start);
                    assert!(first <= date && date <= last);
                    assert!((last - first).num_days() == 6);
                }
            }
        }
    }
}
