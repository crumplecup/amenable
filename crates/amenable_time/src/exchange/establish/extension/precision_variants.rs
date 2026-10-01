//! Qualified-value/seasonal/sub-year-grouping/unspecified-component establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`QualifiedTemporalValueProof`](crate::QualifiedTemporalValueProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::QualifiedTemporalValueProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::QualifiedTemporalValueProof"
)]
pub struct QualifiedTemporalValueProofToken(());

/// Lawful token: [`QualifiedTemporalValueFormatted`](crate::QualifiedTemporalValueFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::QualifiedTemporalValueFormatted")]
#[amenable_derive::establish(
    credential = "crate::QualifiedTemporalValueProofToken",
    proposition = "crate::QualifiedTemporalValueFormatted"
)]
pub struct QualifiedTemporalValueFormattedToken(());

/// Lawful token: [`SeasonalTemporalExpressionProof`](crate::SeasonalTemporalExpressionProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SeasonalTemporalExpressionProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::SeasonalTemporalExpressionProof"
)]
pub struct SeasonalTemporalExpressionProofToken(());

/// Lawful token: [`SeasonalTemporalExpressionFormatted`](crate::SeasonalTemporalExpressionFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SeasonalTemporalExpressionFormatted")]
#[amenable_derive::establish(
    credential = "crate::SeasonalTemporalExpressionProofToken",
    proposition = "crate::SeasonalTemporalExpressionFormatted"
)]
pub struct SeasonalTemporalExpressionFormattedToken(());

/// Lawful token: [`SubYearGroupingExpressionProof`](crate::SubYearGroupingExpressionProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SubYearGroupingExpressionProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::SubYearGroupingExpressionProof"
)]
pub struct SubYearGroupingExpressionProofToken(());

/// Lawful token: [`SubYearGroupingExpressionFormatted`](crate::SubYearGroupingExpressionFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SubYearGroupingExpressionFormatted")]
#[amenable_derive::establish(
    credential = "crate::SubYearGroupingExpressionProofToken",
    proposition = "crate::SubYearGroupingExpressionFormatted"
)]
pub struct SubYearGroupingExpressionFormattedToken(());

/// Lawful token: [`UnspecifiedComponentExpressionProof`](crate::UnspecifiedComponentExpressionProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UnspecifiedComponentExpressionProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::UnspecifiedComponentExpressionProof"
)]
pub struct UnspecifiedComponentExpressionProofToken(());

/// Lawful token: [`UnspecifiedComponentExpressionFormatted`](crate::UnspecifiedComponentExpressionFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UnspecifiedComponentExpressionFormatted")]
#[amenable_derive::establish(
    credential = "crate::UnspecifiedComponentExpressionProofToken",
    proposition = "crate::UnspecifiedComponentExpressionFormatted"
)]
pub struct UnspecifiedComponentExpressionFormattedToken(());
