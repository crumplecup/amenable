use super::CheckedProof;

use std::ffi::os_str::Display as OsStrDisplay;
use std::ffi::{OsStr, OsString};

use crate::{
    CreusotVerifier, CreusotWitness, OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC,
    VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC,
    VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC,
    VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC,
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
impl CreusotWitness for RustStdStandard<OsStr> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_os_str_valid_utf8_content_round_trips_through_to_str".to_string(),
            VERIFY_OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<OsStr>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsStr>",
        "creusot",
        || <RustStdStandard<OsStr> as CreusotWitness>::proof().to_string(),
    )
}

/// Returns
/// `amenable_creusot::OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC`
/// directly -- the verbatim, `harness!`-captured source of the real
/// `#[logic(open)] fn os_str_valid_utf8_content_round_trips_through_to_str`
/// the real site calls, not a hand-retyped copy of its expression.
impl Ensures<CreusotVerifier> for RustStdStandard<OsStr> {
    type Input = ();
    type Bound = &'static str;

    fn ensures(_: ()) -> &'static str {
        OS_STR_VALID_UTF8_CONTENT_ROUND_TRIPS_THROUGH_TO_STR_SRC
    }
}

::inventory::submit! {
    ::amenable_core::ContractRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsStr>",
        "creusot",
        "ensures",
        || <RustStdStandard<OsStr> as Ensures<CreusotVerifier>>::ensures(()),
    )
}

impl CreusotWitness for RustStdStandard<OsString> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_os_string_push_appends_to_the_existing_content".to_string(),
            VERIFY_OS_STRING_PUSH_APPENDS_TO_THE_EXISTING_CONTENT_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<OsString>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<OsString>",
        "creusot",
        || <RustStdStandard<OsString> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<OsStrDisplay<'static>> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_os_str_display_renders_valid_utf8_content_unchanged".to_string(),
            VERIFY_OS_STR_DISPLAY_RENDERS_VALID_UTF8_CONTENT_UNCHANGED_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<OsStrDisplay<'static>>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<std::ffi::os_str::Display<'static>>",
        "creusot",
        || <RustStdStandard<OsStrDisplay<'static>> as CreusotWitness>::proof().to_string(),
    )
}
