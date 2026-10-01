//! Extended-year/decade/century extended-precision establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`ExtendedYearValid`](crate::ExtendedYearValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExtendedYearValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExtendedYearValid"
)]
pub struct ExtendedYearValidToken(());

/// Lawful token: [`ExtendedYearFormatted`](crate::ExtendedYearFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExtendedYearFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExtendedYearValidToken",
    proposition = "crate::ExtendedYearFormatted"
)]
pub struct ExtendedYearFormattedToken(());

/// Lawful token: [`DecadeValid`](crate::DecadeValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DecadeValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DecadeValid"
)]
pub struct DecadeValidToken(());

/// Lawful token: [`DecadeFormatted`](crate::DecadeFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DecadeFormatted")]
#[amenable_derive::establish(
    credential = "crate::DecadeValidToken",
    proposition = "crate::DecadeFormatted"
)]
pub struct DecadeFormattedToken(());

/// Lawful token: [`CenturyValid`](crate::CenturyValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CenturyValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::CenturyValid"
)]
pub struct CenturyValidToken(());

/// Lawful token: [`CenturyFormatted`](crate::CenturyFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CenturyFormatted")]
#[amenable_derive::establish(
    credential = "crate::CenturyValidToken",
    proposition = "crate::CenturyFormatted"
)]
pub struct CenturyFormattedToken(());
