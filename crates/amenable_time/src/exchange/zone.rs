//! `TemporalZoneFactory` exchange surface. Each transition
//! method's descriptors fold into a `*Request` primary, its
//! `Established<_>` preconditions into a `*Preconditions` proposition,
//! and its return-tuple proofs into a `*Established` proposition (the
//! `proof_composition` fold). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    AttachNamedZoneEstablishedToken, AttachNamedZonePreconditionsToken,
    ConfirmNamedZoneRevisionEstablishedToken, ConfirmNamedZoneRevisionPreconditionsToken,
    ConfirmZoneAuthorityEstablishedToken, ConfirmZoneAuthorityPreconditionsToken,
    LocalDateTimeDescriptor, LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid,
    LocalTimeZoneResolutionAuthorityDescriptor, LocalTimeZoneResolutionProofBranch,
    NamedTimeZoneDescriptor, NamedTimeZoneIdentityValid, NamedTimeZoneIdentityValidToken,
    NamedTimeZoneInterpretationTracksTzdbRevision, NamedZoneAttachmentEvidence,
    OffsetConsistentWithNamedZone, OffsetDateTimeDescriptor, ResolveLocalDateTimeEstablishedToken,
    ResolveLocalDateTimePreconditionsToken, TimestampRepresentsFixedInstant,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Output sidecar for the `resolve_named_zone` exchange: [`NamedTimeZoneDescriptor`](crate::NamedTimeZoneDescriptor)
/// plus a token for [`NamedTimeZoneIdentityValid`](crate::NamedTimeZoneIdentityValid).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::NamedTimeZoneIdentityValid", constructor = "pub")]
pub struct ResolvedNamedTimeZone {
    #[sidecar(primary)]
    descriptor: NamedTimeZoneDescriptor,
    #[sidecar(token)]
    token: NamedTimeZoneIdentityValidToken,
}

impl ResolvedNamedTimeZone {
    /// Borrow the resolved descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &NamedTimeZoneDescriptor {
        &self.descriptor
    }
}

/// Descriptors the `confirm_local_time_zone_resolution_authority` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ConfirmZoneAuthorityRequest {
    /// The `zone` descriptor.
    zone: NamedTimeZoneDescriptor,
    /// The `resolution_authority` descriptor.
    resolution_authority: LocalTimeZoneResolutionAuthorityDescriptor,
}

/// Preconditions the `confirm_local_time_zone_resolution_authority` exchange requires — the
/// caller's `Established<_>` sidecars, folded into one proposition.
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
pub struct ConfirmZoneAuthorityPreconditions {
    /// The established `named_time_zone_identity_valid` sub-claim.
    named_time_zone_identity_valid: NamedTimeZoneIdentityValid,
}

/// Proofs the `confirm_local_time_zone_resolution_authority` exchange re-issues, folded into one
/// proposition (the `proof_composition` structural closure).
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
pub struct ConfirmZoneAuthorityEstablished {
    /// The re-issued `zone_transition_resolution_authority_valid` sub-claim.
    zone_transition_resolution_authority_valid: ZoneTransitionResolutionAuthorityValid,
}

/// Input sidecar for the `confirm_local_time_zone_resolution_authority` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ConfirmZoneAuthorityPreconditions",
    constructor = "pub"
)]
pub struct ConfirmZoneAuthorityInput {
    #[sidecar(primary)]
    request: ConfirmZoneAuthorityRequest,
    #[sidecar(token)]
    token: ConfirmZoneAuthorityPreconditionsToken,
}

impl ConfirmZoneAuthorityInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &ConfirmZoneAuthorityRequest {
        &self.request
    }
}

/// Output sidecar for the `confirm_local_time_zone_resolution_authority` exchange — no descriptor,
/// so the folded [`ConfirmZoneAuthorityEstablished`](crate::ConfirmZoneAuthorityEstablished) proposition
/// is itself the primary payload.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(constructor = "pub")]
pub struct ConfirmZoneAuthorityOutput {
    #[sidecar(primary)]
    established: ConfirmZoneAuthorityEstablished,
    #[sidecar(token)]
    token: ConfirmZoneAuthorityEstablishedToken,
}

