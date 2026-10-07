//! The fifth piece of `ExtStandard<chrono::NaiveWeek>`'s witness: the span claim, that
//! when both checked ends of a week exist, they span exactly six days.
//!
//! As a single full-domain harness, this claim times out at three minutes, even though
//! every other piece of the `NaiveWeek` witness is cheap (see `civil_naive_week`'s own
//! doc comment). The investigation that found why, and the partition size that resolves
//! it, is recorded across several gallery cases: `chrono_naive_week_span`,
//! `chrono_day_count_staging`, `chrono_year_span_growth` and its month/day siblings,
//! `chrono_year_span_with_free_month_day_growth`, and
//! `chrono_naive_week_span_partition_test`. The short version: the cost is in the
//! unbounded year range specifically, not in month, day, or their interaction with each
//! other; bounding year to spans of roughly 5243 years collapses it.
//!
//! The proof here has two parts:
//!
//! 1. 100 generated `#[kani::proof]` harnesses (`span_partition_0` through
//!    `span_partition_99`, via `seq_macro::seq!`, since Kani harnesses are one-to-one
//!    with functions and writing 100 by hand is exactly the mechanical repetition this
//!    project generates instead of hand-rolling), each asserting the claim over its own
//!    year slice.
//! 2. `partitions_are_exhaustive_and_disjoint`: a proof that the 100 slices cover every
//!    year chrono supports, with no gaps and no overlaps, so together the 100 plain
//!    proofs really do cover the whole domain.
//!
//! `week_span_holds` is a plain function, not a Kani contract. `chrono_naive_week_span_
//! contract_mechanism_test` (gallery) found that Kani's own contract-checking machinery
//! does not check that multiple `proof_for_contract` harnesses collectively cover a
//! contract's domain — it trusts a declared `#[kani::ensures]` once any one harness
//! exists for it, so exhaustiveness would have had to be proved separately regardless.
//! Separately, and decisively here: wrapping this specific claim in
//! `#[kani::requires]`/`#[kani::ensures]` and discharging it via `#[kani::proof_for_
//! contract]` made it time out, even though the exact same logic in a plain
//! `#[kani::proof]` (confirmed directly, calling the same separate function, with the
//! contract attributes present but unused) verifies in well under a minute. The
//! contract *verification mode* itself, not the attributes or the separate function, is
//! the cost — unlike the day-count claim in `chrono_day_count_staging`, where contract
//! vs. plain assertion made no measurable difference. Since nothing else needs to
//! `stub_verified` this claim, the contract bought nothing here and cost real time, so
//! this file uses plain proofs throughout.

#[cfg(kani)]
use chrono::{Datelike, NaiveDate, Weekday};

#[cfg(kani)]
use super::civil_naive_week::weekday_of;

/// chrono's real minimum supported year.
#[cfg(kani)]
const MIN_YEAR: i32 = -262_143;

/// chrono's real maximum supported year.
#[cfg(kani)]
const MAX_YEAR: i32 = 262_142;

/// Years per partition: `ceil(524_286 / 100)`. The last partition's upper bound
/// overshoots `MAX_YEAR` by 14 years; those extra years produce `None` from
/// `NaiveDate::from_ymd_opt`, which every partition harness already handles.
#[cfg(kani)]
const PARTITION_SIZE: i32 = 5243;

/// Number of partitions.
#[cfg(kani)]
const PARTITION_COUNT: i32 = 100;

amenable_derive::gallery_harness! {
    kani, WEEK_SPAN_HOLDS_SRC, {
        /// The claim: when both checked ends of the week exist, they span exactly six
        /// days. A plain function, not a Kani contract — see the module doc comment
        /// for why. Asserted by the 100 partitioned harnesses below, never proven
        /// directly as one full-domain harness.
        fn week_span_holds(date: NaiveDate, start: Weekday) -> bool {
            match (
                date.week(start).checked_first_day(),
                date.week(start).checked_last_day(),
            ) {
                (Some(first), Some(last)) => (last - first).num_days() == 6,
                _ => true,
            }
        }
    }
}

seq_macro::seq!(N in 0..100 {
    amenable_derive::harness! {
        kani, PARTITION_SRC_~N, {
            /// Year partition N of 100: `week_span_holds`'s claim, asserted for this
            /// slice of the year range, month, day, and the week-start day fully
            /// symbolic.
            #[kani::proof]
            fn span_partition_~N() {
                let year: i32 = kani::any();
                let month: u32 = kani::any();
                let day: u32 = kani::any();
                let start = weekday_of(kani::any());
                kani::assume(
                    year >= MIN_YEAR + N * PARTITION_SIZE
                        && year < MIN_YEAR + (N + 1) * PARTITION_SIZE
                );
                if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
                    assert!(week_span_holds(date, start));
                }
            }
        }
    }
});

/// Whether `year` falls inside partition `n`'s half-open range, the same formula the
/// 100 partition harnesses' `kani::assume` bounds use.
#[cfg(kani)]
fn in_partition(year: i32, n: i32) -> bool {
    year >= MIN_YEAR + n * PARTITION_SIZE && year < MIN_YEAR + (n + 1) * PARTITION_SIZE
}

amenable_derive::harness! {
    kani, PARTITIONS_ARE_EXHAUSTIVE_AND_DISJOINT_SRC, {
        /// For every year chrono supports, exactly one of the 100 partitions above
        /// contains it — computed directly by floor division, then confirmed against
        /// both neighboring partitions to rule out a second match. Without this, the
        /// 100 partition proofs above would not establish that the whole domain is
        /// covered; nothing else checks that for us.
        #[kani::proof]
        fn partitions_are_exhaustive_and_disjoint() {
            let year: i32 = kani::any();
            kani::assume(year >= MIN_YEAR && year <= MAX_YEAR);

            let computed = (year - MIN_YEAR).div_euclid(PARTITION_SIZE);
            assert!(computed >= 0 && computed < PARTITION_COUNT, "computed index out of range");
            assert!(in_partition(year, computed), "computed index does not actually contain year");

            if computed > 0 {
                assert!(!in_partition(year, computed - 1), "previous partition also contains year");
            }
            if computed < PARTITION_COUNT - 1 {
                assert!(!in_partition(year, computed + 1), "next partition also contains year");
            }
        }
    }
}

/// Summary of the fifth piece's proof, for `civil_naive_week`'s witness audit text. The
/// 100 partition harnesses' own source isn't inlined: they are mechanically identical
/// in shape, differing only in which slice of the year range each one assumes, so the
/// claim and the exhaustiveness proof are the content that actually matters for audit.
pub(super) fn proof_summary() -> String {
    format!(
        "{WEEK_SPAN_HOLDS_SRC}\n\n-- partitions_are_exhaustive_and_disjoint --\n{PARTITIONS_ARE_EXHAUSTIVE_AND_DISJOINT_SRC}"
    )
}
