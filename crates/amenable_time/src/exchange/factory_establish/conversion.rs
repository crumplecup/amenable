//! Conversion-factory precondition/established establish tokens.
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
