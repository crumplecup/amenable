//! UTC-offset/named-zone/zone-transition family semantic bundles.
//!
//!
//! The `elicit_temporal` `*Bundle` aggregate proof bundles, ported as
//! folded `#[derive(Evidence, Witness)]` composites (the same
//! structural closure as the rest of `proof_composition`). Each
//! `Established<X>` field collapses to `X`; a `<Foo>Evidence` field
//! already merged into its `*Valid` sibling is dropped; `*ProofBranch`
//! and standalone `*Evidence` fields are kept. These pair with the
//! Phase 5 `ProvenTemporalCarrier` carrier wrapper.

use crate::{
    BackendConversionSemanticBundle, NamedTimeZoneIdentityValid,
    NamedTimeZoneInterpretationTracksTzdbRevision, NamedZoneAttachmentEvidence,
    OffsetConsistentWithNamedZone, TimestampRepresentsFixedInstant,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeHasNamedZone,
};

/// Aggregate semantic bundle for one named time-zone carrier.
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
pub struct NamedTimeZoneSemanticBundle {
    /// The `identity` sub-claim.
    identity: NamedTimeZoneIdentityValid,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one named-zone-attached fixed-instant carrier.
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
pub struct ZonedDateTimeSemanticBundle {
    /// The `fixed_instant` sub-claim.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The `zone_identity` sub-claim.
    zone_identity: NamedTimeZoneIdentityValid,
    /// The `zone_attachment` sub-claim.
    zone_attachment: ZonedDateTimeHasNamedZone,
    /// The `zone_attachment_evidence` sub-claim.
    zone_attachment_evidence: NamedZoneAttachmentEvidence,
    /// The `offset_consistency` sub-claim.
    offset_consistency: OffsetConsistentWithNamedZone,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for explicit local-to-zone resolution authority.
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
pub struct ZoneTransitionResolutionAuthorityBundle {
    /// The `semantics` sub-claim.
    semantics: ZoneTransitionResolutionAuthorityValid,
}

/// Aggregate semantic bundle for one named-zone revision interpretation result.
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
pub struct NamedTimeZoneRevisionBundle {
    /// The `semantics` sub-claim.
    semantics: NamedTimeZoneInterpretationTracksTzdbRevision,
}
