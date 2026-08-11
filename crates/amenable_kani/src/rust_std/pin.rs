//! `KaniWitness` impls for `core::pin`.

use std::pin::Pin;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Pin<Box<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_pin_derefs_and_get_mut_round_trip".to_owned(),
            claim: VERIFY_PIN_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Pin<Box<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Pin<Box<i32>>>",
        verifier: "kani",
        describe: || <RustStdStandard<Pin<Box<i32>>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_PIN_DEREFS_AND_GET_MUT_ROUND_TRIP_SRC, {
        /// `Pin` is transparent to its pointee through `Deref`, and (for
        /// an `Unpin` target like `i32`, which needs no pinning safety
        /// contract) `as_mut().get_mut()` grants a safe `&mut` back to
        /// it — a write through that reference is visible through deref.
        #[kani::proof]
        fn verify_pin_derefs_and_get_mut_round_trip() {
            let value: i32 = kani::any();
            let mut pinned: Pin<Box<i32>> = Box::pin(value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*pinned, value)),
                "deref exposes the pointee"
            );

            let updated: i32 = kani::any();
            *pinned.as_mut().get_mut() = updated;
            assert!(
                DerefReflectsTheStoredValue::ensures((*pinned, updated)),
                "a write through get_mut is visible through deref"
            );
        }
    }
}
