//! RFC 3339 / IXDTF timestamp parse-and-format establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`Rfc3339TimestampProof`](crate::Rfc3339TimestampProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::Rfc3339TimestampProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::Rfc3339TimestampProof"
)]
pub struct Rfc3339TimestampProofToken(());

/// Lawful token: [`Rfc3339TimestampFormatted`](crate::Rfc3339TimestampFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::Rfc3339TimestampFormatted")]
#[amenable_derive::establish(
    credential = "crate::Rfc3339TimestampProofToken",
    proposition = "crate::Rfc3339TimestampFormatted"
)]
pub struct Rfc3339TimestampFormattedToken(());

/// Lawful token: [`IxdtfTimestampProof`](crate::IxdtfTimestampProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfTimestampProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::IxdtfTimestampProof"
)]
pub struct IxdtfTimestampProofToken(());

/// Lawful token: [`IxdtfTimestampFormatted`](crate::IxdtfTimestampFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfTimestampFormatted")]
#[amenable_derive::establish(
    credential = "crate::IxdtfTimestampProofToken",
    proposition = "crate::IxdtfTimestampFormatted"
)]
pub struct IxdtfTimestampFormattedToken(());

/// Lawful token: [`IxdtfZonedTimestampProof`](crate::IxdtfZonedTimestampProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfZonedTimestampProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::IxdtfZonedTimestampProof"
)]
pub struct IxdtfZonedTimestampProofToken(());

/// Lawful token: [`IxdtfZonedTimestampFormatted`](crate::IxdtfZonedTimestampFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfZonedTimestampFormatted")]
#[amenable_derive::establish(
    credential = "crate::IxdtfZonedTimestampProofToken",
    proposition = "crate::IxdtfZonedTimestampFormatted"
)]
pub struct IxdtfZonedTimestampFormattedToken(());