impl ConfirmZoneAuthorityOutput {
    /// Borrow the re-issued proof composite.
    #[must_use]
    pub fn established(&self) -> &ConfirmZoneAuthorityEstablished {
        &self.established
    }
}

/// Descriptors the `resolve_local_date_time` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ResolveLocalDateTimeRequest {
    /// The `timestamp` descriptor.
    timestamp: LocalDateTimeDescriptor,
    /// The `zone` descriptor.
    zone: NamedTimeZoneDescriptor,
    /// The `resolution_authority` descriptor.
    resolution_authority: LocalTimeZoneResolutionAuthorityDescriptor,
}

/// Preconditions the `resolve_local_date_time` exchange requires — the
/// caller's `Established<_>` sidecars, folded into one proposition.
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
pub struct ResolveLocalDateTimePreconditions {
    /// The established `local_date_time_valid` sub-claim.
    local_date_time_valid: LocalDateTimeValid,
    /// The established `local_date_time_does_not_identify_fixed_instant` sub-claim.
    local_date_time_does_not_identify_fixed_instant: LocalDateTimeDoesNotIdentifyFixedInstant,
    /// The established `named_time_zone_identity_valid` sub-claim.
    named_time_zone_identity_valid: NamedTimeZoneIdentityValid,
    /// The established `zone_transition_resolution_authority_valid` sub-claim.
    zone_transition_resolution_authority_valid: ZoneTransitionResolutionAuthorityValid,
}

/// Proofs the `resolve_local_date_time` exchange re-issues, folded into one
/// proposition (the `proof_composition` structural closure).
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
pub struct ResolveLocalDateTimeEstablished {
    /// The re-issued `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
    /// The re-issued `zoned_date_time_has_named_zone` sub-claim.
    zoned_date_time_has_named_zone: ZonedDateTimeHasNamedZone,
    /// The re-issued `named_zone_attachment_evidence` sub-claim.
    named_zone_attachment_evidence: NamedZoneAttachmentEvidence,
    /// The re-issued `offset_consistent_with_named_zone` sub-claim.
    offset_consistent_with_named_zone: OffsetConsistentWithNamedZone,
    /// The re-issued `zone_transition_resolution_authority_valid` sub-claim.
    zone_transition_resolution_authority_valid: ZoneTransitionResolutionAuthorityValid,
    /// The re-issued `local_time_zone_resolution_proof_branch` sub-claim.
    local_time_zone_resolution_proof_branch: LocalTimeZoneResolutionProofBranch,
}

/// Input sidecar for the `resolve_local_date_time` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ResolveLocalDateTimePreconditions",
    constructor = "pub"
)]
pub struct ResolveLocalDateTimeInput {
    #[sidecar(primary)]
    request: ResolveLocalDateTimeRequest,
    #[sidecar(token)]
    token: ResolveLocalDateTimePreconditionsToken,
}

impl ResolveLocalDateTimeInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &ResolveLocalDateTimeRequest {
        &self.request
    }
}

/// Output sidecar for the `resolve_local_date_time` exchange: the
/// [`ZonedDateTimeDescriptor`](crate::ZonedDateTimeDescriptor) it emits plus a
/// token for the folded [`ResolveLocalDateTimeEstablished`](crate::ResolveLocalDateTimeEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ResolveLocalDateTimeEstablished",
    constructor = "pub"
)]
pub struct ResolveLocalDateTimeOutput {
    #[sidecar(primary)]
    descriptor: ZonedDateTimeDescriptor,
    #[sidecar(token)]
    token: ResolveLocalDateTimeEstablishedToken,
}

impl ResolveLocalDateTimeOutput {
    /// Borrow the emitted descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ZonedDateTimeDescriptor {
        &self.descriptor
    }
}

/// Descriptors the `attach_named_zone` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct AttachNamedZoneRequest {
    /// The `timestamp` descriptor.
    timestamp: OffsetDateTimeDescriptor,
    /// The `zone` descriptor.
    zone: NamedTimeZoneDescriptor,
}

/// Preconditions the `attach_named_zone` exchange requires — the
/// caller's `Established<_>` sidecars, folded into one proposition.
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
pub struct AttachNamedZonePreconditions {
    /// The established `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
    /// The established `named_time_zone_identity_valid` sub-claim.
    named_time_zone_identity_valid: NamedTimeZoneIdentityValid,
}

