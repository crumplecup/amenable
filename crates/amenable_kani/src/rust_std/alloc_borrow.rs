//! `KaniWitness` impls for `alloc::borrow`.

use std::borrow::Cow;

#[cfg(kani)]
use amenable_core::Ensures;
use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
#[cfg(kani)]
use crate::AccessorRecoversTheExpectedValue;
#[cfg(kani)]
use crate::DerefReflectsTheStoredValue;
use crate::KaniWitness;
use crate::rust_std::macros::{bridge_kani_witness, kani_ensures};

impl KaniWitness for RustStdStandard<Cow<'static, i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cow_borrowed_and_owned_agree_on_their_value".to_owned(),
            VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(RustStdStandard<Cow<'static, i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Cow<'static, i32>>",
        "kani",
        || <RustStdStandard<Cow<'static, i32>> as KaniWitness>::proof().to_string(),
    )
}

/// A `bool` known to be the `true` `matches!(cow, Cow::Borrowed(_))`
/// reports when a `Cow` is actually constructed as the `Borrowed`
/// variant -- following `IsContinueVariantReportsTrue`'s established
/// shape for a raw boolean claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct CowConstructsBorrowedVariant;

impl KaniWitness for CowConstructsBorrowedVariant {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cow_borrowed_and_owned_agree_on_their_value".to_owned(),
            VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(CowConstructsBorrowedVariant);

kani_ensures!(
    CowConstructsBorrowedVariant,
    "amenable_kani::CowConstructsBorrowedVariant",
    bool,
    |is_borrowed| is_borrowed
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CowConstructsBorrowedVariant",
        "kani",
        || <CowConstructsBorrowedVariant as KaniWitness>::proof().to_string(),
    )
}

/// The `Owned` sibling of [`CowConstructsBorrowedVariant`], same
/// reasoning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, amenable_derive::Standard)]
#[standard(
    basis = "RustStdStandard<i32>",
    basis_ctor = "RustStdStandard::<i32>::new()",
    provenance = "<i32 as amenable_std::RustStdType>::provenance()",
    provenance_type = "amenable_std::RustStdProvenance"
)]
pub struct CowConstructsOwnedVariant;

impl KaniWitness for CowConstructsOwnedVariant {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cow_borrowed_and_owned_agree_on_their_value".to_owned(),
            VERIFY_COW_BORROWED_AND_OWNED_AGREE_ON_THEIR_VALUE_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_kani_witness!(CowConstructsOwnedVariant);

kani_ensures!(
    CowConstructsOwnedVariant,
    "amenable_kani::CowConstructsOwnedVariant",
    bool,
    |is_owned| is_owned
);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_kani::CowConstructsOwnedVariant",
        "kani",
        || <CowConstructsOwnedVariant as KaniWitness>::proof().to_string(),
    )
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
                CowConstructsBorrowedVariant::ensures(matches!(borrowed, Cow::Borrowed(_))),
                "Cow::Borrowed constructs the Borrowed variant"
            );

            let owned: Cow<'_, i32> = Cow::Owned(value);
            assert!(
                DerefReflectsTheStoredValue::ensures((*owned, value)),
                "Cow::Owned derefs to the wrapped value"
            );
            assert!(
                CowConstructsOwnedVariant::ensures(matches!(owned, Cow::Owned(_))),
                "Cow::Owned constructs the Owned variant"
            );

            assert!(
                AccessorRecoversTheExpectedValue::ensures((borrowed.into_owned(), value)),
                "Cow::into_owned preserves the value regardless of variant"
            );
        }
    }
}
