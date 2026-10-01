//! Ordinal-date and week-date alternate-calendar establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`OrdinalDateValid`](crate::OrdinalDateValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrdinalDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::OrdinalDateValid"
)]
pub struct OrdinalDateValidToken(());

/// Lawful token: [`OrdinalDateExtendedFormatted`](crate::OrdinalDateExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrdinalDateExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::OrdinalDateValidToken",
    proposition = "crate::OrdinalDateExtendedFormatted"
)]
pub struct OrdinalDateExtendedFormattedToken(());

/// Lawful token: [`OrdinalDateBasicFormatted`](crate::OrdinalDateBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrdinalDateBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::OrdinalDateValidToken",
    proposition = "crate::OrdinalDateBasicFormatted"
)]
pub struct OrdinalDateBasicFormattedToken(());

/// Lawful token: [`WeekDateValid`](crate::WeekDateValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::WeekDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::WeekDateValid"
)]
pub struct WeekDateValidToken(());

/// Lawful token: [`WeekDateExtendedFormatted`](crate::WeekDateExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::WeekDateExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::WeekDateValidToken",
    proposition = "crate::WeekDateExtendedFormatted"
)]
pub struct WeekDateExtendedFormattedToken(());

/// Lawful token: [`WeekDateBasicFormatted`](crate::WeekDateBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::WeekDateBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::WeekDateValidToken",
    proposition = "crate::WeekDateBasicFormatted"
)]
pub struct WeekDateBasicFormattedToken(());
