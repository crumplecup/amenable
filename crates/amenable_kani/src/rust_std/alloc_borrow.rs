//! `KaniWitness` impls for `alloc::borrow`.

use std::borrow::Cow;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Cow<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_cow_borrowed_and_owned_agree_on_their_value".to_owned(),
            claim: VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        verifier: "kani",
        describe: || <RustStdStandard<Cow<'static, i32>> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC, {
        /// `Cow::Borrowed` and `Cow::Owned` both deref to the wrapped
        /// value regardless of variant, and `into_owned` preserves it.
        #[kani::proof]
        fn verify_cow_borrowed_and_owned_agree_on_their_value() {
            let value: i32 = kani::any();

            let borrowed: Cow<'_, i32> = Cow::Borrowed(&value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*borrowed, value)),
                "Cow::Borrowed derefs to the wrapped value"
            );
            assert!(
                matches!(borrowed, Cow::Borrowed(_)),
                "Cow::Borrowed constructs the Borrowed variant"
            );

            let owned: Cow<'_, i32> = Cow::Owned(value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*owned, value)),
                "Cow::Owned derefs to the wrapped value"
            );
            assert!(
                matches!(owned, Cow::Owned(_)),
                "Cow::Owned constructs the Owned variant"
            );

            assert_eq!(
                borrowed.into_owned(),
                value,
                "Cow::into_owned preserves the value regardless of variant"
            );
        }
    }
}
