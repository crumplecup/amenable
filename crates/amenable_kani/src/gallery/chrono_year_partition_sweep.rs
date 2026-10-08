//! Gallery investigation: a full sweep of chrono's supported year range
//! (`-262143..=262142`, 524,286 distinct years), split into 100 partitions, with month
//! and day left fully symbolic in every partition.
//!
//! `chrono_year_span_with_free_month_day_growth` measured the crossing point against
//! our usual three-minute bound at a span between 8192 and 16384 years. 524,286 / 100 =
//! 5242.86, so each partition here covers roughly 5243 years — comfortably inside the
//! measured safe range (the 4096-year span measured 75.44s, the 8192-year span
//! 129.97s), with margin.
//!
//! Generated with `seq_macro::seq!`, not hand-written: Kani harnesses are one-to-one
//! with functions, so 100 partitions means 100 real functions, but the body is
//! identical in shape across all of them, differing only in which slice of the year
//! range each one assumes. Writing that by hand 100 times is exactly the kind of
//! mechanical repetition this project's own convention says to generate instead.
//!
//! Each partition's claim is the same full day-count equality the other `chrono_*`
//! investigations use (`forward_day_count` against `civil_days_spec`, reused from
//! `chrono_day_count_staging`, not duplicated).
//!
//! Not yet run as a full sweep: 100 Kani proofs at roughly 80-95s each is on the order of
//! 2-3 hours of serial wall time. Three partitions sanity-checked individually before any
//! full-sweep decision:
//!
//! | Partition | Years | Verification time |
//! | --- | --- | --- |
//! | 0 (first) | -262143..-256900 | 95.50s |
//! | 50 (middle) | 7..5250 (crosses year 0) | 81.64s |
//! | 99 (last, overshoots `MAX_YEAR` by 14) | 256914..262157 | 78.37s |
//!
//! All three land in the same tight range, well under the 180s bound, with no sign of
//! the per-partition cost depending on which part of the range it covers.

/// Every item here exists only for the Kani harnesses below, so the whole module is
/// gated once, rather than scattering `#[cfg(kani)]` across each item individually
/// (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    pub(super) use super::super::chrono_day_count_staging::{
        CE_OFFSET_FROM_EPOCH, civil_days_spec, forward_day_count,
    };

    /// chrono's real minimum supported year (`NaiveDate::MIN.year()`).
    pub(super) const MIN_YEAR: i32 = -262_143;

    /// Years per partition: `ceil(524_286 / 100)`. The last partition's upper bound
    /// overshoots chrono's real maximum year by 14 years; those extra years simply
    /// produce `None` from `forward_day_count`, which the `if let Some` below already
    /// handles.
    pub(super) const PARTITION_SIZE: i32 = 5243;
}

#[cfg(kani)]
use kani_only::{
    CE_OFFSET_FROM_EPOCH, MIN_YEAR, PARTITION_SIZE, civil_days_spec, forward_day_count,
};

seq_macro::seq!(N in 0..100 {
    ::inventory::submit! {
        ::amenable_kani::KaniGalleryRegistration::new(
            || ::amenable_kani::KaniGalleryCase::new(
                concat!(
                    "amenable_kani::gallery::chrono_year_partition_sweep::year_partition_",
                    N
                ).to_owned(),
                concat!(
                    "gallery::chrono_year_partition_sweep::year_partition_",
                    N
                ).to_owned(),
                "amenable_kani".to_owned(),
                concat!(
                    "year partition ", N,
                    " of 100 over chrono's full supported range, month and day fully symbolic"
                ).to_owned(),
                ::amenable_kani::KaniGalleryDisposition::Hypothesis,
                ::amenable_kani::KaniGalleryExpectation::Passed,
            ),
        )
    }

    amenable_derive::gallery_harness! {
        kani, PARTITION_SRC_~N, {
            /// One partition of the full-year-range sweep: month and day fully
            /// symbolic, year restricted to this partition's slice.
            #[kani::proof]
            fn year_partition_~N() {
                let year: i32 = kani::any();
                let month: u32 = kani::any();
                let day: u32 = kani::any();
                kani::assume(
                    year >= MIN_YEAR + N * PARTITION_SIZE
                        && year < MIN_YEAR + (N + 1) * PARTITION_SIZE
                );
                if let Some(count) = forward_day_count(year, month, day) {
                    assert!(
                        i64::from(count)
                            == civil_days_spec(i64::from(year), i64::from(month), i64::from(day))
                                + CE_OFFSET_FROM_EPOCH
                    );
                }
            }
        }
    }
});
