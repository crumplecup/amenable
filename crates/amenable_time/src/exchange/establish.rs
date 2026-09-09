//! Output tokens and their [`Establish`](amenable_core::Establish) edges.
//!
//! One private-field unit struct + `#[amenable_derive::establish]` per
//! parse-method proposition (a `proof_composition` `*Valid` for the
//! single-proof methods, a [`super::parse_props`] composite for the
//! multi-proof ones). The verifier-less form generates `impl<V: Verifier>
//! Establish<TemporalInputToken, V> for P where P: Witness<V>` — the
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

/// Lawful token: [`ReducedCalendarDateValid`](crate::ReducedCalendarDateValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedCalendarDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ReducedCalendarDateValid"
)]
pub struct ReducedCalendarDateValidToken(());

/// Lawful token: [`ExtendedYearValid`](crate::ExtendedYearValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExtendedYearValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExtendedYearValid"
)]
pub struct ExtendedYearValidToken(());

/// Lawful token: [`DecadeValid`](crate::DecadeValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DecadeValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DecadeValid"
)]
pub struct DecadeValidToken(());

/// Lawful token: [`CenturyValid`](crate::CenturyValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CenturyValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::CenturyValid"
)]
pub struct CenturyValidToken(());

/// Lawful token: [`QualifiedTemporalValueProof`](crate::QualifiedTemporalValueProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::QualifiedTemporalValueProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::QualifiedTemporalValueProof"
)]
pub struct QualifiedTemporalValueProofToken(());

/// Lawful token: [`OrdinalDateValid`](crate::OrdinalDateValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OrdinalDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::OrdinalDateValid"
)]
pub struct OrdinalDateValidToken(());

/// Lawful token: [`WeekDateValid`](crate::WeekDateValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::WeekDateValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::WeekDateValid"
)]
pub struct WeekDateValidToken(());

/// Lawful token: [`LocalTimeValid`](crate::LocalTimeValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalTimeValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::LocalTimeValid"
)]
pub struct LocalTimeValidToken(());

/// Lawful token: [`ReducedLocalTimeValid`](crate::ReducedLocalTimeValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ReducedLocalTimeValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ReducedLocalTimeValid"
)]
pub struct ReducedLocalTimeValidToken(());

/// Lawful token: [`UtcOffsetValid`](crate::UtcOffsetValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UtcOffsetValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::UtcOffsetValid"
)]
pub struct UtcOffsetValidToken(());

/// Lawful token: [`LocalDateTimeProof`](crate::LocalDateTimeProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalDateTimeProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::LocalDateTimeProof"
)]
pub struct LocalDateTimeProofToken(());

/// Lawful token: [`OffsetDateTimeProof`](crate::OffsetDateTimeProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OffsetDateTimeProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::OffsetDateTimeProof"
)]
pub struct OffsetDateTimeProofToken(());

/// Lawful token: [`DateWithShiftValid`](crate::DateWithShiftValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateWithShiftValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DateWithShiftValid"
)]
pub struct DateWithShiftValidToken(());

/// Lawful token: [`TimeOfDayWithShiftValid`](crate::TimeOfDayWithShiftValid) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeOfDayWithShiftValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TimeOfDayWithShiftValid"
)]
pub struct TimeOfDayWithShiftValidToken(());

/// Lawful token: [`Rfc3339TimestampProof`](crate::Rfc3339TimestampProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::Rfc3339TimestampProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::Rfc3339TimestampProof"
)]
pub struct Rfc3339TimestampProofToken(());

/// Lawful token: [`IxdtfTimestampProof`](crate::IxdtfTimestampProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfTimestampProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::IxdtfTimestampProof"
)]
pub struct IxdtfTimestampProofToken(());

/// Lawful token: [`SeasonalTemporalExpressionProof`](crate::SeasonalTemporalExpressionProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SeasonalTemporalExpressionProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::SeasonalTemporalExpressionProof"
)]
pub struct SeasonalTemporalExpressionProofToken(());

/// Lawful token: [`SubYearGroupingExpressionProof`](crate::SubYearGroupingExpressionProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SubYearGroupingExpressionProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::SubYearGroupingExpressionProof"
)]
pub struct SubYearGroupingExpressionProofToken(());

/// Lawful token: [`UnspecifiedComponentExpressionProof`](crate::UnspecifiedComponentExpressionProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UnspecifiedComponentExpressionProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::UnspecifiedComponentExpressionProof"
)]
pub struct UnspecifiedComponentExpressionProofToken(());

/// Lawful token: [`TemporalSetProof`](crate::TemporalSetProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TemporalSetProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TemporalSetProof"
)]
pub struct TemporalSetProofToken(());

/// Lawful token: [`GroupedTimeScaleUnitProof`](crate::GroupedTimeScaleUnitProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::GroupedTimeScaleUnitProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::GroupedTimeScaleUnitProof"
)]
pub struct GroupedTimeScaleUnitProofToken(());

/// Lawful token: [`DateTimeFormulaProof`](crate::DateTimeFormulaProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateTimeFormulaProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DateTimeFormulaProof"
)]
pub struct DateTimeFormulaProofToken(());

/// Lawful token: [`TimeIntervalProof`](crate::TimeIntervalProof) was established from a
/// received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeIntervalProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TimeIntervalProof"
)]
pub struct TimeIntervalProofToken(());
