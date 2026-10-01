//! Local/offset date-time parse-and-format establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`LocalDateTimeProof`](crate::LocalDateTimeProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalDateTimeProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::LocalDateTimeProof"
)]
pub struct LocalDateTimeProofToken(());

/// Lawful token: [`LocalDateTimeExtendedFormatted`](crate::LocalDateTimeExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalDateTimeExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::LocalDateTimeProofToken",
    proposition = "crate::LocalDateTimeExtendedFormatted"
)]
pub struct LocalDateTimeExtendedFormattedToken(());

/// Lawful token: [`LocalDateTimeBasicFormatted`](crate::LocalDateTimeBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalDateTimeBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::LocalDateTimeProofToken",
    proposition = "crate::LocalDateTimeBasicFormatted"
)]
pub struct LocalDateTimeBasicFormattedToken(());

/// Lawful token: [`OffsetDateTimeProof`](crate::OffsetDateTimeProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OffsetDateTimeProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::OffsetDateTimeProof"
)]
pub struct OffsetDateTimeProofToken(());

/// Lawful token: [`OffsetDateTimeExtendedFormatted`](crate::OffsetDateTimeExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OffsetDateTimeExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::OffsetDateTimeProofToken",
    proposition = "crate::OffsetDateTimeExtendedFormatted"
)]
pub struct OffsetDateTimeExtendedFormattedToken(());

/// Lawful token: [`OffsetDateTimeBasicFormatted`](crate::OffsetDateTimeBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OffsetDateTimeBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::OffsetDateTimeProofToken",
    proposition = "crate::OffsetDateTimeBasicFormatted"
)]
pub struct OffsetDateTimeBasicFormattedToken(());
