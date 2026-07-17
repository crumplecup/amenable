//! `KaniWitness` impls for `core::cmp`.

use std::cmp::{Ordering, Reverse};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_ordering_reverse_involution",
            claim: VERIFY_ORDERING_REVERSE_INVOLUTION_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Ordering>",
        verifier: "kani",
        describe: || <RustStdStandard<Ordering> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ORDERING_REVERSE_INVOLUTION_SRC, {
        /// `Ordering` has exactly three inhabitants, and `.reverse()` is an
        /// involution that swaps `Less` and `Greater` while fixing `Equal`.
        /// Kani has no `Arbitrary` impl for `Ordering`, so this enumerates
        /// all three variants concretely rather than sampling symbolically.
        #[kani::proof]
        fn verify_ordering_reverse_involution() {
            for o in [Ordering::Less, Ordering::Equal, Ordering::Greater] {
                assert_eq!(o.reverse().reverse(), o, "reverse is an involution");
            }
            assert_eq!(
                Ordering::Less.reverse(),
                Ordering::Greater,
                "Less reverses to Greater"
            );
            assert_eq!(
                Ordering::Greater.reverse(),
                Ordering::Less,
                "Greater reverses to Less"
            );
            assert_eq!(
                Ordering::Equal.reverse(),
                Ordering::Equal,
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
            harness: "verify_reverse_inverts_comparison",
            claim: VERIFY_REVERSE_INVERTS_COMPARISON_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Reverse<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Reverse<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Reverse<i32>> as KaniWitness>::proof().to_string(),
    }
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
