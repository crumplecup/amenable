use super::CheckedProof;

use std::backtrace::{Backtrace, BacktraceStatus};

use crate::{
    CreusotVerifier, CreusotWitness, VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC,
    VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC,
};
use amenable_core::{Evidence, Witness};

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
impl CreusotWitness for RustStdStandard<Backtrace> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_backtrace_force_capture_always_actually_captures".to_string(),
            VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<Backtrace>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Backtrace>",
        "creusot",
        || <RustStdStandard<Backtrace> as CreusotWitness>::proof().to_string(),
    )
}

impl CreusotWitness for RustStdStandard<BacktraceStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof::new(
            "verify_backtrace_status_reports_captured_after_force_capture".to_string(),
            VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC.to_string(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_creusot_witness!(RustStdStandard<BacktraceStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<BacktraceStatus>",
        "creusot",
        || <RustStdStandard<BacktraceStatus> as CreusotWitness>::proof().to_string(),
    )
}
