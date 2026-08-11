//! `KaniWitness` impls for `alloc::boxed`.

use std::boxed::Box;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;
#[cfg(kani)]
use std::cell::Cell;

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_box_derefs_and_writes_through".to_owned(),
            claim: VERIFY_BOX_DEREFS_AND_WRITES_THROUGH_SRC.to_owned(),
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
        /// `DerefMut` is visible on the next read. Checked with `i32`
        /// for the deref/write claim, and separately with a
        /// drop-instrumented, non-`Copy` witness type to confirm that a
        /// replacement through `DerefMut` drops the displaced value
        /// exactly once and that the replacement is dropped when the box
        /// is destroyed. `i32` alone cannot distinguish correct destruction
        /// from a double drop or leak because it has no drop glue.
        #[kani::proof]
        fn verify_box_derefs_and_writes_through() {
            let mut boxed = <Box<i32> as crate::KaniCompose>::kani_any();
            let value = *boxed;
            assert!(
                DerefReflectsTheStoredValue::ensures((*boxed, value)),
                "deref exposes the wrapped value"
            );

            let updated = <i32 as crate::KaniCompose>::kani_any();
            *boxed = updated;
            assert!(
                DerefReflectsTheStoredValue::ensures((*boxed, updated)),
                "a write through deref_mut is visible"
            );

            struct DropWitness {
                drop_count: std::rc::Rc<Cell<u32>>,
            }
            impl Drop for DropWitness {
                fn drop(&mut self) {
                    self.drop_count.set(self.drop_count.get() + 1);
                }
            }

            let drop_count = std::rc::Rc::new(Cell::new(0));
            let mut witness = Box::new(DropWitness {
                drop_count: drop_count.clone(),
            });
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 0)),
                "the value is not dropped while it remains boxed"
            );
            *witness = DropWitness {
                drop_count: drop_count.clone(),
            };
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 1)),
                "assigning through deref_mut drops the displaced value exactly once"
            );
            drop(witness);
            assert!(
                RustStdStandard::<Cell<u32>>::ensures((drop_count.get(), 2)),
                "dropping the box drops the replacement exactly once"
            );
        }
    }
}
