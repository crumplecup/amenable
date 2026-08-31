use super::CheckedProof;

use crate::{
    ASCII_BYTE_HOLDS_SRC, CreusotVerifier, CreusotWitness, STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC,
    VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC,
};
use amenable_core::{Ensures, Evidence, Provenance, Requires, Witness};

use amenable_std::{AsciiByte, RustStdProvenance, RustStdStandard};

#[expect(
    deprecated,
    reason = "LinesAny is stable, only deprecated in favor of Lines; covering it is a coverage-completeness question, not a call to use it"
)]
type LinesAnyStatic = std::str::LinesAny<'static>;

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
impl CreusotWitness for RustStdStandard<LinesAnyStatic> {
    type SupportingEvidence = Self;
    type ProofArtifact = RustStdProvenance;

    fn proof() -> Self::ProofArtifact {
        <Self::SupportingEvidence as Evidence>::basis().audit()
    }
}

bridge_creusot_witness!(RustStdStandard<LinesAnyStatic>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<LinesAny<'static>>",
        "creusot",
        || <RustStdStandard<LinesAnyStatic> as CreusotWitness>::proof().report().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<str> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_string(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<str>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<str>",
        "creusot",
        || <RustStdStandard<str> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns `amenable_creusot::STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn str_byte_length_and_content_holds` the real site
/// calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<str> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        STR_BYTE_LENGTH_AND_CONTENT_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<str>",
        "creusot",
        "ensures",
        || <RustStdStandard<str> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

/// [`AsciiByte`] reuses the same harness rather than adding a new Creusot
/// proof — it names the precondition the harness already requires, it
/// doesn't prove anything new.
impl CreusotWitness for AsciiByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_str_byte_length_and_content".to_string(),
            VERIFY_STR_BYTE_LENGTH_AND_CONTENT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(AsciiByte);

/// Returns `amenable_creusot::ASCII_BYTE_HOLDS_SRC` directly -- the
/// verbatim, `harness!`-captured source of the real `#[logic(open)] fn
/// ascii_byte_holds` the real site calls, not a hand-retyped copy of its
/// expression.
impl Requires<CreusotVerifier> for AsciiByte {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        ASCII_BYTE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::AsciiByte",
        "creusot",
        "requires",
        || <AsciiByte as Requires<CreusotVerifier>>::requires(()),
    )
}
