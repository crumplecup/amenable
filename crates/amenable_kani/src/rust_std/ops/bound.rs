//! `Bound<i32>`'s `KaniWitness` impl and endpoint round-trip harness, plus
//! the `BoundHasNoEndpoint` raw-boolean claim its `Unbounded` check reuses.

use std::ops::Bound;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<Bound<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_bound_round_trips_its_endpoint".to_owned(),
            VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Bound<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
        "kani",
        || <RustStdStandard<Bound<i32>> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<Bound<i32>>,
    "amenable_std::rust_std::RustStdStandard<Bound<i32>>",
    (i32, i32),
    |(actual, expected)| actual == expected
);

/// A `bool` known to be the `true` a `matches!(bound, Bound::Unbounded)`
/// check reports when the bound really is `Unbounded`, carrying no
/// endpoint -- following `EmptiedContainerReportsEmpty`'s established
/// shape for a raw boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct BoundHasNoEndpoint;

impl KaniWitness for BoundHasNoEndpoint {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_bound_round_trips_its_endpoint".to_owned(),
            VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(BoundHasNoEndpoint);

kani_ensures!(
    BoundHasNoEndpoint,
    "amenable_kani::BoundHasNoEndpoint",
    bool,
    |is_unbounded| is_unbounded
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::BoundHasNoEndpoint",
        "kani",
        || <BoundHasNoEndpoint as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_BOUND_ROUND_TRIPS_ITS_ENDPOINT_SRC, {
        /// `Bound` has exactly three inhabitants, two of which carry an
        /// endpoint. Kani has no `Arbitrary` impl for `Bound`, so each
        /// variant is constructed explicitly rather than sampled
        /// symbolically, matching the `Ordering` harness's approach. The
        /// assertions call `RustStdStandard::<Bound<i32>>::ensures`
        /// directly rather than restating the comparison.
        #[kani::proof]
        fn verify_bound_round_trips_its_endpoint() {
            let v: i32 = kani::any();

            match Bound::Included(v) {
                Bound::Included(inner) => {
                    assert!(
                        RustStdStandard::<Bound<i32>>::ensures((inner, v)),
                        "Included round-trips its endpoint"
                    )
                }
                _ => unreachable!("Bound::Included never matches another variant"),
            }

            match Bound::Excluded(v) {
                Bound::Excluded(inner) => {
                    assert!(
                        RustStdStandard::<Bound<i32>>::ensures((inner, v)),
                        "Excluded round-trips its endpoint"
                    )
                }
                _ => unreachable!("Bound::Excluded never matches another variant"),
            }

            let unbounded: Bound<i32> = Bound::Unbounded;
            assert!(
                BoundHasNoEndpoint::ensures(matches!(unbounded, Bound::Unbounded)),
                "Unbounded carries no endpoint"
            );
        }
    }
}
