//! Named-zone identity resolution and local-time-zone resolution-authority confirmation.
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
    ConfirmZoneAuthorityEstablishedToken, ConfirmZoneAuthorityPreconditionsToken,
    LocalTimeZoneResolutionAuthorityDescriptor, NamedTimeZoneDescriptor,
    NamedTimeZoneIdentityValid, NamedTimeZoneIdentityValidToken,
    ZoneTransitionResolutionAuthorityValid,
};

/// Output sidecar for the `resolve_named_zone` exchange: [`NamedTimeZoneDescriptor`](crate::NamedTimeZoneDescriptor)
/// plus a token for [`NamedTimeZoneIdentityValid`](crate::NamedTimeZoneIdentityValid).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::NamedTimeZoneIdentityValid", constructor = "pub")]
pub struct ResolvedNamedTimeZone {
    #[sidecar(primary)]
    descriptor: NamedTimeZoneDescriptor,
    #[sidecar(token)]
    token: NamedTimeZoneIdentityValidToken,
}
/// Descriptors the `confirm_local_time_zone_resolution_authority` exchange consumes.
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
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
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
/// Output sidecar for the `confirm_local_time_zone_resolution_authority` exchange — no descriptor,
/// so the folded [`ConfirmZoneAuthorityEstablished`](crate::ConfirmZoneAuthorityEstablished) proposition
/// is itself the primary payload.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(constructor = "pub")]
pub struct ConfirmZoneAuthorityOutput {
    #[sidecar(primary)]
    established: ConfirmZoneAuthorityEstablished,
    #[sidecar(token)]
    token: ConfirmZoneAuthorityEstablishedToken,
}
