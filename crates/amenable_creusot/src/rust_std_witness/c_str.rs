use super::CheckedProof;

use std::ffi::{CStr, FromBytesUntilNulError, FromBytesWithNulError};
use std::ffi::{CString, FromVecWithNulError, IntoStringError, NulError};

use crate::{
    CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_HOLDS_SRC,
    CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_HOLDS_SRC, CreusotVerifier,
    CreusotWitness, FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_HOLDS_SRC,
    INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_HOLDS_SRC, NON_NUL_BYTE_HOLDS_SRC,
    NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_HOLDS_SRC, NUL_ONLY_AT_THE_END_VALIDATES_SRC,
    VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC,
    VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC,
    VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC,
    VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
    VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC,
    VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC,
    VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC,
};
use amenable_core::{Ensures, Evidence, Requires, Witness};

use amenable_std::{NonNulByte, NulOnlyAtTheEndValidates, RustStdStandard};

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
impl CreusotWitness for RustStdStandard<CString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cstring_excludes_the_terminator_and_rejects_interior_nul".to_string(),
            VERIFY_CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<CString>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CString>",
        "creusot",
        || <RustStdStandard<CString> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// cstring_excludes_the_terminator_and_rejects_interior_nul_holds` the
/// real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<CString> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        CSTRING_EXCLUDES_THE_TERMINATOR_AND_REJECTS_INTERIOR_NUL_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<CString>",
        "creusot",
        "ensures",
        || <RustStdStandard<CString> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<FromVecWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_vec_with_nul_requires_the_nul_only_at_the_end".to_string(),
            VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<FromVecWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromVecWithNulError>",
        "creusot",
        || <RustStdStandard<FromVecWithNulError> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<IntoStringError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_into_string_error_recovers_the_original_cstring".to_string(),
            VERIFY_INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<IntoStringError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<IntoStringError>",
        "creusot",
        || <RustStdStandard<IntoStringError> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// into_string_error_recovers_the_original_cstring_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<IntoStringError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        INTO_STRING_ERROR_RECOVERS_THE_ORIGINAL_CSTRING_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<IntoStringError>",
        "creusot",
        "ensures",
        || <RustStdStandard<IntoStringError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<NulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_nul_error_reports_the_interior_nuls_position".to_string(),
            VERIFY_NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<NulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<NulError>",
        "creusot",
        || <RustStdStandard<NulError> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn nul_error_reports_the_interior_nuls_position_holds`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<NulError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        NUL_ERROR_REPORTS_THE_INTERIOR_NULS_POSITION_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<NulError>",
        "creusot",
        "ensures",
        || <RustStdStandard<NulError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<CStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_string(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<CStr>);

/// Returns
/// `amenable_creusot::CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// cstr_excludes_the_terminating_nul_from_to_bytes_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<CStr> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<CStr>",
        "creusot",
        "ensures",
        || <RustStdStandard<CStr> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<CStr>",
        "creusot",
        || <RustStdStandard<CStr> as CreusotWitness>::proof().to_string(),
    )
}

/// [`NonNulByte`] reuses the same harness rather than adding a new
/// Creusot proof — it names the precondition every `CStr`/`CString`-family
/// proof in this crate already requires, it doesn't prove anything new.
impl CreusotWitness for NonNulByte {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_cstr_excludes_the_terminating_nul_from_to_bytes".to_string(),
            VERIFY_CSTR_EXCLUDES_THE_TERMINATING_NUL_FROM_TO_BYTES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(NonNulByte);

/// Returns `amenable_creusot::NON_NUL_BYTE_HOLDS_SRC` directly — the
/// verbatim, `harness!`-captured source of the real `#[logic(open)] fn
/// non_nul_byte_holds` every site in that cluster now calls, not a
/// hand-retyped copy of its expression. There is exactly one place this
/// precondition's text exists in the whole codebase.
impl Requires<CreusotVerifier> for NonNulByte {
    type Input = ();
    type Bound = &'static str;

    fn requires(_: ()) -> &'static str {
        NON_NUL_BYTE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::NonNulByte",
        "creusot",
        "requires",
        || <NonNulByte as Requires<CreusotVerifier>>::requires(()),
    )
}

/// [`NulOnlyAtTheEndValidates`] reuses the `FromVecWithNulError` harness
/// rather than adding a new Creusot proof — it names the postcondition
/// both `from_vec_with_nul`/`from_bytes_with_nul` proofs already share,
/// it doesn't prove anything new.
impl CreusotWitness for NulOnlyAtTheEndValidates {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_vec_with_nul_requires_the_nul_only_at_the_end".to_string(),
            VERIFY_FROM_VEC_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(NulOnlyAtTheEndValidates);

/// Returns `amenable_creusot::NUL_ONLY_AT_THE_END_VALIDATES_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn nul_only_at_the_end_validates` both
/// `verify_from_vec_with_nul_requires_the_nul_only_at_the_end` and
/// `verify_from_bytes_with_nul_requires_the_nul_only_at_the_end` call,
/// not a hand-retyped copy of its expression. There is exactly one
/// place this postcondition's text exists in the whole codebase.
impl Ensures<CreusotVerifier> for NulOnlyAtTheEndValidates {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        NUL_ONLY_AT_THE_END_VALIDATES_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::NulOnlyAtTheEndValidates",
        "creusot",
        "ensures",
        || <NulOnlyAtTheEndValidates as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<FromBytesUntilNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_bytes_until_nul_requires_a_nul_byte_somewhere".to_string(),
            VERIFY_FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<FromBytesUntilNulError>);

/// Returns
/// `amenable_creusot::FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_HOLDS_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn
/// from_bytes_until_nul_requires_a_nul_byte_somewhere_holds` the real
/// site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<FromBytesUntilNulError> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        FROM_BYTES_UNTIL_NUL_REQUIRES_A_NUL_BYTE_SOMEWHERE_HOLDS_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromBytesUntilNulError>",
        "creusot",
        "ensures",
        || <RustStdStandard<FromBytesUntilNulError> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromBytesUntilNulError>",
        "creusot",
        || <RustStdStandard<FromBytesUntilNulError> as CreusotWitness>::proof()
            .to_string(),
    )
}

impl CreusotWitness for RustStdStandard<FromBytesWithNulError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_from_bytes_with_nul_requires_the_nul_only_at_the_end".to_string(),
            VERIFY_FROM_BYTES_WITH_NUL_REQUIRES_THE_NUL_ONLY_AT_THE_END_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<FromBytesWithNulError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<FromBytesWithNulError>",
        "creusot",
        || <RustStdStandard<FromBytesWithNulError> as CreusotWitness>::proof()
            .to_string(),
    )
}
