//! `Ordering`'s `KaniWitness` impl and the one harness that checks a
//! non-`SeqCst` memory ordering.

#[cfg(kani)]
use std::sync::atomic::AtomicI32;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

#[cfg(kani)]
use crate::AtomicLoadReflectsTheLastWrite;
use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<std::sync::atomic::Ordering> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_relaxed_ordering_still_makes_a_store_observable".to_owned(),
            VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<std::sync::atomic::Ordering>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Ordering>",
        "kani",
        || <RustStdStandard<std::sync::atomic::Ordering> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_RELAXED_ORDERING_STILL_MAKES_A_STORE_OBSERVABLE_SRC, {
        /// Every proof elsewhere in this module uses `SeqCst`; this one
        /// checks `Ordering::Relaxed` specifically -- a store under
        /// `Relaxed` is still observable via a `Relaxed` load on the
        /// same atomic (the ordering variant only affects cross-thread
        /// synchronization, not single-thread visibility).
        #[kani::proof]
        fn verify_relaxed_ordering_still_makes_a_store_observable() {
            let value: i32 = kani::any();
            let atomic = AtomicI32::new(0);
            atomic.store(value, std::sync::atomic::Ordering::Relaxed);
            assert!(
                AtomicLoadReflectsTheLastWrite::ensures((
                    atomic.load(std::sync::atomic::Ordering::Relaxed),
                    value
                )),
                "a Relaxed store is observable via a Relaxed load on the same atomic"
            );
        }
    }
}
