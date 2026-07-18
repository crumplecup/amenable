//! `KaniWitness` impls for `alloc::boxed`.

use std::boxed::Box;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_box_derefs_and_writes_through",
            claim: VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Box<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Box<i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC, {
        /// `Box::new` derefs to the value it wraps, and a write through
        /// `DerefMut` is visible on the next read.
        #[kani::proof]
        fn verify_box_derefs_and_writes_through() {
            let value: i32 = kani::any();
            let mut boxed = Box::new(value);
            assert_eq!(*boxed, value, "deref exposes the wrapped value");

            let updated: i32 = kani::any();
            *boxed = updated;
            assert_eq!(*boxed, updated, "a write through deref_mut is visible");
        }
    }
}
