//! `KaniWitness` impls for `std::backtrace`.
//!
//! The direct `Backtrace::force_capture()` path depends on platform unwinding
//! internals that Kani cannot execute today. Production proofs therefore use
//! an Amenable-owned backtrace model instead; the direct std path remains
//! preserved in the gallery as a false trail.

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
        /// This proof uses the Amenable-owned backtrace accommodation model:
        /// if the real capture path refines these modeled laws, the
        /// Rust-facing claim follows.
        #[kani::proof]
        fn verify_backtrace_force_capture_always_actually_captures() {
            let backtrace = crate::KaniBacktrace::force_capture();
            assert_eq!(backtrace.status(), crate::KaniBacktraceStatus::Captured);
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
        /// Same underlying status law as `Backtrace`, from the status type's
        /// own perspective: forced capture deterministically yields
        /// `Captured` in the accommodation model.
        #[kani::proof]
        fn verify_backtrace_status_reports_captured_after_force_capture() {
            let status = crate::KaniBacktrace::force_capture().status();
            assert_eq!(status, crate::KaniBacktraceStatus::Captured);
        }
    }
}
