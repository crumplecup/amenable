use super::CheckedProof;

use std::boxed::Box;

use crate::{
    BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC, CreusotVerifier, CreusotWitness,
    VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC,
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
impl CreusotWitness for RustStdStandard<Box<i32>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_box_new_preserves_the_wrapped_value".to_string(),
            VERIFY_BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Box<i32>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        "creusot",
        || <RustStdStandard<Box<i32>> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn box_new_preserves_the_wrapped_value` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<Box<i32>> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        BOX_NEW_PRESERVES_THE_WRAPPED_VALUE_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<Box<i32>>",
        "creusot",
        "ensures",
        || <RustStdStandard<Box<i32>> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
