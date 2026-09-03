//! `std::time`'s `Instant`, `SystemTime`, `SystemTimeError`, and `Duration`
//! `VerusWitness` impls.

use super::machinery::{VerusCheckedProof, VerusWitness, bridge_verus_witness};
use crate::RustStdStandard;
use amenable_core::Evidence;

const VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Instant> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_instant_model_is_monotonically_nondecreasing".to_owned(),
            VERIFY_INSTANT_MODEL_IS_MONOTONICALLY_NONDECREASING_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Instant>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Instant>",
        "verus",
        || {
            <RustStdStandard<std::time::Instant> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTime> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_time_model_duration_since_computes_the_elapsed_span".to_owned(),
            VERIFY_SYSTEM_TIME_MODEL_DURATION_SINCE_COMPUTES_THE_ELAPSED_SPAN_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTime>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTime>",
        "verus",
        || {
            <RustStdStandard<std::time::SystemTime> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::SystemTimeError> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_system_time_error_model_recovers_how_far_backward_it_went".to_owned(),
            VERIFY_SYSTEM_TIME_ERROR_MODEL_RECOVERS_HOW_FAR_BACKWARD_IT_WENT_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::SystemTimeError>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<SystemTimeError>",
        "verus",
        || {
            <RustStdStandard<std::time::SystemTimeError> as VerusWitness>::proof().to_string()
        },
    )
}

const VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC: &str =
    include_str!("../../../amenable_verus/src/rust_std/misc/std_time_carrier.rs");

impl VerusWitness for RustStdStandard<std::time::Duration> {
    type SupportingEvidence = Self;
    type ProofArtifact = VerusCheckedProof;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn proof() -> Self::ProofArtifact {
        VerusCheckedProof::new(
            "verify_duration_model_new_normalizes_nanos_and_carries_into_secs".to_owned(),
            VERIFY_DURATION_MODEL_NEW_NORMALIZES_NANOS_AND_CARRIES_INTO_SECS_SRC.to_owned(),
            <Self::SupportingEvidence as Evidence>::basis().audit(),
        )
    }
}

bridge_verus_witness!(RustStdStandard<std::time::Duration>);

::inventory::submit! {
    ::amenable_core::ProofRecord::new(
        "amenable_std::rust_std::RustStdStandard<Duration>",
        "verus",
        || {
            <RustStdStandard<std::time::Duration> as VerusWitness>::proof().to_string()
        },
    )
}

amenable_derive::verus_requires_predicate!(
    RustStdStandard<std::time::Duration>,
    "amenable_std::rust_std::RustStdStandard<Duration>",
    "duration_new_secs_headroom_holds"
);

amenable_derive::verus_ensures_predicate!(
    RustStdStandard<std::time::Duration>,
    "amenable_std::rust_std::RustStdStandard<Duration>",
    "duration_new_result_matches"
);
