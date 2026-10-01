//! Full and reduced-precision Gregorian calendar-date establish tokens.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` -- the
//! `amenable_gaap::tokens` pattern. A backend's `Exchange` body mints the
//! output token via `<P as Establish<TemporalInputToken, V>>::establish(
//! input.sidecar())`.

/// Lawful token: [`CalendarDateValid`](crate::CalendarDateValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CalendarDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::CalendarDateValid"
)]
pub struct CalendarDateValidToken(());

/// Lawful token: [`CalendarDateExtendedFormatted`](crate::CalendarDateExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CalendarDateExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::CalendarDateValidToken",
    proposition = "crate::CalendarDateExtendedFormatted"
)]
pub struct CalendarDateExtendedFormattedToken(());

/// Lawful token: [`CalendarDateBasicFormatted`](crate::CalendarDateBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CalendarDateBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::CalendarDateValidToken",
    proposition = "crate::CalendarDateBasicFormatted"
)]
pub struct CalendarDateBasicFormattedToken(());

/// Lawful token: [`ReducedCalendarDateValid`](crate::ReducedCalendarDateValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedCalendarDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ReducedCalendarDateValid"
)]
pub struct ReducedCalendarDateValidToken(());

/// Lawful token: [`ReducedCalendarDateExtendedFormatted`](crate::ReducedCalendarDateExtendedFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedCalendarDateExtendedFormatted")]
#[amenable_derive::establish(
    credential = "crate::ReducedCalendarDateValidToken",
    proposition = "crate::ReducedCalendarDateExtendedFormatted"
)]
pub struct ReducedCalendarDateExtendedFormattedToken(());

/// Lawful token: [`ReducedCalendarDateBasicFormatted`](crate::ReducedCalendarDateBasicFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedCalendarDateBasicFormatted")]
#[amenable_derive::establish(
    credential = "crate::ReducedCalendarDateValidToken",
    proposition = "crate::ReducedCalendarDateBasicFormatted"
)]
pub struct ReducedCalendarDateBasicFormattedToken(());
