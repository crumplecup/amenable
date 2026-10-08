//! `KaniWitness` for `amenable_ext::ExtStandard<chrono::NaiveWeek>`.
//!
//! chrono gives `NaiveWeek` no public constructor: every value comes from
//! `NaiveDate::week`. So the claim is about the values a real date and a chosen
//! week-start day produce, over every `i32` year, `u32` month and day, and all seven
//! week-start days.
//!
//! The claim, stated once: for every valid date and every week-start day, either both
//! checked ends of the week exist and satisfy the week's specification, or the week
//! reaches out of `NaiveDate`'s range. The checked forms are used because chrono
//! documents `first_day` and `last_day` as panicking outside that range.
//!
//! The claim is decomposed into pieces, each asserted over the same symbolic inputs.
//! Four live here, each a single full-domain harness:
//!
//! 1. `first_day_starts_on_the_chosen_weekday`: the first day's weekday is the start.
//! 2. `first_and_last_day_bracket_the_date`: the first day is on or before the date,
//!    and the last day is on or after it.
//! 3. `first_day_exists_exactly_when_the_date_can_step_back`: `checked_first_day` is
//!    `Some` exactly when the date can step back to the first day inside the calendar.
//! 4. `last_day_exists_exactly_when_the_date_can_step_forward`: `checked_last_day` is
//!    `Some` exactly when the date can step forward to the last day inside the calendar.
//!
//! The fifth piece, the span (the last day minus the first day is six days), does not
//! live here: as a single full-domain harness it times out at three minutes, even
//! though every other piece is cheap. It lives in the sibling module
//! `week_span_partitions`, proved by a year-partitioned sweep instead of one harness —
//! see that module's own doc comment for the full account, and the gallery cases
//! `chrono_naive_week_span`, `chrono_day_count_staging`,
//! `chrono_naive_week_stub_composition`, and `chrono_year_span_growth` (and its
//! siblings) for the investigation that found the partition size.

use amenable_core::Evidence;
use amenable_ext::ExtStandard;
use chrono::NaiveWeek;

use crate::rust_std::bridge_kani_witness;

/// Every item here exists only for the Kani harnesses below, so the whole module is
/// gated once, rather than scattering `#[cfg(kani)]` across each item individually
/// (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    use chrono::{Datelike, Days, Weekday};

    /// Map any `u8` onto the seven weekdays, so a symbolic `u8` covers every
    /// weekday. Used only inside the harnesses below, and re-exported one level
    /// up for `week_span_partitions`, a sibling module under `chrono`.
    pub(in crate::chrono) fn weekday_of(index: u8) -> Weekday {
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

    /// Days a date steps back from its own weekday to reach `start` (`0..=6`). Used
    /// only inside the harnesses below.
    pub(super) fn days_back_to(date: chrono::NaiveDate, start: Weekday) -> u32 {
        (date.weekday().num_days_from_monday() + 7 - start.num_days_from_monday()) % 7
    }

    /// Piece 1's own bound, named so each harness's `assert!` points at a real,
    /// registered contract fragment instead of a raw equation.
    pub(super) fn first_day_starts_on_the_chosen_weekday_holds(
        first: chrono::NaiveDate,
        start: Weekday,
    ) -> bool {
        first.weekday() == start
    }

    /// Piece 2's own bound, named for the same reason.
    pub(super) fn first_and_last_day_bracket_the_date_holds(
        first: chrono::NaiveDate,
        date: chrono::NaiveDate,
        last: chrono::NaiveDate,
    ) -> bool {
        first <= date && date <= last
    }

    /// Piece 3's own bound, named for the same reason.
    pub(super) fn first_day_exists_exactly_when_the_date_can_step_back_holds(
        checked_some: bool,
        fits: bool,
    ) -> bool {
        checked_some == fits
    }

    /// Piece 4's own bound, named for the same reason.
    pub(super) fn last_day_exists_exactly_when_the_date_can_step_forward_holds(
        checked_some: bool,
        fits: bool,
    ) -> bool {
        checked_some == fits
    }
}

// `Days` is used directly by the harness bodies below (`checked_sub_days`/
// `checked_add_days`), not by anything inside `kani_only`.
#[cfg(kani)]
use chrono::Days;
// Re-exported: `week_span_partitions` (a sibling module under `chrono`) reuses this
// same weekday mapping rather than duplicating it.
#[cfg(kani)]
pub(super) use kani_only::weekday_of;
#[cfg(kani)]
use kani_only::{
    days_back_to, first_and_last_day_bracket_the_date_holds,
    first_day_exists_exactly_when_the_date_can_step_back_holds,
    first_day_starts_on_the_chosen_weekday_holds,
    last_day_exists_exactly_when_the_date_can_step_forward_holds,
};

