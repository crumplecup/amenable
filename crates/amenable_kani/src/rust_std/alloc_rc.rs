//! `KaniWitness` impls for `alloc::rc`.

use std::rc::{Rc, Weak as RcWeak};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Rc<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rc_strong_count_tracks_clones",
            claim: VERIFY_RC_STRONG_COUNT_TRACKS_CLONES_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Rc<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Rc<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Rc<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RC_STRONG_COUNT_TRACKS_CLONES_SRC, {
        /// `Rc` derefs to its wrapped value, and `strong_count`
        /// increments on `clone` and decrements again once the clone
        /// is dropped.
        #[kani::proof]
        fn verify_rc_strong_count_tracks_clones() {
            let value: i32 = kani::any();
            let rc = Rc::new(value);
            assert_eq!(*rc, value, "deref exposes the wrapped value");
            assert_eq!(Rc::strong_count(&rc), 1, "a fresh Rc has strong_count 1");

            let rc2 = Rc::clone(&rc);
            assert_eq!(Rc::strong_count(&rc), 2, "clone increments strong_count");
            drop(rc2);
            assert_eq!(
                Rc::strong_count(&rc),
                1,
                "dropping the clone decrements strong_count back"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<RcWeak<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_rc_weak_upgrade_fails_once_the_strong_count_hits_zero",
            claim: VERIFY_RC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<RcWeak<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<RcWeak<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<RcWeak<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_RC_WEAK_UPGRADE_FAILS_ONCE_THE_STRONG_COUNT_HITS_ZERO_SRC, {
        /// A `Weak` upgrades successfully while a strong reference is
        /// alive, and fails once the last one is dropped — the
        /// defining behavior that distinguishes it from a strong `Rc`.
        #[kani::proof]
        fn verify_rc_weak_upgrade_fails_once_the_strong_count_hits_zero() {
            let value: i32 = kani::any();
            let rc = Rc::new(value);
            let weak = Rc::downgrade(&rc);
            assert_eq!(Rc::weak_count(&rc), 1, "downgrade increments weak_count");
            assert!(
                weak.upgrade().is_some(),
                "upgrade succeeds while a strong reference is alive"
            );

            drop(rc);
            assert!(
                weak.upgrade().is_none(),
                "upgrade fails once all strong references are dropped"
            );
        }
    }
}
