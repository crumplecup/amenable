//! Duration/interval parse-and-format establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`DurationFormValid`](crate::DurationFormValid) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DurationFormValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DurationFormValid"
)]
pub struct DurationFormValidToken(());

/// Lawful token: [`DurationFormatted`](crate::DurationFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DurationFormatted")]
#[amenable_derive::establish(
    credential = "crate::DurationFormValidToken",
    proposition = "crate::DurationFormatted"
)]
pub struct DurationFormattedToken(());

/// Lawful token: [`RecurringIntervalFormValid`](crate::RecurringIntervalFormValid) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RecurringIntervalFormValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::RecurringIntervalFormValid"
)]
pub struct RecurringIntervalFormValidToken(());

/// Lawful token: [`RecurringIntervalFormatted`](crate::RecurringIntervalFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RecurringIntervalFormatted")]
#[amenable_derive::establish(
    credential = "crate::RecurringIntervalFormValidToken",
    proposition = "crate::RecurringIntervalFormatted"
)]
pub struct RecurringIntervalFormattedToken(());

/// Lawful token: [`TimeIntervalProof`](crate::TimeIntervalProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeIntervalProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TimeIntervalProof"
)]
pub struct TimeIntervalProofToken(());

/// Lawful token: [`TimeIntervalFormatted`](crate::TimeIntervalFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeIntervalFormatted")]
#[amenable_derive::establish(
    credential = "crate::TimeIntervalProofToken",
    proposition = "crate::TimeIntervalFormatted"
)]
pub struct TimeIntervalFormattedToken(());
