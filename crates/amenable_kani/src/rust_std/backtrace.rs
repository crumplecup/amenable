//! `KaniWitness` impls for `std::backtrace`.

use std::backtrace::{Backtrace, BacktraceStatus};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Backtrace> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_backtrace_force_capture_always_actually_captures".to_owned(),
            claim: VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Backtrace>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Backtrace>",
        verifier: "kani",
        describe: || <RustStdStandard<Backtrace> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BACKTRACE_FORCE_CAPTURE_ALWAYS_ACTUALLY_CAPTURES_SRC, {
        /// `Backtrace::force_capture()` always actually captures,
        /// regardless of the `RUST_BACKTRACE` environment variable —
        /// unlike `Backtrace::capture()`, whose status depends on it.
        #[kani::proof]
        fn verify_backtrace_force_capture_always_actually_captures() {
            let backtrace = Backtrace::force_capture();
            assert_eq!(backtrace.status(), BacktraceStatus::Captured);
        }
    }
}

impl KaniWitness for RustStdStandard<BacktraceStatus> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_backtrace_status_reports_captured_after_force_capture".to_owned(),
            claim: VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC.to_owned(),
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<BacktraceStatus>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<BacktraceStatus>",
        verifier: "kani",
        describe: || <RustStdStandard<BacktraceStatus> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_BACKTRACE_STATUS_REPORTS_CAPTURED_AFTER_FORCE_CAPTURE_SRC, {
        /// Same underlying claim as `Backtrace`, from `BacktraceStatus`'s
        /// own perspective: `Captured` is reachable deterministically.
        #[kani::proof]
        fn verify_backtrace_status_reports_captured_after_force_capture() {
            assert_eq!(Backtrace::force_capture().status(), BacktraceStatus::Captured);
        }
    }
}
