use super::CheckedProof;

use crate::{
    CHAR_ROUNDTRIPS_SRC, CreusotVerifier, CreusotWitness, VALID_UNICODE_SCALAR_HOLDS_SRC,
    VERIFY_CHAR_ROUNDTRIP_SRC,
};
use amenable_core::{Ensures, Evidence, Witness};

use amenable_std::{RustStdStandard, ValidUnicodeScalar};

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
impl CreusotWitness for RustStdStandard<char> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_roundtrip".to_string(),
            VERIFY_CHAR_ROUNDTRIP_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<char>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<char>",
        "creusot",
        || <RustStdStandard<char> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::CHAR_ROUNDTRIPS_SRC` directly -- the
/// verbatim, `harness!`-captured source of the real `#[logic(open)] fn
/// char_roundtrips` the real site calls, not a hand-retyped copy of
/// its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<char> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        CHAR_ROUNDTRIPS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<char>",
        "creusot",
        "ensures",
        || <RustStdStandard<char> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

/// The [`ValidUnicodeScalar`] contract type reuses `verify_char_roundtrip`
/// rather than adding a new Creusot proof — it names the postcondition the
/// harness already checks (`c@ <= 0xD7FF || (c@ >= 0xE000 && c@ <=
/// 0x10FFFF)`, via the named `valid_unicode_scalar_holds` predicate), it
/// doesn't prove anything new.
impl CreusotWitness for ValidUnicodeScalar {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_char_roundtrip".to_string(),
            VERIFY_CHAR_ROUNDTRIP_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(ValidUnicodeScalar);

/// Returns `amenable_creusot::VALID_UNICODE_SCALAR_HOLDS_SRC` directly --
/// the verbatim, `harness!`-captured source of the real `#[logic(open)] fn
/// valid_unicode_scalar_holds` the real site calls, not a hand-retyped
/// copy of its expression.
impl Ensures<CreusotVerifier> for ValidUnicodeScalar {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        VALID_UNICODE_SCALAR_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::ValidUnicodeScalar",
        "creusot",
        "ensures",
        || <ValidUnicodeScalar as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::ValidUnicodeScalar",
        "creusot",
        || <ValidUnicodeScalar as CreusotWitness>::proof().to_string(),
    )
}
