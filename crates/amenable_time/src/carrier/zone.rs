//! UTC-offset/named-zone/zone-transition semantic-bundle tokens and carriers.
//!
//! Phase 5 carrier surface. Each aggregate proof bundle
//! (`proof_composition::semantic_bundles`) gets one `<Bundle>Token`,
//! `#[establish]`-minted from the token that produced it -- the
//! bundle proposition is the *named aggregate*, the token *stands for
//! it*. [`ProvenTemporalCarrier`] pairs a backend-native carrier `T`
//! with one such token; keyed on the token so the token stays the
//! single source of truth (the proposition is
//! `<STok as ProofToken>::Proposition`).
//!
//! The `#[proof_token]` / `#[establish]` / `#[sidecar]` attributes
//! re-parse their `crate::...` string arguments as paths, so no `use`
//! is needed for the token structs themselves.

use crate::ProvenTemporalCarrier;

/// Lawful token standing for the whole [`NamedTimeZoneSemanticBundle`](crate::NamedTimeZoneSemanticBundle),
/// minted from a proven [`NamedTimeZoneIdentityValidToken`](crate::NamedTimeZoneIdentityValidToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NamedTimeZoneSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::NamedTimeZoneIdentityValidToken",
    proposition = "crate::NamedTimeZoneSemanticBundle"
)]
pub struct NamedTimeZoneSemanticBundleToken(());
/// Lawful token standing for the whole [`ZonedDateTimeSemanticBundle`](crate::ZonedDateTimeSemanticBundle),
/// minted from a proven [`AttachNamedZoneEstablishedToken`](crate::AttachNamedZoneEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ZonedDateTimeSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::AttachNamedZoneEstablishedToken",
    proposition = "crate::ZonedDateTimeSemanticBundle"
)]
pub struct ZonedDateTimeSemanticBundleToken(());
/// Lawful token standing for the whole [`ZoneTransitionResolutionAuthorityBundle`](crate::ZoneTransitionResolutionAuthorityBundle),
/// minted from a proven [`ConfirmZoneAuthorityEstablishedToken`](crate::ConfirmZoneAuthorityEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ZoneTransitionResolutionAuthorityBundle")]
#[amenable_derive::establish(
    credential = "crate::ConfirmZoneAuthorityEstablishedToken",
    proposition = "crate::ZoneTransitionResolutionAuthorityBundle"
)]
pub struct ZoneTransitionResolutionAuthorityBundleToken(());
/// Lawful token standing for the whole [`NamedTimeZoneRevisionBundle`](crate::NamedTimeZoneRevisionBundle),
/// minted from a proven [`ConfirmNamedZoneRevisionEstablishedToken`](crate::ConfirmNamedZoneRevisionEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NamedTimeZoneRevisionBundle")]
#[amenable_derive::establish(
    credential = "crate::ConfirmNamedZoneRevisionEstablishedToken",
    proposition = "crate::NamedTimeZoneRevisionBundle"
)]
pub struct NamedTimeZoneRevisionBundleToken(());
/// User-facing proven namedtimezone carrier: a backend-native
/// `NamedTimeZone` value under a [`NamedTimeZoneSemanticBundle`](crate::NamedTimeZoneSemanticBundle) token.
pub type ProvenNamedTimeZoneCarrier<T> = ProvenTemporalCarrier<T, NamedTimeZoneSemanticBundleToken>;
/// User-facing proven zoneddatetime carrier: a backend-native
/// `ZonedDateTime` value under a [`ZonedDateTimeSemanticBundle`](crate::ZonedDateTimeSemanticBundle) token.
pub type ProvenZonedDateTimeCarrier<T> = ProvenTemporalCarrier<T, ZonedDateTimeSemanticBundleToken>;
