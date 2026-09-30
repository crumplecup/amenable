//! Emission-proof composites for the RFC 3339 / IXDTF timestamp family.
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{
    IxdtfAdditionalInformationProofBranch, IxdtfCalendarAnnotationProofBranch,
    IxdtfSerializationCarriesNamedZoneAnnotation, IxdtfTimeZoneAnnotationProofBranch,
    IxdtfTimestampValid, SerializationCarriesExplicitUtcRelationship,
};

/// Emission proof for [`FormattedRfc3339Timestamp`](crate::FormattedRfc3339Timestamp) — the `format_rfc3339_timestamp` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct Rfc3339TimestampFormatted {
    /// The `serialization_carries_explicit_utc_relationship` sub-claim.
    serialization_carries_explicit_utc_relationship: SerializationCarriesExplicitUtcRelationship,
}

/// Emission proof for [`FormattedIxdtfZonedTimestamp`](crate::FormattedIxdtfZonedTimestamp) — the `format_ixdtf_zoned_timestamp` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct IxdtfZonedTimestampFormatted {
    /// The `ixdtf_serialization_carries_named_zone_annotation` sub-claim.
    ixdtf_serialization_carries_named_zone_annotation: IxdtfSerializationCarriesNamedZoneAnnotation,
}

/// Emission proof for [`FormattedIxdtfTimestamp`](crate::FormattedIxdtfTimestamp) — the `format_ixdtf_timestamp` output proof(s), folded.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct IxdtfTimestampFormatted {
    /// The `ixdtf_timestamp_valid` sub-claim.
    ixdtf_timestamp_valid: IxdtfTimestampValid,
    /// The `ixdtf_time_zone_annotation_proof_branch` sub-claim.
    ixdtf_time_zone_annotation_proof_branch: IxdtfTimeZoneAnnotationProofBranch,
    /// The `ixdtf_calendar_annotation_proof_branch` sub-claim.
    ixdtf_calendar_annotation_proof_branch: IxdtfCalendarAnnotationProofBranch,
    /// The `ixdtf_additional_information_proof_branch` sub-claim.
    ixdtf_additional_information_proof_branch: IxdtfAdditionalInformationProofBranch,
}
