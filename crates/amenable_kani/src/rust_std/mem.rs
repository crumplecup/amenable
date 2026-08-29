//! `KaniWitness` impls for `core::mem`.

use std::mem::{Discriminant, ManuallyDrop};

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
#[cfg(kani)]
use crate::CollectedSequenceMatchesExpected;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<ManuallyDrop<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_manually_drop_derefs_and_into_inner_round_trip".to_owned(),
            VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<ManuallyDrop<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<ManuallyDrop<i32>>",
        "kani",
        || <RustStdStandard<ManuallyDrop<i32>> as KaniWitness>::proof().to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_MANUALLY_DROP_DEREFS_AND_INTO_INNER_ROUND_TRIP_SRC, {
        /// `ManuallyDrop` is transparent to its wrapped value through
        /// both `Deref` and `into_inner`; the Drop-suppression it exists
        /// for has no observable effect on `i32` (which has no `Drop`
        /// impl to suppress in the first place), so this checks the
        /// access surface that IS observable.
        #[kani::proof]
        fn verify_manually_drop_derefs_and_into_inner_round_trip() {
            let value: i32 = kani::any();
            let wrapped = ManuallyDrop::new(value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*wrapped, value)),
                "deref exposes the wrapped value"
            );
            assert!(
                AccessorRecoversTheExpectedValue::ensures((ManuallyDrop::into_inner(wrapped), value)),
                "into_inner returns the wrapped value"
            );
        }
    }
}

impl KaniWitness for RustStdStandard<Discriminant<Option<i32>>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_discriminant_identifies_variant_not_payload".to_owned(),
            VERIFY_DISCRIMINANT_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Discriminant<Option<i32>>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Discriminant<Option<i32>>>",
        "kani",
        || <RustStdStandard<Discriminant<Option<i32>>> as KaniWitness>::proof()
            .to_string(),
    )
}

amenable_derive::harness! {
    kani, VERIFY_DISCRIMINANT_IDENTIFIES_VARIANT_NOT_PAYLOAD_SRC, {
        /// `discriminant` identifies which variant a value holds,
        /// independent of any payload: two `Some` values with different
        /// payloads share a discriminant, while `Some` and `None` do not.
        #[kani::proof]
        fn verify_discriminant_identifies_variant_not_payload() {
            let a: i32 = kani::any();
            let b: i32 = kani::any();

            assert!(
                CollectedSequenceMatchesExpected::ensures((
                    std::mem::discriminant(&Some(a)),
                    std::mem::discriminant(&Some(b))
                )),
                "same variant, different payloads, share a discriminant"
            );
            assert_ne!(
                std::mem::discriminant(&Some(a)),
                std::mem::discriminant(&None::<i32>),
                "different variants have different discriminants"
            );
        }
    }
}
