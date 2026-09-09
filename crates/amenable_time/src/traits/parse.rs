//! [`TemporalParser`] — the parse half of the temporal seam, ported from
//! `elicit_temporal::traits::TemporalParser`.
//!
//! `TemporalParser<V>`'s supertrait bundle *is* the 24 parse exchanges:
//! a type implements it exactly when it is every `Exchange<RawInput,
//! ParsedX, V>` for verifier `V`. The `where <Prop>: Witness<V>` bounds
//! are the honest precondition — `TemporalParser<KaniVerifier>` only
//! makes sense once Kani has a proof for each parse proposition. A
//! backend writes the inherent parse methods and lets
//! `#[amenable_derive::capture_exchange_body]` generate each
//! `impl<V> Exchange<RawInput, ParsedX, V> for Jiff`; the blanket below
//! then gives it `TemporalParser<V>` for free.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    CalendarDateValid, CenturyValid, DateTimeFormulaProof, DateWithShiftValid, DecadeValid,
    ExtendedYearValid, GroupedTimeScaleUnitProof, IxdtfTimestampProof, LocalDateTimeProof,
    LocalTimeValid, OffsetDateTimeProof, OrdinalDateValid, ParsedCalendarDate, ParsedCentury,
    ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade, ParsedExtendedYear,
    ParsedGroupedTimeScaleUnit, ParsedIxdtfTimestamp, ParsedLocalDateTime, ParsedLocalTime,
    ParsedOffsetDateTime, ParsedOrdinalDate, ParsedQualifiedTemporalValue,
    ParsedReducedCalendarDate, ParsedReducedLocalTime, ParsedRfc3339Timestamp,
    ParsedSeasonalTemporalExpression, ParsedSubYearGroupingExpression, ParsedTemporalSet,
    ParsedTimeInterval, ParsedTimeOfDayWithShift, ParsedUnspecifiedComponentExpression,
    ParsedUtcOffset, ParsedWeekDate, QualifiedTemporalValueProof, RawInput,
    ReducedCalendarDateValid, ReducedLocalTimeValid, Rfc3339TimestampProof,
    SeasonalTemporalExpressionProof, SubYearGroupingExpressionProof, TemporalError,
    TemporalSetProof, TimeIntervalProof, TimeOfDayWithShiftValid,
    UnspecifiedComponentExpressionProof, UtcOffsetValid, WeekDateValid,
};

/// A backend that provides every standards-governed temporal parse as an
/// [`Exchange`], for verifier `V`.
pub trait TemporalParser<V: Verifier>:
    Send
    + Sync
    + Exchange<RawInput, ParsedCalendarDate, V, Error = TemporalError>
    + Exchange<RawInput, ParsedReducedCalendarDate, V, Error = TemporalError>
    + Exchange<RawInput, ParsedExtendedYear, V, Error = TemporalError>
    + Exchange<RawInput, ParsedDecade, V, Error = TemporalError>
    + Exchange<RawInput, ParsedCentury, V, Error = TemporalError>
    + Exchange<RawInput, ParsedQualifiedTemporalValue, V, Error = TemporalError>
    + Exchange<RawInput, ParsedOrdinalDate, V, Error = TemporalError>
    + Exchange<RawInput, ParsedWeekDate, V, Error = TemporalError>
    + Exchange<RawInput, ParsedLocalTime, V, Error = TemporalError>
    + Exchange<RawInput, ParsedReducedLocalTime, V, Error = TemporalError>
    + Exchange<RawInput, ParsedUtcOffset, V, Error = TemporalError>
    + Exchange<RawInput, ParsedLocalDateTime, V, Error = TemporalError>
    + Exchange<RawInput, ParsedOffsetDateTime, V, Error = TemporalError>
    + Exchange<RawInput, ParsedDateWithShift, V, Error = TemporalError>
    + Exchange<RawInput, ParsedTimeOfDayWithShift, V, Error = TemporalError>
    + Exchange<RawInput, ParsedRfc3339Timestamp, V, Error = TemporalError>
    + Exchange<RawInput, ParsedIxdtfTimestamp, V, Error = TemporalError>
    + Exchange<RawInput, ParsedSeasonalTemporalExpression, V, Error = TemporalError>
    + Exchange<RawInput, ParsedSubYearGroupingExpression, V, Error = TemporalError>
    + Exchange<RawInput, ParsedUnspecifiedComponentExpression, V, Error = TemporalError>
    + Exchange<RawInput, ParsedTemporalSet, V, Error = TemporalError>
    + Exchange<RawInput, ParsedGroupedTimeScaleUnit, V, Error = TemporalError>
    + Exchange<RawInput, ParsedDateTimeFormula, V, Error = TemporalError>
    + Exchange<RawInput, ParsedTimeInterval, V, Error = TemporalError>
