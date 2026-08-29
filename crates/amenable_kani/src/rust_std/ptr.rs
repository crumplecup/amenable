//! `KaniWitness` impls for `core::ptr`.

use std::ptr::NonNull;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
use crate::KaniWitness;
#[cfg(kani)]
use crate::OptionIsNoneReportsTrue;
#[cfg(kani)]
use crate::OptionIsSomeReportsTrue;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<NonNull<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_non_null_rejects_the_null_pointer".to_owned(),
            VERIFY_NON_NULL_REJECTS_THE_NULL_POINTER_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<NonNull<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NonNull<i32>>",
        "kani",
        || <RustStdStandard<NonNull<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_NON_NULL_REJECTS_THE_NULL_POINTER_SRC, {
        /// `NonNull`'s entire reason to exist is the null check:
        /// `new` accepts a valid pointer and rejects the null pointer,
        /// and `from` preserves the address of the reference it was
        /// built from. Every operation here is address-level (no
        /// dereference), so this needs no `unsafe` — this crate forbids
        /// it (`#![forbid(unsafe_code)]` in `lib.rs`).
        #[kani::proof]
        fn verify_non_null_rejects_the_null_pointer() {
            let mut value: i32 = kani::any();
            let ptr: *mut i32 = &mut value;
            assert!(
                OptionIsSomeReportsTrue::ensures(NonNull::new(ptr).is_some()),
                "a valid pointer is accepted"
            );
            assert!(
                OptionIsNoneReportsTrue::ensures(NonNull::new(std::ptr::null_mut::<i32>()).is_none()),
                "the null pointer is rejected"
            );

            let nn = NonNull::from(&mut value);
            assert!(
                CollectedSequenceMatchesExpected::ensures((nn.as_ptr(), ptr)),
                "NonNull::from preserves the address"
            );
        }
    }
}
