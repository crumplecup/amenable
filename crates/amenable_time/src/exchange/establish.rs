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

/// Lawful token: [`IxdtfZonedTimestampProof`](crate::IxdtfZonedTimestampProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfZonedTimestampProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::IxdtfZonedTimestampProof"
)]
pub struct IxdtfZonedTimestampProofToken(());

/// Lawful token: [`DurationFormValid`](crate::DurationFormValid) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DurationFormValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::DurationFormValid"
)]
pub struct DurationFormValidToken(());

/// Lawful token: [`RecurringIntervalFormValid`](crate::RecurringIntervalFormValid) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RecurringIntervalFormValid")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::RecurringIntervalFormValid"
)]
pub struct RecurringIntervalFormValidToken(());

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

/// Lawful token: [`DateWithShiftFormatted`](crate::DateWithShiftFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateWithShiftFormatted")]
#[amenable_derive::establish(
    credential = "crate::DateWithShiftValidToken",
    proposition = "crate::DateWithShiftFormatted"
)]
pub struct DateWithShiftFormattedToken(());

/// Lawful token: [`TimeOfDayWithShiftFormatted`](crate::TimeOfDayWithShiftFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeOfDayWithShiftFormatted")]
#[amenable_derive::establish(
    credential = "crate::TimeOfDayWithShiftValidToken",
    proposition = "crate::TimeOfDayWithShiftFormatted"
)]
pub struct TimeOfDayWithShiftFormattedToken(());

/// Lawful token: [`ExtendedYearFormatted`](crate::ExtendedYearFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExtendedYearFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExtendedYearValidToken",
    proposition = "crate::ExtendedYearFormatted"
)]
pub struct ExtendedYearFormattedToken(());

/// Lawful token: [`DecadeFormatted`](crate::DecadeFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DecadeFormatted")]
#[amenable_derive::establish(
    credential = "crate::DecadeValidToken",
    proposition = "crate::DecadeFormatted"
)]
pub struct DecadeFormattedToken(());

/// Lawful token: [`CenturyFormatted`](crate::CenturyFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::CenturyFormatted")]
#[amenable_derive::establish(
    credential = "crate::CenturyValidToken",
    proposition = "crate::CenturyFormatted"
)]
pub struct CenturyFormattedToken(());

/// Lawful token: [`QualifiedTemporalValueFormatted`](crate::QualifiedTemporalValueFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::QualifiedTemporalValueFormatted")]
#[amenable_derive::establish(
    credential = "crate::QualifiedTemporalValueProofToken",
    proposition = "crate::QualifiedTemporalValueFormatted"
)]
pub struct QualifiedTemporalValueFormattedToken(());

/// Lawful token: [`Rfc3339TimestampFormatted`](crate::Rfc3339TimestampFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::Rfc3339TimestampFormatted")]
#[amenable_derive::establish(
    credential = "crate::Rfc3339TimestampProofToken",
    proposition = "crate::Rfc3339TimestampFormatted"
)]
pub struct Rfc3339TimestampFormattedToken(());

/// Lawful token: [`IxdtfZonedTimestampFormatted`](crate::IxdtfZonedTimestampFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfZonedTimestampFormatted")]
#[amenable_derive::establish(
    credential = "crate::IxdtfZonedTimestampProofToken",
    proposition = "crate::IxdtfZonedTimestampFormatted"
)]
pub struct IxdtfZonedTimestampFormattedToken(());

/// Lawful token: [`IxdtfTimestampFormatted`](crate::IxdtfTimestampFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IxdtfTimestampFormatted")]
#[amenable_derive::establish(
    credential = "crate::IxdtfTimestampProofToken",
    proposition = "crate::IxdtfTimestampFormatted"
)]
pub struct IxdtfTimestampFormattedToken(());

/// Lawful token: [`SeasonalTemporalExpressionFormatted`](crate::SeasonalTemporalExpressionFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SeasonalTemporalExpressionFormatted")]
#[amenable_derive::establish(
    credential = "crate::SeasonalTemporalExpressionProofToken",
    proposition = "crate::SeasonalTemporalExpressionFormatted"
)]
pub struct SeasonalTemporalExpressionFormattedToken(());

/// Lawful token: [`SubYearGroupingExpressionFormatted`](crate::SubYearGroupingExpressionFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SubYearGroupingExpressionFormatted")]
#[amenable_derive::establish(
    credential = "crate::SubYearGroupingExpressionProofToken",
    proposition = "crate::SubYearGroupingExpressionFormatted"
)]
pub struct SubYearGroupingExpressionFormattedToken(());

/// Lawful token: [`UnspecifiedComponentExpressionFormatted`](crate::UnspecifiedComponentExpressionFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::UnspecifiedComponentExpressionFormatted")]
#[amenable_derive::establish(
    credential = "crate::UnspecifiedComponentExpressionProofToken",
    proposition = "crate::UnspecifiedComponentExpressionFormatted"
)]
pub struct UnspecifiedComponentExpressionFormattedToken(());

/// Lawful token: [`TemporalSetFormatted`](crate::TemporalSetFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TemporalSetFormatted")]
#[amenable_derive::establish(
    credential = "crate::TemporalSetProofToken",
    proposition = "crate::TemporalSetFormatted"
)]
pub struct TemporalSetFormattedToken(());

/// Lawful token: [`GroupedTimeScaleUnitFormatted`](crate::GroupedTimeScaleUnitFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::GroupedTimeScaleUnitFormatted")]
#[amenable_derive::establish(
    credential = "crate::GroupedTimeScaleUnitProofToken",
    proposition = "crate::GroupedTimeScaleUnitFormatted"
)]
pub struct GroupedTimeScaleUnitFormattedToken(());

/// Lawful token: [`DateTimeFormulaFormatted`](crate::DateTimeFormulaFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateTimeFormulaFormatted")]
#[amenable_derive::establish(
    credential = "crate::DateTimeFormulaProofToken",
    proposition = "crate::DateTimeFormulaFormatted"
)]
pub struct DateTimeFormulaFormattedToken(());

/// Lawful token: [`DurationFormatted`](crate::DurationFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DurationFormatted")]
#[amenable_derive::establish(
    credential = "crate::DurationFormValidToken",
    proposition = "crate::DurationFormatted"
)]
pub struct DurationFormattedToken(());

/// Lawful token: [`RecurringIntervalFormatted`](crate::RecurringIntervalFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RecurringIntervalFormatted")]
#[amenable_derive::establish(
    credential = "crate::RecurringIntervalFormValidToken",
    proposition = "crate::RecurringIntervalFormatted"
)]
pub struct RecurringIntervalFormattedToken(());

/// Lawful token: [`TimeIntervalFormatted`](crate::TimeIntervalFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeIntervalFormatted")]
#[amenable_derive::establish(
    credential = "crate::TimeIntervalProofToken",
    proposition = "crate::TimeIntervalFormatted"
)]
pub struct TimeIntervalFormattedToken(());