where
    CalendarDateValid: Witness<V>,
    ReducedCalendarDateValid: Witness<V>,
    ExtendedYearValid: Witness<V>,
    DecadeValid: Witness<V>,
    CenturyValid: Witness<V>,
    QualifiedTemporalValueProof: Witness<V>,
    OrdinalDateValid: Witness<V>,
    WeekDateValid: Witness<V>,
    LocalTimeValid: Witness<V>,
    ReducedLocalTimeValid: Witness<V>,
    UtcOffsetValid: Witness<V>,
    LocalDateTimeProof: Witness<V>,
    OffsetDateTimeProof: Witness<V>,
    DateWithShiftValid: Witness<V>,
    TimeOfDayWithShiftValid: Witness<V>,
    Rfc3339TimestampProof: Witness<V>,
    IxdtfTimestampProof: Witness<V>,
    SeasonalTemporalExpressionProof: Witness<V>,
    SubYearGroupingExpressionProof: Witness<V>,
    UnspecifiedComponentExpressionProof: Witness<V>,
    TemporalSetProof: Witness<V>,
    GroupedTimeScaleUnitProof: Witness<V>,
    DateTimeFormulaProof: Witness<V>,
    TimeIntervalProof: Witness<V>,
{
}

impl<T, V> TemporalParser<V> for T
where
    V: Verifier,
    CalendarDateValid: Witness<V>,
    ReducedCalendarDateValid: Witness<V>,
    ExtendedYearValid: Witness<V>,
    DecadeValid: Witness<V>,
    CenturyValid: Witness<V>,
    QualifiedTemporalValueProof: Witness<V>,
    OrdinalDateValid: Witness<V>,
    WeekDateValid: Witness<V>,
    LocalTimeValid: Witness<V>,
    ReducedLocalTimeValid: Witness<V>,
    UtcOffsetValid: Witness<V>,
    LocalDateTimeProof: Witness<V>,
    OffsetDateTimeProof: Witness<V>,
    DateWithShiftValid: Witness<V>,
    TimeOfDayWithShiftValid: Witness<V>,
    Rfc3339TimestampProof: Witness<V>,
    IxdtfTimestampProof: Witness<V>,
    SeasonalTemporalExpressionProof: Witness<V>,
    SubYearGroupingExpressionProof: Witness<V>,
    UnspecifiedComponentExpressionProof: Witness<V>,
    TemporalSetProof: Witness<V>,
    GroupedTimeScaleUnitProof: Witness<V>,
    DateTimeFormulaProof: Witness<V>,
    TimeIntervalProof: Witness<V>,
    T: Send
        + Sync
        + Exchange<RawInput, ParsedCalendarDate, V, Error = TemporalError>
        + Exchange<RawInput, ParsedReducedCalendarDate, V, Error = TemporalError>
        + Exchange<RawInput, ParsedExtendedYear, V, Error = TemporalError>
        + Exchange<RawInput, ParsedDecade, V, Error = TemporalError>
        + Exchange<RawInput, ParsedCentury, V, Error = TemporalError>
        + Exchange<RawInput, ParsedQualifiedTemporalValue, V, Error = TemporalError>
        + Exchange<RawInput, ParsedOrdinalDate, V, Error = TemporalError>
        + Exchange<RawInput, ParsedWeekDate, V, Error = TemporalError>
        + Exchange<RawInput, ParsedLocalTime, V, Error = TemporalError>
        + Exchange<RawInput, ParsedReducedLocalTime, V, Error = TemporalError>
        + Exchange<RawInput, ParsedUtcOffset, V, Error = TemporalError>
        + Exchange<RawInput, ParsedLocalDateTime, V, Error = TemporalError>
        + Exchange<RawInput, ParsedOffsetDateTime, V, Error = TemporalError>
        + Exchange<RawInput, ParsedDateWithShift, V, Error = TemporalError>
        + Exchange<RawInput, ParsedTimeOfDayWithShift, V, Error = TemporalError>
        + Exchange<RawInput, ParsedRfc3339Timestamp, V, Error = TemporalError>
        + Exchange<RawInput, ParsedIxdtfTimestamp, V, Error = TemporalError>
        + Exchange<RawInput, ParsedSeasonalTemporalExpression, V, Error = TemporalError>
        + Exchange<RawInput, ParsedSubYearGroupingExpression, V, Error = TemporalError>
        + Exchange<RawInput, ParsedUnspecifiedComponentExpression, V, Error = TemporalError>
        + Exchange<RawInput, ParsedTemporalSet, V, Error = TemporalError>
        + Exchange<RawInput, ParsedGroupedTimeScaleUnit, V, Error = TemporalError>
        + Exchange<RawInput, ParsedDateTimeFormula, V, Error = TemporalError>
        + Exchange<RawInput, ParsedTimeInterval, V, Error = TemporalError>,
{
}