/// Proofs the `attach_named_zone` exchange re-issues, folded into one
/// proposition (the `proof_composition` structural closure).
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
pub struct AttachNamedZoneEstablished {
    /// The re-issued `zoned_date_time_has_named_zone` sub-claim.
    zoned_date_time_has_named_zone: ZonedDateTimeHasNamedZone,
    /// The re-issued `named_zone_attachment_evidence` sub-claim.
    named_zone_attachment_evidence: NamedZoneAttachmentEvidence,
    /// The re-issued `offset_consistent_with_named_zone` sub-claim.
    offset_consistent_with_named_zone: OffsetConsistentWithNamedZone,
}

/// Input sidecar for the `attach_named_zone` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::AttachNamedZonePreconditions",
    constructor = "pub"
)]
pub struct AttachNamedZoneInput {
    #[sidecar(primary)]
    request: AttachNamedZoneRequest,
    #[sidecar(token)]
    token: AttachNamedZonePreconditionsToken,
}

impl AttachNamedZoneInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &AttachNamedZoneRequest {
        &self.request
    }
}

/// Output sidecar for the `attach_named_zone` exchange: the
/// [`ZonedDateTimeDescriptor`](crate::ZonedDateTimeDescriptor) it emits plus a
/// token for the folded [`AttachNamedZoneEstablished`](crate::AttachNamedZoneEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::AttachNamedZoneEstablished", constructor = "pub")]
pub struct AttachNamedZoneOutput {
    #[sidecar(primary)]
    descriptor: ZonedDateTimeDescriptor,
    #[sidecar(token)]
    token: AttachNamedZoneEstablishedToken,
}

impl AttachNamedZoneOutput {
    /// Borrow the emitted descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ZonedDateTimeDescriptor {
        &self.descriptor
    }
}

/// Descriptors the `confirm_named_zone_revision` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ConfirmNamedZoneRevisionRequest {
    /// The `timestamp` descriptor.
    timestamp: ZonedDateTimeDescriptor,
}

/// Preconditions the `confirm_named_zone_revision` exchange requires — the
/// caller's `Established<_>` sidecars, folded into one proposition.
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
pub struct ConfirmNamedZoneRevisionPreconditions {
    /// The established `zoned_date_time_has_named_zone` sub-claim.
    zoned_date_time_has_named_zone: ZonedDateTimeHasNamedZone,
}

/// Proofs the `confirm_named_zone_revision` exchange re-issues, folded into one
/// proposition (the `proof_composition` structural closure).
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
pub struct ConfirmNamedZoneRevisionEstablished {
    /// The re-issued `named_time_zone_interpretation_tracks_tzdb_revision` sub-claim.
    named_time_zone_interpretation_tracks_tzdb_revision:
        NamedTimeZoneInterpretationTracksTzdbRevision,
}

/// Input sidecar for the `confirm_named_zone_revision` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ConfirmNamedZoneRevisionPreconditions",
    constructor = "pub"
)]
pub struct ConfirmNamedZoneRevisionInput {
    #[sidecar(primary)]
    request: ConfirmNamedZoneRevisionRequest,
    #[sidecar(token)]
    token: ConfirmNamedZoneRevisionPreconditionsToken,
}

impl ConfirmNamedZoneRevisionInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &ConfirmNamedZoneRevisionRequest {
        &self.request
    }
}

/// Output sidecar for the `confirm_named_zone_revision` exchange — no descriptor,
/// so the folded [`ConfirmNamedZoneRevisionEstablished`](crate::ConfirmNamedZoneRevisionEstablished) proposition
/// is itself the primary payload.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(constructor = "pub")]
pub struct ConfirmNamedZoneRevisionOutput {
    #[sidecar(primary)]
    established: ConfirmNamedZoneRevisionEstablished,
    #[sidecar(token)]
    token: ConfirmNamedZoneRevisionEstablishedToken,
}

impl ConfirmNamedZoneRevisionOutput {
    /// Borrow the re-issued proof composite.
    #[must_use]
    pub fn established(&self) -> &ConfirmNamedZoneRevisionEstablished {
        &self.established
    }
}
