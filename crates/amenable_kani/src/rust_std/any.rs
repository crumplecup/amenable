//! `KaniWitness` impls for `core::any`.

use std::any::TypeId;

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<TypeId> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_type_id_is_reflexive_and_distinguishes_distinct_types".to_owned(),
            claim: VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
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

amenable_derive::harness! {
    kani, VERIFY_TYPE_ID_IS_REFLEXIVE_AND_DISTINGUISHES_DISTINCT_TYPES_SRC, {
        /// `TypeId::of::<T>()` is the same value across calls for the same
        /// `T`, and differs between distinct `T`s.
        #[kani::proof]
        fn verify_type_id_is_reflexive_and_distinguishes_distinct_types() {
            assert_eq!(TypeId::of::<i32>(), TypeId::of::<i32>(), "TypeId::of is reflexive for the same type");
            assert_ne!(TypeId::of::<i32>(), TypeId::of::<bool>(), "TypeId::of distinguishes distinct types");
        }
    }
}
