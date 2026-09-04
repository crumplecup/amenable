//! `KaniWitness` impls for `core::any`.

use std::any::TypeId;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<TypeId> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_type_id_is_reflexive_and_distinguishes_distinct_types".to_owned(),
            VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<TypeId>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<TypeId>",
        "kani",
        || <RustStdStandard<TypeId> as KaniWitness>::proof().to_string(),
    )
}

kani_ensures!(
    RustStdStandard<TypeId>,
    "amenable_std::rust_std::RustStdStandard<TypeId>",
    (TypeId, TypeId),
    |(actual, expected)| actual == expected
);

amenable_derive::harness! {
    kani, VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC, {
        /// `TypeId::of::<T>()` is the same value across calls for the same
        /// `T`, and differs between distinct `T`s.
        #[kani::proof]
        fn verify_type_id_is_reflexive_and_distinguishes_distinct_types() {
            assert!(
                RustStdStandard::<TypeId>::ensures((TypeId::of::<i32>(), TypeId::of::<i32>())),
                "TypeId::of is reflexive for the same type"
            );
            assert_ne!(TypeId::of::<i32>(), TypeId::of::<bool>(), "TypeId::of distinguishes distinct types");
        }
    }
}
