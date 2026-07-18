//! `KaniWitness` impls for `std::time`'s clock types.
//!
//! `SystemTime` and `SystemTimeError` are proved via arithmetic on
//! `SystemTime::UNIX_EPOCH` rather than a real clock read, so the claim
//! is fully deterministic and doesn't depend on when the harness
//! happens to run.

use std::time::{Instant, SystemTime, SystemTimeError};

use amenable_core::Evidence;
use amenable_std::RustStdStandard;

use super::CheckedProof;
use crate::KaniWitness;
use crate::rust_std::macros::bridge_kani_witness;

impl KaniWitness for RustStdStandard<Instant> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_instant_is_monotonically_nondecreasing",
            claim: VERIFY_INSTANT_IS_MONOTONICALLY_NONDECREASING_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<Instant>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<Instant>",
        verifier: "kani",
        describe: || <RustStdStandard<Instant> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_INSTANT_IS_MONOTONICALLY_NONDECREASING_SRC, {
        /// A later `Instant::now()` is never earlier than one taken
        /// before it.
        #[kani::proof]
        fn verify_instant_is_monotonically_nondecreasing() {
            let first = Instant::now();
            let second = Instant::now();
            assert!(second >= first);
        }
    }
}

impl KaniWitness for RustStdStandard<SystemTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_system_time_duration_since_computes_the_elapsed_span",
            claim: VERIFY_SYSTEM_TIME_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SystemTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SystemTime>",
        verifier: "kani",
        describe: || <RustStdStandard<SystemTime> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SYSTEM_TIME_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC, {
        /// `.duration_since()` reports exactly the span between two
        /// `SystemTime`s built from `UNIX_EPOCH` arithmetic — fully
        /// deterministic, no real clock read involved.
        #[kani::proof]
        fn verify_system_time_duration_since_computes_the_elapsed_span() {
            use std::time::Duration;

            let later = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
            let elapsed = later.duration_since(SystemTime::UNIX_EPOCH).unwrap();
            assert_eq!(elapsed, Duration::from_secs(100));
        }
    }
}

impl KaniWitness for RustStdStandard<SystemTimeError> {
    type SupportingEvidence = Self;
    type ProofArtifact = CheckedProof;

    fn proof() -> Self::ProofArtifact {
        CheckedProof {
            harness: "verify_system_time_error_recovers_how_far_backward_it_went",
            claim: VERIFY_SYSTEM_TIME_ERROR_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC,
            provenance: <Self::SupportingEvidence as Evidence>::basis().audit(),
        }
    }
}

bridge_kani_witness!(RustStdStandard<SystemTimeError>);

::inventory::submit! {
    ::amenable_core::ProofRecord {
        evidence: "amenable_std::rust_std::RustStdStandard<SystemTimeError>",
        verifier: "kani",
        describe: || <RustStdStandard<SystemTimeError> as KaniWitness>::proof().to_string(),
    }
}

amenable_derive::harness! {
    kani, VERIFY_SYSTEM_TIME_ERROR_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC, {
        /// `.duration_since()` fails when the argument is later than
        /// `self`, and the resulting error's `.duration()` reports
        /// exactly how far backward that gap is.
        #[kani::proof]
        fn verify_system_time_error_recovers_how_far_backward_it_went() {
            use std::time::Duration;

            let earlier = SystemTime::UNIX_EPOCH;
            let later = SystemTime::UNIX_EPOCH + Duration::from_secs(100);
            match earlier.duration_since(later) {
                Err(err) => assert_eq!(err.duration(), Duration::from_secs(100)),
                Ok(_) => panic!("expected duration_since to fail going backward"),
            }
        }
    }
}
