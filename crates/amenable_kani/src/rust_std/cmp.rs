//! `KaniWitness` impls for `core::cmp`.

use std::cmp::Reverse;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<std::cmp::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ordering_reverse_involution".to_owned(),
            claim: VERIFY_ORDERING_REVERSE_INVOLUTION_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<std::cmp::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::cmp::Ordering>",
        "kani",
        || <RustStdStandard<std::cmp::Ordering> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_ORDERING_REVERSE_INVOLUTION_SRC, {
        /// `Ordering` has exactly three inhabitants, and `.reverse()` is an
        /// involution that swaps `Less` and `Greater` while fixing `Equal`.
        /// Kani has no `Arbitrary` impl for `Ordering`, so this enumerates
        /// all three variants concretely rather than sampling symbolically.
        #[kani::proof]
        fn verify_ordering_reverse_involution() {
            for o in [
                std::cmp::Ordering::Less,
                std::cmp::Ordering::Equal,
                std::cmp::Ordering::Greater,
            ] {
                assert_eq!(o.reverse().reverse(), o, "reverse is an involution");
            }
            assert_eq!(
                std::cmp::Ordering::Less.reverse(),
                std::cmp::Ordering::Greater,
                "Less reverses to Greater"
            );
            assert_eq!(
                std::cmp::Ordering::Greater.reverse(),
                std::cmp::Ordering::Less,
                "Greater reverses to Less"
            );
            assert_eq!(
                std::cmp::Ordering::Equal.reverse(),
                std::cmp::Ordering::Equal,
                "Equal reverses to itself"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Reverse<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_reverse_inverts_comparison".to_owned(),
            claim: VERIFY_REVERSE_INVERTS_COMPARISON_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Reverse<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Reverse<i32>>",
        "kani",
        || <RustStdStandard<Reverse<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_REVERSE_INVERTS_COMPARISON_SRC, {
        /// `Reverse<T>` inverts `T`'s comparison direction, and its `.0`
        /// field round-trips the wrapped value unchanged.
        #[kani::proof]
        fn verify_reverse_inverts_comparison() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            assert_eq!(
                Reverse(a).cmp(&Reverse(b)),
                b.cmp(&a),
                "Reverse inverts comparison direction"
            );
            assert_eq!(Reverse(a).0, a, "Reverse round-trips its wrapped value");
        }
    }
}
