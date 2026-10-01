//! Temporal-set and date-time-formula (derived/composed) establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`TemporalSetProof`](crate::TemporalSetProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TemporalSetProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TemporalSetProof"
)]
pub struct TemporalSetProofToken(());

/// Lawful token: [`TemporalSetFormatted`](crate::TemporalSetFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TemporalSetFormatted")]
#[amenable_derive::establish(
    credential = "crate::TemporalSetProofToken",
    proposition = "crate::TemporalSetFormatted"
)]
pub struct TemporalSetFormattedToken(());

/// Lawful token: [`GroupedTimeScaleUnitProof`](crate::GroupedTimeScaleUnitProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::GroupedTimeScaleUnitProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::GroupedTimeScaleUnitProof"
)]
pub struct GroupedTimeScaleUnitProofToken(());

/// Lawful token: [`GroupedTimeScaleUnitFormatted`](crate::GroupedTimeScaleUnitFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::GroupedTimeScaleUnitFormatted")]
#[amenable_derive::establish(
    credential = "crate::GroupedTimeScaleUnitProofToken",
    proposition = "crate::GroupedTimeScaleUnitFormatted"
)]
pub struct GroupedTimeScaleUnitFormattedToken(());

/// Lawful token: [`DateTimeFormulaProof`](crate::DateTimeFormulaProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateTimeFormulaProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DateTimeFormulaProof"
)]
pub struct DateTimeFormulaProofToken(());

/// Lawful token: [`DateTimeFormulaFormatted`](crate::DateTimeFormulaFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateTimeFormulaFormatted")]
#[amenable_derive::establish(
    credential = "crate::DateTimeFormulaProofToken",
    proposition = "crate::DateTimeFormulaFormatted"
)]
pub struct DateTimeFormulaFormattedToken(());
