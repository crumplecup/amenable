//! The `resolve_local_date_time` exchange.
//!
//!
//! `TemporalZoneFactory` exchange surface. Each transition
//! method's descriptors fold into a `*Request` primary, its
//! `Established<_>` preconditions into a `*Preconditions` proposition,
//! and its return-tuple proofs into a `*Established` proposition (the
//! `proof_composition` fold). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.
//!
//! Each `*Request` type gets a real `derive_new::new` constructor (a
//! genuine gap fixed while building the first real backend against
//! this trait, `amenable_ext::jiff`'s own Phase 4b): these fields are
//! private with only `derive_getters::Getters` read access, and none of
//! them previously had any public constructor at all, `Default`
//! included since some fields -- the plain-value case -- round-trip
//! fine, but a real caller needs to set genuinely different, specific
//! field values (an actual IANA identifier, a real resolution
//! authority), not the all-defaulted case.

use derive_new::new;

use crate::{
    LocalDateTimeDescriptor, LocalDateTimeDoesNotIdentifyFixedInstant, LocalDateTimeValid,
    LocalTimeZoneResolutionAuthorityDescriptor, LocalTimeZoneResolutionProofBranch,
    NamedTimeZoneDescriptor, NamedTimeZoneIdentityValid, NamedZoneAttachmentEvidence,
    OffsetConsistentWithNamedZone, ResolveLocalDateTimeEstablishedToken,
    ResolveLocalDateTimePreconditionsToken, TimestampRepresentsFixedInstant,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Descriptors the `resolve_local_date_time` exchange consumes.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_getters::Getters,
    new,
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
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
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
/// Output sidecar for the `resolve_local_date_time` exchange: the
/// [`ZonedDateTimeDescriptor`](crate::ZonedDateTimeDescriptor) it emits plus a
/// token for the folded [`ResolveLocalDateTimeEstablished`](crate::ResolveLocalDateTimeEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
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