impl crate::KaniWitness for ExtStandard<NaiveWeek> {
    type SupportingEvidence = Self;
    type ProofArtifact = crate::ext_macros::ExtCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        crate::ext_macros::ExtCheckedProof::new(
            "first_day_starts_on_the_chosen_weekday, first_and_last_day_bracket_the_date, \
             first_day_exists_exactly_when_the_date_can_step_back, \
             last_day_exists_exactly_when_the_date_can_step_forward, plus the fifth piece \
             (the span) in the sibling module week_span_partitions: week_span_holds's claim, \
             asserted by 100 year-partitioned plain proof harnesses \
             (span_partition_0..span_partition_99), with partitions_are_exhaustive_and_disjoint \
             confirming the 100 partitions cover every year chrono supports with no gaps or \
             overlaps"
                .to_owned(),
            format!(
                "{FIRST_DAY_WEEKDAY_SRC}\n{BRACKET_SRC}\n{FIRST_LINK_SRC}\n{LAST_LINK_SRC}\n\n\
                 -- fifth piece, in week_span_partitions --\n{}",
                super::week_span_partitions::proof_summary(),
            ),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(ExtStandard<NaiveWeek>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_ext::ExtStandard<chrono::NaiveWeek>",
        "kani",
        || <ExtStandard<NaiveWeek> as crate::KaniWitness>::proof().to_string(),
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::civil_naive_week::first_day_starts_on_the_chosen_weekday_holds",
        "kani",
        "ensures",
        || "first.weekday() == start",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::civil_naive_week::first_and_last_day_bracket_the_date_holds",
        "kani",
        "ensures",
        || "first <= date && date <= last",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::civil_naive_week::first_day_exists_exactly_when_the_date_can_step_back_holds",
        "kani",
        "ensures",
        || "date.week(start).checked_first_day().is_some() == fits",
    )
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_kani::chrono::civil_naive_week::last_day_exists_exactly_when_the_date_can_step_forward_holds",
        "kani",
        "ensures",
        || "date.week(start).checked_last_day().is_some() == fits",
    )
}

amenable_derive::harness! {
    kani, FIRST_DAY_WEEKDAY_SRC, {
        /// Piece 1: when both checked ends exist, the first day's weekday is the start.
        #[kani::proof]
        fn first_day_starts_on_the_chosen_weekday() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                let week = date.week(start);
                if let (Some(first), Some(_)) = (week.checked_first_day(), week.checked_last_day()) {
                    assert!(first_day_starts_on_the_chosen_weekday_holds(first, start));
                }
            }
        }
    }
}

amenable_derive::harness! {
    kani, BRACKET_SRC, {
        /// Piece 2: when both checked ends exist, the first day is on or before the
        /// date, and the last day is on or after it.
        #[kani::proof]
        fn first_and_last_day_bracket_the_date() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                let week = date.week(start);
                if let (Some(first), Some(last)) = (week.checked_first_day(), week.checked_last_day()) {
                    assert!(first_and_last_day_bracket_the_date_holds(first, date, last));
                }
            }
        }
    }
}

amenable_derive::harness! {
    kani, FIRST_LINK_SRC, {
        /// Piece 4: `checked_first_day` exists exactly when the date can step back to
        /// the first day inside the calendar.
        #[kani::proof]
        fn first_day_exists_exactly_when_the_date_can_step_back() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                let fits = date
                    .checked_sub_days(Days::new(u64::from(days_back_to(date, start))))
                    .is_some();
                assert!(first_day_exists_exactly_when_the_date_can_step_back_holds(
                    date.week(start).checked_first_day().is_some(),
                    fits
                ));
            }
        }
    }
}

amenable_derive::harness! {
    kani, LAST_LINK_SRC, {
        /// Piece 5: `checked_last_day` exists exactly when the date can step forward to
        /// the last day inside the calendar.
        #[kani::proof]
        fn last_day_exists_exactly_when_the_date_can_step_forward() {
            let year: i32 = kani::any();
            let month: u32 = kani::any();
            let day: u32 = kani::any();
            let start = weekday_of(kani::any());
            if let Some(date) = chrono::NaiveDate::from_ymd_opt(year, month, day) {
                let fits = date
                    .checked_add_days(Days::new(u64::from(6 - days_back_to(date, start))))
                    .is_some();
                assert!(last_day_exists_exactly_when_the_date_can_step_forward_holds(
                    date.week(start).checked_last_day().is_some(),
                    fits
                ));
            }
        }
    }
}
