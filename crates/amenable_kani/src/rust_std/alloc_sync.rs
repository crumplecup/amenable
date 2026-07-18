//! `KaniWitness` impls for `alloc::sync`.

use std::sync::{Arc, Weak as ArcWeak};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Arc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_arc_strong_count_tracks_clones",
            claim: VERIFY_ARC_STRONG_COUNT_TRACKS_CLONES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Arc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Arc<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Arc<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ARC_STRONG_COUNT_TRACKS_CLONES_SRC, {
        /// Same reference-counting contract as `Rc`, for the
        /// thread-safe carrier: `Arc` derefs to its wrapped value, and
        /// `strong_count` increments on `clone` and decrements again
        /// once the clone is dropped.
        #[kani::proof]
        fn verify_arc_strong_count_tracks_clones() {
            let value: i32 = kani::any();
            let arc = Arc::new(value);
            assert_eq!(*arc, value, "deref exposes the wrapped value");
            assert_eq!(Arc::strong_count(&arc), 1, "a fresh Arc has strong_count 1");

            let arc2 = Arc::clone(&arc);
            assert_eq!(Arc::strong_count(&arc), 2, "clone increments strong_count");
            drop(arc2);
            assert_eq!(
                Arc::strong_count(&arc),
                1,
                "dropping the clone decrements strong_count back"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<ArcWeak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_arc_weak_upgrade_fails_once_the_strong_count_hits_zero",
            claim: VERIFY_ARC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<ArcWeak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<ArcWeak<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<ArcWeak<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_ARC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC, {
        /// Same upgrade contract as `RcWeak`: succeeds while a strong
        /// reference is alive, fails once the last one is dropped.
        #[kani::proof]
        fn verify_arc_weak_upgrade_fails_once_the_strong_count_hits_zero() {
            let value: i32 = kani::any();
            let arc = Arc::new(value);
            let weak = Arc::downgrade(&arc);
            assert_eq!(Arc::weak_count(&arc), 1, "downgrade increments weak_count");
            assert!(
                weak.upgrade().is_some(),
                "upgrade succeeds while a strong reference is alive"
            );

            drop(arc);
            assert!(
                weak.upgrade().is_none(),
                "upgrade fails once all strong references are dropped"
            );
        }
    }
}
