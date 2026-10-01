//! RFC 3339 / IXDTF timestamp parse composite proofs.
//!
//!
//! Per-method composite proof propositions -- for the parse methods
//! whose `elicit_temporal` return tuple carries 2+ proof sidecars, the
//! sidecars folded into one `#[derive(Evidence, Witness)]` struct (the
//! same folding as `proof_composition`) so the output `Sidecar` keeps
//! its single-`token` shape.

use crate::{
    IxdtfAdditionalInformationProofBranch, IxdtfCalendarAnnotationProofBranch,
    IxdtfTimeZoneAnnotationProofBranch, IxdtfTimestampValid, OffsetConsistentWithNamedZone,
    OffsetDateTimeValid, Rfc3339TimestampValid, TimestampRepresentsFixedInstant,
    ZonedDateTimeHasNamedZone,
};

/// Proof for [`ParsedRfc3339Timestamp`](crate::ParsedRfc3339Timestamp) — the 3 proof sidecars `elicit_temporal`'s `parse_rfc3339_timestamp` returns, folded into one composite proposition.
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
pub struct Rfc3339TimestampProof {
    /// The `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The `rfc3339_timestamp_valid` sub-claim.
    rfc3339_timestamp_valid: Rfc3339TimestampValid,
    /// The `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
}
/// Proof for [`ParsedIxdtfTimestamp`](crate::ParsedIxdtfTimestamp) — the 5 proof sidecars `elicit_temporal`'s `parse_ixdtf_timestamp` returns, folded into one composite proposition.
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
pub struct IxdtfTimestampProof {
    /// The `ixdtf_timestamp_valid` sub-claim.
    ixdtf_timestamp_valid: IxdtfTimestampValid,
    /// The `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
    /// The `ixdtf_time_zone_annotation_proof_branch` sub-claim.
    ixdtf_time_zone_annotation_proof_branch: IxdtfTimeZoneAnnotationProofBranch,
    /// The `ixdtf_calendar_annotation_proof_branch` sub-claim.
    ixdtf_calendar_annotation_proof_branch: IxdtfCalendarAnnotationProofBranch,
    /// The `ixdtf_additional_information_proof_branch` sub-claim.
    ixdtf_additional_information_proof_branch: IxdtfAdditionalInformationProofBranch,
}
/// Composite input proposition for the matching formatter method(s).
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
pub struct IxdtfZonedTimestampProof {
    /// The `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
    /// The `zoned_date_time_has_named_zone` sub-claim.
    zoned_date_time_has_named_zone: ZonedDateTimeHasNamedZone,
    /// The `offset_consistent_with_named_zone` sub-claim.
    offset_consistent_with_named_zone: OffsetConsistentWithNamedZone,
}
