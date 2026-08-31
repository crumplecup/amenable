#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
#[cfg(kani)]
use amenable_core::Requires;
use amenable_std::RustStdStandard;
#[cfg(kani)]
use amenable_std::ValidUnicodeScalar;

use crate::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<*const i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_const_pointer_cast_is_reproducible".to_owned(),
            VERIFY_CONST_POINTER_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<*const i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*const i32>",
        "kani",
        || <RustStdStandard<*const i32> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<*const i32>,
    "amenable_std::rust_std::RustStdStandard<*const i32>",
    (*const i32, *const i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_CONST_POINTER_CAST_IS_REPRODUCIBLE_SRC, {
        /// Casting the same reference to a raw pointer twice gives the
        /// same address, without ever dereferencing the pointer -- a
        /// safe property of the cast itself, deliberately checked without
        /// `unsafe` (this crate forbids it in its own source).
        #[kani::proof]
        fn verify_const_pointer_cast_is_reproducible() {
            let value: i32 = kani::any();
            let first: *const i32 = &value;
            let second: *const i32 = &value;
            assert!(
                RustStdStandard::<*const i32>::ensures((first, second)),
                "casting the same reference twice gives the same address"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<*mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mut_pointer_cast_is_reproducible".to_owned(),
            VERIFY_MUT_POINTER_CAST_IS_REPRODUCIBLE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<*mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<*mut i32>",
        "kani",
        || <RustStdStandard<*mut i32> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<*mut i32>,
    "amenable_std::rust_std::RustStdStandard<*mut i32>",
    (*mut i32, *mut i32),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_MUT_POINTER_CAST_IS_REPRODUCIBLE_SRC, {
        /// Same as the `*const i32` proof, for a mutable raw pointer:
        /// casting the same exclusive reference to a raw pointer twice
        /// gives the same address, without ever dereferencing it.
        #[kani::proof]
        fn verify_mut_pointer_cast_is_reproducible() {
            let mut value: i32 = kani::any();
            let first: *mut i32 = &mut value;
            let second: *mut i32 = &mut value;
            assert!(
                RustStdStandard::<*mut i32>::ensures((first, second)),
                "casting the same reference twice gives the same address"
            );
        }
    }
}
