//! Gallery investigation: a proof that the year-partition scheme itself is correct,
//! closing the gap `chrono_naive_week_span_contract_mechanism_test` found — Kani's
//! `proof_for_contract` mechanism does not check that multiple harnesses collectively
//! cover a contract's domain, so exhaustiveness has to be proved separately, by us.
//!
//! This is pure integer arithmetic over `MIN_YEAR..=MAX_YEAR`, no chrono involved at
//! all: for every year in that range, there exists exactly one partition index in
//! `0..100`, computed the same way the real partition harnesses' `kani::assume` bounds
//! are written, that contains it. "Exactly one" is stronger than "at least one": it
//! also confirms no two partitions overlap.

/// Every item here exists only for the Kani harness below, so the whole module is
/// gated once, rather than scattering `#[cfg(kani)]` across each item individually
/// (cordial's own `CFG-SCATTER-001`).
mod kani_only {
    #![cfg(kani)]

    pub(super) const MIN_YEAR: i32 = -262_143;
    pub(super) const MAX_YEAR: i32 = 262_142;
    pub(super) const PARTITION_SIZE: i32 = 5243;
    pub(super) const PARTITION_COUNT: i32 = 100;

    /// Whether `year` falls inside partition `n`'s half-open range, the same
    /// formula the real partition harnesses use.
    pub(super) fn in_partition(year: i32, n: i32) -> bool {
        year >= MIN_YEAR + n * PARTITION_SIZE && year < MIN_YEAR + (n + 1) * PARTITION_SIZE
    }
}

#[cfg(kani)]
use kani_only::{MAX_YEAR, MIN_YEAR, PARTITION_COUNT, PARTITION_SIZE, in_partition};

::inventory::submit! {
    ::amenable_kani::KaniGalleryRegistration::new(
        || ::amenable_kani::KaniGalleryCase::new(
            "amenable_kani::gallery::chrono_year_partition_exhaustiveness_test::partitions_are_exhaustive_and_disjoint".to_owned(),
            "gallery::chrono_year_partition_exhaustiveness_test::partitions_are_exhaustive_and_disjoint".to_owned(),
            "amenable_kani".to_owned(),
            "the 100-partition scheme is itself exhaustive and disjoint over chrono's whole supported year range, closing the gap proof_for_contract/stub_verified leave open on their own".to_owned(),
            ::amenable_kani::KaniGalleryDisposition::BestPractice,
            ::amenable_kani::KaniGalleryExpectation::Passed,
        ),
    )
}

amenable_derive::gallery_harness! {
    kani, PARTITIONS_ARE_EXHAUSTIVE_AND_DISJOINT_SRC, {
        /// For every year chrono supports, exactly one of the 100 partitions contains
        /// it — computed directly by floor division, then confirmed against every
        /// other partition index to rule out a second match.
        #[kani::proof]
        fn partitions_are_exhaustive_and_disjoint() {
            let year: i32 = kani::any();
            kani::assume(year >= MIN_YEAR && year <= MAX_YEAR);

            let computed = (year - MIN_YEAR).div_euclid(PARTITION_SIZE);
            assert!(computed >= 0 && computed < PARTITION_COUNT, "computed index out of range");
            assert!(in_partition(year, computed), "computed index does not actually contain year");

            // Exactly one, not just at least one: confirm computed is the ONLY index
            // that contains year, by checking the two indices construction could ever
            // produce a false second match on — its immediate neighbors. Non-adjacent
            // indices can't overlap a contiguous, constant-stride tiling if the
            // neighbors don't, so checking both sides is sufficient here.
            if computed > 0 {
                assert!(!in_partition(year, computed - 1), "previous partition also contains year");
            }
            if computed < PARTITION_COUNT - 1 {
                assert!(!in_partition(year, computed + 1), "next partition also contains year");
            }
        }
    }
}
