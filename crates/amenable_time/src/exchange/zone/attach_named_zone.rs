//! The `attach_named_zone` exchange.
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
    AttachNamedZoneEstablishedToken, AttachNamedZonePreconditionsToken, NamedTimeZoneDescriptor,
    NamedTimeZoneIdentityValid, NamedZoneAttachmentEvidence, OffsetConsistentWithNamedZone,
    OffsetDateTimeDescriptor, TimestampRepresentsFixedInstant, ZonedDateTimeDescriptor,
    ZonedDateTimeHasNamedZone,
};

/// Descriptors the `attach_named_zone` exchange consumes.
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
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
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
/// Output sidecar for the `attach_named_zone` exchange: the
/// [`ZonedDateTimeDescriptor`](crate::ZonedDateTimeDescriptor) it emits plus a
/// token for the folded [`AttachNamedZoneEstablished`](crate::AttachNamedZoneEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::AttachNamedZoneEstablished", constructor = "pub")]
pub struct AttachNamedZoneOutput {
    #[sidecar(primary)]
    descriptor: ZonedDateTimeDescriptor,
    #[sidecar(token)]
    token: AttachNamedZoneEstablishedToken,
}
