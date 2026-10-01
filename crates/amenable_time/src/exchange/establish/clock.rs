//! Local-time/UTC-offset parse-and-format establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`LocalTimeValid`](crate::LocalTimeValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalTimeValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::LocalTimeValid"
)]
pub struct LocalTimeValidToken(());

/// Lawful token: [`LocalTimeExtendedFormatted`](crate::LocalTimeExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalTimeExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::LocalTimeValidToken",
    proposition = "crate::LocalTimeExtendedFormatted"
)]
pub struct LocalTimeExtendedFormattedToken(());

/// Lawful token: [`LocalTimeBasicFormatted`](crate::LocalTimeBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalTimeBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::LocalTimeValidToken",
    proposition = "crate::LocalTimeBasicFormatted"
)]
pub struct LocalTimeBasicFormattedToken(());

/// Lawful token: [`ReducedLocalTimeValid`](crate::ReducedLocalTimeValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedLocalTimeValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ReducedLocalTimeValid"
)]
pub struct ReducedLocalTimeValidToken(());

/// Lawful token: [`ReducedLocalTimeExtendedFormatted`](crate::ReducedLocalTimeExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedLocalTimeExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::ReducedLocalTimeValidToken",
    proposition = "crate::ReducedLocalTimeExtendedFormatted"
)]
pub struct ReducedLocalTimeExtendedFormattedToken(());

/// Lawful token: [`ReducedLocalTimeBasicFormatted`](crate::ReducedLocalTimeBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedLocalTimeBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::ReducedLocalTimeValidToken",
    proposition = "crate::ReducedLocalTimeBasicFormatted"
)]
pub struct ReducedLocalTimeBasicFormattedToken(());

/// Lawful token: [`UtcOffsetValid`](crate::UtcOffsetValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UtcOffsetValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::UtcOffsetValid"
)]
pub struct UtcOffsetValidToken(());

/// Lawful token: [`UtcOffsetExtendedFormatted`](crate::UtcOffsetExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UtcOffsetExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::UtcOffsetValidToken",
    proposition = "crate::UtcOffsetExtendedFormatted"
)]
pub struct UtcOffsetExtendedFormattedToken(());

/// Lawful token: [`UtcOffsetBasicFormatted`](crate::UtcOffsetBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UtcOffsetBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::UtcOffsetValidToken",
    proposition = "crate::UtcOffsetBasicFormatted"
)]
pub struct UtcOffsetBasicFormattedToken(());
