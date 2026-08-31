use super::CheckedProof;

use crate::{
    CreusotVerifier, CreusotWitness, STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC,
    VERIFY_STRING_ROUNDTRIP_SRC,
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
impl CreusotWitness for RustStdStandard<String> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_string_roundtrip".to_string(),
            VERIFY_STRING_ROUNDTRIP_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<String>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<String>",
        "creusot",
        || <RustStdStandard<String> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn string_roundtrips_and_preserves_length` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<String> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        STRING_ROUNDTRIPS_AND_PRESERVES_LENGTH_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<String>",
        "creusot",
        "ensures",
        || <RustStdStandard<String> as Ensures<CreusotVerifier>>::ensures(()),
    )
}
