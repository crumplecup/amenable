//! Named-zone/local-date-time resolution native-carrier exchange types.
//!
//! Phase 5 Step 4b -- the `*_native` factory analogs' exchange types.
//! Carrier<->carrier: the native-carrier versions of the
//! descriptor-side factory transitions. Multi-input methods fold their
//! runtime values into a `<M>NativeRequest<B>` primary (basis = the
//! [`NativeCarrierRequest`] marker, so the compound needs no `Default`);
//! methods with an extra output proof fold it into a `<M>NativeEstablished`
//! composite behind a `<M>NativeToken`.

use derive_new::new;

use crate::{
    LocalTimeZoneResolutionAuthorityDescriptor, LocalTimeZoneResolutionProofBranch,
    NamedTimeZoneRevisionBundle, NamedTimeZoneRevisionBundleToken, TemporalCivilProps,
    TemporalInputToken, TemporalInstantProps, TemporalZoneProps,
    ZoneTransitionResolutionAuthorityBundle, ZonedDateTimeSemanticBundle,
};

/// Runtime values the `resolve_local_date_time_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters, new)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct ResolveLocalDateTimeNativeRequest<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    /// The `timestamp` input.
    timestamp: B::LocalDateTime,
    /// The `zone` input.
    zone: B::NamedTimeZone,
    /// The `resolution_authority` input.
    resolution_authority: LocalTimeZoneResolutionAuthorityDescriptor,
    /// The `authority` input.
    authority: ZoneTransitionResolutionAuthorityBundle,
}
/// Input sidecar for the `resolve_local_date_time_native` exchange.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct ResolveLocalDateTimeNativeInput<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    #[sidecar(primary)]
    request: ResolveLocalDateTimeNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}
/// Proofs the `resolve_local_date_time_native` exchange re-issues, folded into one proposition.
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
pub struct ResolveLocalDateTimeNativeEstablished {
    /// The re-issued `zoned_date_time_semantic_bundle` sub-claim.
    zoned_date_time_semantic_bundle: ZonedDateTimeSemanticBundle,
    /// The re-issued `zone_transition_resolution_authority_bundle` sub-claim.
    zone_transition_resolution_authority_bundle: ZoneTransitionResolutionAuthorityBundle,
    /// The re-issued `local_time_zone_resolution_proof_branch` sub-claim.
    local_time_zone_resolution_proof_branch: LocalTimeZoneResolutionProofBranch,
}
/// Lawful token for the folded [`ResolveLocalDateTimeNativeEstablished`](crate::ResolveLocalDateTimeNativeEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ResolveLocalDateTimeNativeEstablished")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ResolveLocalDateTimeNativeEstablished"
)]
pub struct ResolveLocalDateTimeNativeToken(());
/// Output sidecar for the `resolve_local_date_time_native` exchange: the emitted
/// native `ZonedDateTime` carrier + a [`ResolveLocalDateTimeNativeEstablished`](crate::ResolveLocalDateTimeNativeEstablished) token.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::ResolveLocalDateTimeNativeEstablished",
    constructor = "pub"
)]
pub struct ResolveLocalDateTimeNativeOutput<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    #[sidecar(primary)]
    carrier: B::ZonedDateTime,
    #[sidecar(token)]
    token: ResolveLocalDateTimeNativeToken,
}
/// Runtime values the `attach_named_zone_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters, new)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct AttachNamedZoneNativeRequest<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    /// The `timestamp` input.
    timestamp: B::OffsetDateTime,
    /// The `zone` input.
    zone: B::NamedTimeZone,
}
/// Input sidecar for the `attach_named_zone_native` exchange.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct AttachNamedZoneNativeInput<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    #[sidecar(primary)]
    request: AttachNamedZoneNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}
/// Output sidecar for the `confirm_named_zone_revision_native` exchange — pure proof,
/// the [`NamedTimeZoneRevisionBundle`](crate::NamedTimeZoneRevisionBundle) it re-issues + its token.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::NamedTimeZoneRevisionBundle",
    constructor = "pub"
)]
pub struct ConfirmNamedZoneRevisionNativeOutput {
    #[sidecar(primary)]
    bundle: NamedTimeZoneRevisionBundle,
    #[sidecar(token)]
    token: NamedTimeZoneRevisionBundleToken,
}
