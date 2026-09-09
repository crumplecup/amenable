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

/// Lawful token: the folded [`NormalizeToUtcPreconditions`](crate::NormalizeToUtcPreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NormalizeToUtcPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::NormalizeToUtcPreconditions"
)]
pub struct NormalizeToUtcPreconditionsToken(());

/// Lawful token: the folded [`NormalizeToUtcEstablished`](crate::NormalizeToUtcEstablished)
/// proofs were re-issued from a proven [`NormalizeToUtcPreconditions`](crate::NormalizeToUtcPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NormalizeToUtcEstablished")]
#[amenable_derive::establish(
    credential = "crate::NormalizeToUtcPreconditionsToken",
    proposition = "crate::NormalizeToUtcEstablished"
)]
pub struct NormalizeToUtcEstablishedToken(());

/// Lawful token: the folded [`StripNamedZonePreconditions`](crate::StripNamedZonePreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::StripNamedZonePreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::StripNamedZonePreconditions"
)]
pub struct StripNamedZonePreconditionsToken(());

/// Lawful token: the folded [`StripNamedZoneEstablished`](crate::StripNamedZoneEstablished)
/// proofs were re-issued from a proven [`StripNamedZonePreconditions`](crate::StripNamedZonePreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::StripNamedZoneEstablished")]
#[amenable_derive::establish(
    credential = "crate::StripNamedZonePreconditionsToken",
    proposition = "crate::StripNamedZoneEstablished"
)]
pub struct StripNamedZoneEstablishedToken(());

/// Lawful token: the folded [`AdjustPrecisionLosslesslyPreconditions`](crate::AdjustPrecisionLosslesslyPreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::AdjustPrecisionLosslesslyPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::AdjustPrecisionLosslesslyPreconditions"
)]
pub struct AdjustPrecisionLosslesslyPreconditionsToken(());

/// Lawful token: the folded [`AdjustPrecisionLosslesslyEstablished`](crate::AdjustPrecisionLosslesslyEstablished)
/// proofs were re-issued from a proven [`AdjustPrecisionLosslesslyPreconditions`](crate::AdjustPrecisionLosslesslyPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::AdjustPrecisionLosslesslyEstablished")]
#[amenable_derive::establish(
    credential = "crate::AdjustPrecisionLosslesslyPreconditionsToken",
    proposition = "crate::AdjustPrecisionLosslesslyEstablished"
)]
pub struct AdjustPrecisionLosslesslyEstablishedToken(());

/// Lawful token: the folded [`TruncateSubsecondsPreconditions`](crate::TruncateSubsecondsPreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TruncateSubsecondsPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TruncateSubsecondsPreconditions"
)]
pub struct TruncateSubsecondsPreconditionsToken(());

/// Lawful token: the folded [`TruncateSubsecondsEstablished`](crate::TruncateSubsecondsEstablished)
/// proofs were re-issued from a proven [`TruncateSubsecondsPreconditions`](crate::TruncateSubsecondsPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TruncateSubsecondsEstablished")]
#[amenable_derive::establish(
    credential = "crate::TruncateSubsecondsPreconditionsToken",
    proposition = "crate::TruncateSubsecondsEstablished"
)]
pub struct TruncateSubsecondsEstablishedToken(());

/// Lawful token: the folded [`OrderOffsetEndpointsPreconditions`](crate::OrderOffsetEndpointsPreconditions)
/// were established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrderOffsetEndpointsPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::OrderOffsetEndpointsPreconditions"
)]
pub struct OrderOffsetEndpointsPreconditionsToken(());

/// Lawful token: the folded [`OrderOffsetEndpointsEstablished`](crate::OrderOffsetEndpointsEstablished)
/// proofs were re-issued from a proven [`OrderOffsetEndpointsPreconditions`](crate::OrderOffsetEndpointsPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrderOffsetEndpointsEstablished")]
#[amenable_derive::establish(
    credential = "crate::OrderOffsetEndpointsPreconditionsToken",
    proposition = "crate::OrderOffsetEndpointsEstablished"
)]
pub struct OrderOffsetEndpointsEstablishedToken(());
