//! Zone-factory precondition/established establish tokens.
//!
//!
//! Output tokens and their [`Establish`](amenable_core::Establish)
//! edges for the zone / conversion / interval factories.
//!
//! Each transition mints two edges: a `*PreconditionsToken`
//! established from the received input (the caller's individual
//! precondition sidecars are folded into the `*Preconditions`
//! composite, whose `Witness<V>` carries the real content), and a
//! `*EstablishedToken` established from that preconditions token.

/// Lawful token: [`NamedTimeZoneIdentityValid`](crate::NamedTimeZoneIdentityValid) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NamedTimeZoneIdentityValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::NamedTimeZoneIdentityValid"
)]
pub struct NamedTimeZoneIdentityValidToken(());
/// Lawful token: the folded [`ConfirmZoneAuthorityPreconditions`](crate::ConfirmZoneAuthorityPreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ConfirmZoneAuthorityPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ConfirmZoneAuthorityPreconditions"
)]
pub struct ConfirmZoneAuthorityPreconditionsToken(());
/// Lawful token: the folded [`ConfirmZoneAuthorityEstablished`](crate::ConfirmZoneAuthorityEstablished)
/// proofs were re-issued from a proven [`ConfirmZoneAuthorityPreconditions`](crate::ConfirmZoneAuthorityPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ConfirmZoneAuthorityEstablished")]
#[amenable_derive::establish(
    credential = "crate::ConfirmZoneAuthorityPreconditionsToken",
    proposition = "crate::ConfirmZoneAuthorityEstablished"
)]
pub struct ConfirmZoneAuthorityEstablishedToken(());
/// Lawful token: the folded [`ResolveLocalDateTimePreconditions`](crate::ResolveLocalDateTimePreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ResolveLocalDateTimePreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ResolveLocalDateTimePreconditions"
)]
pub struct ResolveLocalDateTimePreconditionsToken(());
/// Lawful token: the folded [`ResolveLocalDateTimeEstablished`](crate::ResolveLocalDateTimeEstablished)
/// proofs were re-issued from a proven [`ResolveLocalDateTimePreconditions`](crate::ResolveLocalDateTimePreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ResolveLocalDateTimeEstablished")]
#[amenable_derive::establish(
    credential = "crate::ResolveLocalDateTimePreconditionsToken",
    proposition = "crate::ResolveLocalDateTimeEstablished"
)]
pub struct ResolveLocalDateTimeEstablishedToken(());
/// Lawful token: the folded [`AttachNamedZonePreconditions`](crate::AttachNamedZonePreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::AttachNamedZonePreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::AttachNamedZonePreconditions"
)]
pub struct AttachNamedZonePreconditionsToken(());
/// Lawful token: the folded [`AttachNamedZoneEstablished`](crate::AttachNamedZoneEstablished)
/// proofs were re-issued from a proven [`AttachNamedZonePreconditions`](crate::AttachNamedZonePreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::AttachNamedZoneEstablished")]
#[amenable_derive::establish(
    credential = "crate::AttachNamedZonePreconditionsToken",
    proposition = "crate::AttachNamedZoneEstablished"
)]
pub struct AttachNamedZoneEstablishedToken(());
/// Lawful token: the folded [`ConfirmNamedZoneRevisionPreconditions`](crate::ConfirmNamedZoneRevisionPreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ConfirmNamedZoneRevisionPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ConfirmNamedZoneRevisionPreconditions"
)]
pub struct ConfirmNamedZoneRevisionPreconditionsToken(());
/// Lawful token: the folded [`ConfirmNamedZoneRevisionEstablished`](crate::ConfirmNamedZoneRevisionEstablished)
/// proofs were re-issued from a proven [`ConfirmNamedZoneRevisionPreconditions`](crate::ConfirmNamedZoneRevisionPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ConfirmNamedZoneRevisionEstablished")]
#[amenable_derive::establish(
    credential = "crate::ConfirmNamedZoneRevisionPreconditionsToken",
    proposition = "crate::ConfirmNamedZoneRevisionEstablished"
)]
pub struct ConfirmNamedZoneRevisionEstablishedToken(());
