//! `KaniWitness` impls and `RangeBounds::contains` harnesses for the six
//! range types (`Range`, `RangeFrom`, `RangeTo`, `RangeToInclusive`,
//! `RangeInclusive`, `RangeFull`).

use std::ops::RangeTo;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<std::ops::Range<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_contains_matches_bounds".to_owned(),
            VERIFY_RANGE_CONTAINS_MATCHES_BOUNDS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::ops::Range<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ops::Range<i32>>",
        "kani",
        || <RustStdStandard<std::ops::Range<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::ops::Range<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::ops::Range<i32>>",
    (bool, bool),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RANGE_CONTAINS_MATCHES_BOUNDS_SRC, {
        /// `Range::contains` and `Range::is_empty` are both defined in
        /// terms of the half-open `start..end` interval; this restates
        /// that definition as a checkable postcondition rather than
        /// trusting the standard library's own implementation matches it.
        #[kani::proof]
        fn verify_range_contains_matches_bounds() {
            let start: i32 = kani::any();
            let end: i32 = kani::any();
            let x: i32 = kani::any();
            let r = start..end;

            assert!(
                RustStdStandard::<std::ops::Range<i32>>::ensures((
                    r.contains(&x),
                    x >= start && x < end
                )),
                "Range::contains matches its half-open bounds"
            );
            assert!(
                RustStdStandard::<std::ops::Range<i32>>::ensures((r.is_empty(), !(start < end))),
                "Range::is_empty holds exactly when start is not less than end"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::ops::RangeFrom<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_from_contains_matches_bound".to_owned(),
            VERIFY_RANGE_FROM_CONTAINS_MATCHES_BOUND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::ops::RangeFrom<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ops::RangeFrom<i32>>",
        "kani",
        || <RustStdStandard<std::ops::RangeFrom<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::ops::RangeFrom<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::ops::RangeFrom<i32>>",
    (bool, bool),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RANGE_FROM_CONTAINS_MATCHES_BOUND_SRC, {
        /// `RangeFrom` is unbounded above, so `contains` reduces to its
        /// single lower bound.
        #[kani::proof]
        fn verify_range_from_contains_matches_bound() {
            let start: i32 = kani::any();
            let x: i32 = kani::any();
            let r = start..;

            assert!(
                RustStdStandard::<std::ops::RangeFrom<i32>>::ensures((r.contains(&x), x >= start)),
                "RangeFrom::contains matches its lower bound"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RangeTo<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_to_contains_matches_bound".to_owned(),
            VERIFY_RANGE_TO_CONTAINS_MATCHES_BOUND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<RangeTo<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RangeTo<i32>>",
        "kani",
        || <RustStdStandard<RangeTo<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<RangeTo<i32>>,
    "amenable_std::rust_std::RustStdStandard<RangeTo<i32>>",
    (bool, bool),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RANGE_TO_CONTAINS_MATCHES_BOUND_SRC, {
        /// `RangeTo` is unbounded below, so `contains` reduces to its
        /// single exclusive upper bound.
        #[kani::proof]
        fn verify_range_to_contains_matches_bound() {
            let end: i32 = kani::any();
            let x: i32 = kani::any();
            let r = ..end;

            assert!(
                RustStdStandard::<RangeTo<i32>>::ensures((r.contains(&x), x < end)),
                "RangeTo::contains matches its exclusive upper bound"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::ops::RangeToInclusive<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_to_inclusive_contains_matches_bound".to_owned(),
            VERIFY_RANGE_TO_INCLUSIVE_CONTAINS_MATCHES_BOUND_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::ops::RangeToInclusive<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ops::RangeToInclusive<i32>>",
        "kani",
        || <RustStdStandard<std::ops::RangeToInclusive<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::ops::RangeToInclusive<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::ops::RangeToInclusive<i32>>",
    (bool, bool),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RANGE_TO_INCLUSIVE_CONTAINS_MATCHES_BOUND_SRC, {
        /// `RangeToInclusive` is unbounded below, so `contains` reduces to
        /// its single inclusive upper bound.
        #[kani::proof]
        fn verify_range_to_inclusive_contains_matches_bound() {
            let end: i32 = kani::any();
            let x: i32 = kani::any();
            let r = ..=end;

            assert!(
                RustStdStandard::<std::ops::RangeToInclusive<i32>>::ensures((
                    r.contains(&x),
                    x <= end
                )),
                "RangeToInclusive::contains matches its inclusive upper bound"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::ops::RangeInclusive<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_inclusive_contains_and_emptiness".to_owned(),
            VERIFY_RANGE_INCLUSIVE_CONTAINS_AND_EMPTINESS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::ops::RangeInclusive<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ops::RangeInclusive<i32>>",
        "kani",
        || <RustStdStandard<std::ops::RangeInclusive<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::ops::RangeInclusive<i32>>,
    "amenable_std::rust_std::RustStdStandard<std::ops::RangeInclusive<i32>>",
    (bool, bool),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_RANGE_INCLUSIVE_CONTAINS_AND_EMPTINESS_SRC, {
        /// `RangeInclusive::contains` matches its closed bounds. Its
        /// `is_empty` also consults an internal `exhausted` flag set by
        /// iteration, so this claim is scoped to a freshly constructed
        /// range (never iterated) — the only state a `start..=end`
        /// literal can be in.
        #[kani::proof]
        fn verify_range_inclusive_contains_and_emptiness() {
            let start: i32 = kani::any();
            let end: i32 = kani::any();
            let x: i32 = kani::any();
            let r = start..=end;

            assert!(
                RustStdStandard::<std::ops::RangeInclusive<i32>>::ensures((
                    r.contains(&x),
                    x >= start && x <= end
                )),
                "RangeInclusive::contains matches its closed bounds"
            );
            assert!(
                RustStdStandard::<std::ops::RangeInclusive<i32>>::ensures((
                    r.is_empty(),
                    !(start <= end)
                )),
                "a freshly constructed RangeInclusive is empty iff start > end"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<std::ops::RangeFull> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_range_full_contains_everything".to_owned(),
            VERIFY_RANGE_FULL_CONTAINS_EVERYTHING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::ops::RangeFull>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<RangeFull>",
        "kani",
        || <RustStdStandard<std::ops::RangeFull> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<std::ops::RangeFull>,
    "amenable_std::rust_std::RustStdStandard<std::ops::RangeFull>",
    bool,
    |contains| contains
);

amenable_derive::harness! {
    kani, VERIFY_RANGE_FULL_CONTAINS_EVERYTHING_SRC, {
        /// `RangeFull` carries no fields but still has real behavior:
        /// its `contains` is unconditionally `true`.
        #[kani::proof]
        fn verify_range_full_contains_everything() {
            let x: i32 = kani::any();
            assert!(
                RustStdStandard::<std::ops::RangeFull>::ensures(std::ops::RangeBounds::contains(
                    &(..),
                    &x
                )),
                "RangeFull contains every value"
            );
        }
    }
}
