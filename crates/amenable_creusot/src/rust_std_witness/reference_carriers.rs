use super::CheckedProof;

use crate::{
    CreusotVerifier, CreusotWitness,
    MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC,
    SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC,
    VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC,
    VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::RustStdStandard;

macro_rules! bridge_creusot_witness {
    ($ty:ty) => {
        impl Witness<CreusotVerifier> for $ty {
            type SupportingEvidence = <$ty as CreusotWitness>::SupportingEvidence;
            type ProofArtifact = <$ty as CreusotWitness>::ProofArtifact;

            fn proof() -> Self::ProofArtifact {
                <$ty as CreusotWitness>::proof()
            }
        }
    };
}
impl CreusotWitness for RustStdStandard<&'static i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_shared_reference_dereferences_to_the_referent".to_string(),
            VERIFY_SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<&'static i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static i32>",
        "creusot",
        || <RustStdStandard<&'static i32> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn shared_reference_dereferences_to_the_referent`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<&'static i32> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        SHARED_REFERENCE_DEREFERENCES_TO_THE_REFERENT_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static i32>",
        "creusot",
        "ensures",
        || <RustStdStandard<&'static i32> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<&'static mut i32> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_mutable_reference_dereferences_to_and_updates_the_referent".to_string(),
            VERIFY_MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<&'static mut i32>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        "creusot",
        || <RustStdStandard<&'static mut i32> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// mutable_reference_dereferences_to_and_updates_the_referent` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<&'static mut i32> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        MUTABLE_REFERENCE_DEREFERENCES_TO_AND_UPDATES_THE_REFERENT_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<&'static mut i32>",
        "creusot",
        "ensures",
        || <RustStdStandard<&'static mut i32> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
