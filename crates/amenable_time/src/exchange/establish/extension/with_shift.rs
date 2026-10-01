//! Explicit-form date/time-of-day-with-shift establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`DateWithShiftValid`](crate::DateWithShiftValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateWithShiftValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DateWithShiftValid"
)]
pub struct DateWithShiftValidToken(());

/// Lawful token: [`DateWithShiftFormatted`](crate::DateWithShiftFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateWithShiftFormatted")]
#[amenable_derive::establish(
    credential = "crate::DateWithShiftValidToken",
    proposition = "crate::DateWithShiftFormatted"
)]
pub struct DateWithShiftFormattedToken(());

/// Lawful token: [`TimeOfDayWithShiftValid`](crate::TimeOfDayWithShiftValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeOfDayWithShiftValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TimeOfDayWithShiftValid"
)]
pub struct TimeOfDayWithShiftValidToken(());

/// Lawful token: [`TimeOfDayWithShiftFormatted`](crate::TimeOfDayWithShiftFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeOfDayWithShiftFormatted")]
#[amenable_derive::establish(
    credential = "crate::TimeOfDayWithShiftValidToken",
    proposition = "crate::TimeOfDayWithShiftFormatted"
)]
pub struct TimeOfDayWithShiftFormattedToken(());
