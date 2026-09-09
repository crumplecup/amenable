//! [`TemporalFormatter`] — the emit half of the temporal seam, ported
//! from `elicit_temporal::traits::TemporalFormatter`. Same shape as
//! [`TemporalParser`](crate::TemporalParser): `TemporalFormatter<V>`'s
//! supertrait bundle *is* the 36 emit exchanges. The `where <Prop>:
//! Witness<V>` bounds — one per distinct input *and* output proposition
//! — are the honest precondition, backend-provided.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    CalendarDateBasicFormatted, CalendarDateExtendedFormatted, CalendarDateValid, CenturyFormatted,
    CenturyValid, DateTimeFormulaFormatted, DateTimeFormulaProof, DateWithShiftFormatted,
    DateWithShiftValid, DecadeFormatted, DecadeValid, DurationFormValid, DurationFormatted,
    ExtendedYearFormatted, ExtendedYearValid, FormattedCalendarDateBasic,
    FormattedCalendarDateExtended, FormattedCentury, FormattedDateTimeFormula,
    FormattedDateWithShift, FormattedDecade, FormattedDuration, FormattedExtendedYear,
    FormattedGroupedTimeScaleUnit, FormattedIxdtfTimestamp, FormattedIxdtfZonedTimestamp,
    FormattedLocalDateTimeBasic, FormattedLocalDateTimeExtended, FormattedLocalTimeBasic,
    FormattedLocalTimeExtended, FormattedOffsetDateTimeBasic, FormattedOffsetDateTimeExtended,
    FormattedOrdinalDateBasic, FormattedOrdinalDateExtended, FormattedQualifiedTemporalValue,
    FormattedRecurringInterval, FormattedReducedCalendarDateBasic,
    FormattedReducedCalendarDateExtended, FormattedReducedLocalTimeBasic,
    FormattedReducedLocalTimeExtended, FormattedRfc3339Timestamp,
    FormattedSeasonalTemporalExpression, FormattedSubYearGroupingExpression, FormattedTemporalSet,
    FormattedTimeInterval, FormattedTimeOfDayWithShift, FormattedUnspecifiedComponentExpression,
    FormattedUtcOffsetBasic, FormattedUtcOffsetExtended, FormattedWeekDateBasic,
    FormattedWeekDateExtended, GroupedTimeScaleUnitFormatted, GroupedTimeScaleUnitProof,
    IxdtfTimestampFormatted, IxdtfTimestampProof, IxdtfZonedTimestampFormatted,
    IxdtfZonedTimestampProof, LocalDateTimeBasicFormatted, LocalDateTimeExtendedFormatted,
    LocalDateTimeProof, LocalTimeBasicFormatted, LocalTimeExtendedFormatted, LocalTimeValid,
    OffsetDateTimeBasicFormatted, OffsetDateTimeExtendedFormatted, OffsetDateTimeProof,
    OrdinalDateBasicFormatted, OrdinalDateExtendedFormatted, OrdinalDateValid, ParsedCalendarDate,
    ParsedCentury, ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade, ParsedDuration,
    ParsedExtendedYear, ParsedGroupedTimeScaleUnit, ParsedIxdtfTimestamp,
    ParsedIxdtfZonedTimestamp, ParsedLocalDateTime, ParsedLocalTime, ParsedOffsetDateTime,
    ParsedOrdinalDate, ParsedQualifiedTemporalValue, ParsedRecurringInterval,
    ParsedReducedCalendarDate, ParsedReducedLocalTime, ParsedRfc3339Timestamp,
    ParsedSeasonalTemporalExpression, ParsedSubYearGroupingExpression, ParsedTemporalSet,
    ParsedTimeInterval, ParsedTimeOfDayWithShift, ParsedUnspecifiedComponentExpression,
    ParsedUtcOffset, ParsedWeekDate, QualifiedTemporalValueFormatted, QualifiedTemporalValueProof,
    RecurringIntervalFormValid, RecurringIntervalFormatted, ReducedCalendarDateBasicFormatted,
    ReducedCalendarDateExtendedFormatted, ReducedCalendarDateValid, ReducedLocalTimeBasicFormatted,
    ReducedLocalTimeExtendedFormatted, ReducedLocalTimeValid, Rfc3339TimestampFormatted,
    Rfc3339TimestampProof, SeasonalTemporalExpressionFormatted, SeasonalTemporalExpressionProof,
    SubYearGroupingExpressionFormatted, SubYearGroupingExpressionProof, TemporalError,
    TemporalInputReceived, TemporalSetFormatted, TemporalSetProof, TimeIntervalFormatted,
    TimeIntervalProof, TimeOfDayWithShiftFormatted, TimeOfDayWithShiftValid,
    UnspecifiedComponentExpressionFormatted, UnspecifiedComponentExpressionProof,
    UtcOffsetBasicFormatted, UtcOffsetExtendedFormatted, UtcOffsetValid, WeekDateBasicFormatted,
    WeekDateExtendedFormatted, WeekDateValid,
};

/// A backend that emits every standards-governed temporal form as an
/// [`Exchange`], for verifier `V`.
pub trait TemporalFormatter<V: Verifier>: Send
    + Sync
    + Exchange<ParsedCalendarDate, FormattedCalendarDateExtended, V, Error = TemporalError>
    + Exchange<ParsedCalendarDate, FormattedCalendarDateBasic, V, Error = TemporalError>
    + Exchange<
        ParsedReducedCalendarDate,
        FormattedReducedCalendarDateExtended,
        V,
        Error = TemporalError,
    > + Exchange<ParsedReducedCalendarDate, FormattedReducedCalendarDateBasic, V, Error = TemporalError>
    + Exchange<ParsedOrdinalDate, FormattedOrdinalDateExtended, V, Error = TemporalError>
    + Exchange<ParsedOrdinalDate, FormattedOrdinalDateBasic, V, Error = TemporalError>
    + Exchange<ParsedWeekDate, FormattedWeekDateExtended, V, Error = TemporalError>
    + Exchange<ParsedWeekDate, FormattedWeekDateBasic, V, Error = TemporalError>
    + Exchange<ParsedLocalTime, FormattedLocalTimeExtended, V, Error = TemporalError>
    + Exchange<ParsedLocalTime, FormattedLocalTimeBasic, V, Error = TemporalError>
    + Exchange<ParsedReducedLocalTime, FormattedReducedLocalTimeExtended, V, Error = TemporalError>
    + Exchange<ParsedReducedLocalTime, FormattedReducedLocalTimeBasic, V, Error = TemporalError>
    + Exchange<ParsedUtcOffset, FormattedUtcOffsetExtended, V, Error = TemporalError>
    + Exchange<ParsedUtcOffset, FormattedUtcOffsetBasic, V, Error = TemporalError>
    + Exchange<ParsedLocalDateTime, FormattedLocalDateTimeExtended, V, Error = TemporalError>
    + Exchange<ParsedLocalDateTime, FormattedLocalDateTimeBasic, V, Error = TemporalError>
    + Exchange<ParsedOffsetDateTime, FormattedOffsetDateTimeExtended, V, Error = TemporalError>
    + Exchange<ParsedOffsetDateTime, FormattedOffsetDateTimeBasic, V, Error = TemporalError>
    + Exchange<ParsedDateWithShift, FormattedDateWithShift, V, Error = TemporalError>
    + Exchange<ParsedTimeOfDayWithShift, FormattedTimeOfDayWithShift, V, Error = TemporalError>
    + Exchange<ParsedExtendedYear, FormattedExtendedYear, V, Error = TemporalError>
    + Exchange<ParsedDecade, FormattedDecade, V, Error = TemporalError>
    + Exchange<ParsedCentury, FormattedCentury, V, Error = TemporalError>
    + Exchange<
        ParsedQualifiedTemporalValue,
        FormattedQualifiedTemporalValue,
        V,
        Error = TemporalError,
    > + Exchange<ParsedRfc3339Timestamp, FormattedRfc3339Timestamp, V, Error = TemporalError>
    + Exchange<ParsedIxdtfZonedTimestamp, FormattedIxdtfZonedTimestamp, V, Error = TemporalError>
    + Exchange<ParsedIxdtfTimestamp, FormattedIxdtfTimestamp, V, Error = TemporalError>
    + Exchange<
        ParsedSeasonalTemporalExpression,
        FormattedSeasonalTemporalExpression,
        V,
        Error = TemporalError,
    > + Exchange<
        ParsedSubYearGroupingExpression,
        FormattedSubYearGroupingExpression,
        V,
        Error = TemporalError,
    > + Exchange<
        ParsedUnspecifiedComponentExpression,
        FormattedUnspecifiedComponentExpression,
        V,
        Error = TemporalError,
    > + Exchange<ParsedTemporalSet, FormattedTemporalSet, V, Error = TemporalError>
    + Exchange<ParsedGroupedTimeScaleUnit, FormattedGroupedTimeScaleUnit, V, Error = TemporalError>
    + Exchange<ParsedDateTimeFormula, FormattedDateTimeFormula, V, Error = TemporalError>
    + Exchange<ParsedDuration, FormattedDuration, V, Error = TemporalError>
    + Exchange<ParsedRecurringInterval, FormattedRecurringInterval, V, Error = TemporalError>
    + Exchange<ParsedTimeInterval, FormattedTimeInterval, V, Error = TemporalError>
where
    TemporalInputReceived: Witness<V>,
    CalendarDateValid: Witness<V>,
    CenturyValid: Witness<V>,
    DateTimeFormulaProof: Witness<V>,
    DateWithShiftValid: Witness<V>,
    DecadeValid: Witness<V>,
    DurationFormValid: Witness<V>,
    ExtendedYearValid: Witness<V>,
    GroupedTimeScaleUnitProof: Witness<V>,
    IxdtfTimestampProof: Witness<V>,
    IxdtfZonedTimestampProof: Witness<V>,
    LocalDateTimeProof: Witness<V>,
    LocalTimeValid: Witness<V>,
    OffsetDateTimeProof: Witness<V>,
    OrdinalDateValid: Witness<V>,
    QualifiedTemporalValueProof: Witness<V>,
    RecurringIntervalFormValid: Witness<V>,
    ReducedCalendarDateValid: Witness<V>,
    ReducedLocalTimeValid: Witness<V>,
    Rfc3339TimestampProof: Witness<V>,
    SeasonalTemporalExpressionProof: Witness<V>,
    SubYearGroupingExpressionProof: Witness<V>,
    TemporalSetProof: Witness<V>,
    TimeIntervalProof: Witness<V>,
    TimeOfDayWithShiftValid: Witness<V>,
    UnspecifiedComponentExpressionProof: Witness<V>,
    UtcOffsetValid: Witness<V>,
    WeekDateValid: Witness<V>,
    CalendarDateBasicFormatted: Witness<V>,
    CalendarDateExtendedFormatted: Witness<V>,
    CenturyFormatted: Witness<V>,
    DateTimeFormulaFormatted: Witness<V>,
    DateWithShiftFormatted: Witness<V>,
    DecadeFormatted: Witness<V>,
    DurationFormatted: Witness<V>,
    ExtendedYearFormatted: Witness<V>,
    GroupedTimeScaleUnitFormatted: Witness<V>,
    IxdtfTimestampFormatted: Witness<V>,
    IxdtfZonedTimestampFormatted: Witness<V>,
    LocalDateTimeBasicFormatted: Witness<V>,
    LocalDateTimeExtendedFormatted: Witness<V>,
    LocalTimeBasicFormatted: Witness<V>,
    LocalTimeExtendedFormatted: Witness<V>,
    OffsetDateTimeBasicFormatted: Witness<V>,
    OffsetDateTimeExtendedFormatted: Witness<V>,
    OrdinalDateBasicFormatted: Witness<V>,
    OrdinalDateExtendedFormatted: Witness<V>,
    QualifiedTemporalValueFormatted: Witness<V>,
    RecurringIntervalFormatted: Witness<V>,
    ReducedCalendarDateBasicFormatted: Witness<V>,
    ReducedCalendarDateExtendedFormatted: Witness<V>,
    ReducedLocalTimeBasicFormatted: Witness<V>,
    ReducedLocalTimeExtendedFormatted: Witness<V>,
    Rfc3339TimestampFormatted: Witness<V>,
    SeasonalTemporalExpressionFormatted: Witness<V>,
    SubYearGroupingExpressionFormatted: Witness<V>,
    TemporalSetFormatted: Witness<V>,
    TimeIntervalFormatted: Witness<V>,
    TimeOfDayWithShiftFormatted: Witness<V>,
    UnspecifiedComponentExpressionFormatted: Witness<V>,
    UtcOffsetBasicFormatted: Witness<V>,
    UtcOffsetExtendedFormatted: Witness<V>,
    WeekDateBasicFormatted: Witness<V>,
    WeekDateExtendedFormatted: Witness<V>,
{
}

impl<T, V> TemporalFormatter<V> for T
where
    V: Verifier,
    TemporalInputReceived: Witness<V>,
    CalendarDateValid: Witness<V>,
    CenturyValid: Witness<V>,
    DateTimeFormulaProof: Witness<V>,
    DateWithShiftValid: Witness<V>,
    DecadeValid: Witness<V>,
    DurationFormValid: Witness<V>,
    ExtendedYearValid: Witness<V>,
    GroupedTimeScaleUnitProof: Witness<V>,
    IxdtfTimestampProof: Witness<V>,
    IxdtfZonedTimestampProof: Witness<V>,
    LocalDateTimeProof: Witness<V>,
    LocalTimeValid: Witness<V>,
    OffsetDateTimeProof: Witness<V>,
    OrdinalDateValid: Witness<V>,
    QualifiedTemporalValueProof: Witness<V>,
    RecurringIntervalFormValid: Witness<V>,
    ReducedCalendarDateValid: Witness<V>,
    ReducedLocalTimeValid: Witness<V>,
    Rfc3339TimestampProof: Witness<V>,
    SeasonalTemporalExpressionProof: Witness<V>,
    SubYearGroupingExpressionProof: Witness<V>,
    TemporalSetProof: Witness<V>,
    TimeIntervalProof: Witness<V>,
    TimeOfDayWithShiftValid: Witness<V>,
    UnspecifiedComponentExpressionProof: Witness<V>,
    UtcOffsetValid: Witness<V>,
    WeekDateValid: Witness<V>,
    CalendarDateBasicFormatted: Witness<V>,
    CalendarDateExtendedFormatted: Witness<V>,
    CenturyFormatted: Witness<V>,
    DateTimeFormulaFormatted: Witness<V>,
    DateWithShiftFormatted: Witness<V>,
    DecadeFormatted: Witness<V>,
    DurationFormatted: Witness<V>,
    ExtendedYearFormatted: Witness<V>,
    GroupedTimeScaleUnitFormatted: Witness<V>,
    IxdtfTimestampFormatted: Witness<V>,
    IxdtfZonedTimestampFormatted: Witness<V>,
    LocalDateTimeBasicFormatted: Witness<V>,
    LocalDateTimeExtendedFormatted: Witness<V>,
    LocalTimeBasicFormatted: Witness<V>,
    LocalTimeExtendedFormatted: Witness<V>,
    OffsetDateTimeBasicFormatted: Witness<V>,
    OffsetDateTimeExtendedFormatted: Witness<V>,
    OrdinalDateBasicFormatted: Witness<V>,
    OrdinalDateExtendedFormatted: Witness<V>,
    QualifiedTemporalValueFormatted: Witness<V>,
    RecurringIntervalFormatted: Witness<V>,
    ReducedCalendarDateBasicFormatted: Witness<V>,
    ReducedCalendarDateExtendedFormatted: Witness<V>,
    ReducedLocalTimeBasicFormatted: Witness<V>,
    ReducedLocalTimeExtendedFormatted: Witness<V>,
    Rfc3339TimestampFormatted: Witness<V>,
    SeasonalTemporalExpressionFormatted: Witness<V>,
    SubYearGroupingExpressionFormatted: Witness<V>,
    TemporalSetFormatted: Witness<V>,
    TimeIntervalFormatted: Witness<V>,
    TimeOfDayWithShiftFormatted: Witness<V>,
    UnspecifiedComponentExpressionFormatted: Witness<V>,
    UtcOffsetBasicFormatted: Witness<V>,
    UtcOffsetExtendedFormatted: Witness<V>,
    WeekDateBasicFormatted: Witness<V>,
    WeekDateExtendedFormatted: Witness<V>,
    T: Send
        + Sync
        + Exchange<ParsedCalendarDate, FormattedCalendarDateExtended, V, Error = TemporalError>
        + Exchange<ParsedCalendarDate, FormattedCalendarDateBasic, V, Error = TemporalError>
        + Exchange<
            ParsedReducedCalendarDate,
            FormattedReducedCalendarDateExtended,
            V,
            Error = TemporalError,
        > + Exchange<
            ParsedReducedCalendarDate,
            FormattedReducedCalendarDateBasic,
            V,
            Error = TemporalError,
        > + Exchange<ParsedOrdinalDate, FormattedOrdinalDateExtended, V, Error = TemporalError>
        + Exchange<ParsedOrdinalDate, FormattedOrdinalDateBasic, V, Error = TemporalError>
        + Exchange<ParsedWeekDate, FormattedWeekDateExtended, V, Error = TemporalError>
        + Exchange<ParsedWeekDate, FormattedWeekDateBasic, V, Error = TemporalError>
        + Exchange<ParsedLocalTime, FormattedLocalTimeExtended, V, Error = TemporalError>
        + Exchange<ParsedLocalTime, FormattedLocalTimeBasic, V, Error = TemporalError>
        + Exchange<
            ParsedReducedLocalTime,
            FormattedReducedLocalTimeExtended,
            V,
            Error = TemporalError,
        > + Exchange<ParsedReducedLocalTime, FormattedReducedLocalTimeBasic, V, Error = TemporalError>
        + Exchange<ParsedUtcOffset, FormattedUtcOffsetExtended, V, Error = TemporalError>
        + Exchange<ParsedUtcOffset, FormattedUtcOffsetBasic, V, Error = TemporalError>
        + Exchange<ParsedLocalDateTime, FormattedLocalDateTimeExtended, V, Error = TemporalError>
        + Exchange<ParsedLocalDateTime, FormattedLocalDateTimeBasic, V, Error = TemporalError>
        + Exchange<ParsedOffsetDateTime, FormattedOffsetDateTimeExtended, V, Error = TemporalError>
        + Exchange<ParsedOffsetDateTime, FormattedOffsetDateTimeBasic, V, Error = TemporalError>
        + Exchange<ParsedDateWithShift, FormattedDateWithShift, V, Error = TemporalError>
        + Exchange<ParsedTimeOfDayWithShift, FormattedTimeOfDayWithShift, V, Error = TemporalError>
        + Exchange<ParsedExtendedYear, FormattedExtendedYear, V, Error = TemporalError>
        + Exchange<ParsedDecade, FormattedDecade, V, Error = TemporalError>
        + Exchange<ParsedCentury, FormattedCentury, V, Error = TemporalError>
        + Exchange<
            ParsedQualifiedTemporalValue,
            FormattedQualifiedTemporalValue,
            V,
            Error = TemporalError,
        > + Exchange<ParsedRfc3339Timestamp, FormattedRfc3339Timestamp, V, Error = TemporalError>
        + Exchange<ParsedIxdtfZonedTimestamp, FormattedIxdtfZonedTimestamp, V, Error = TemporalError>
        + Exchange<ParsedIxdtfTimestamp, FormattedIxdtfTimestamp, V, Error = TemporalError>
        + Exchange<
            ParsedSeasonalTemporalExpression,
            FormattedSeasonalTemporalExpression,
            V,
            Error = TemporalError,
        > + Exchange<
            ParsedSubYearGroupingExpression,
            FormattedSubYearGroupingExpression,
            V,
            Error = TemporalError,
        > + Exchange<
            ParsedUnspecifiedComponentExpression,
            FormattedUnspecifiedComponentExpression,
            V,
            Error = TemporalError,
        > + Exchange<ParsedTemporalSet, FormattedTemporalSet, V, Error = TemporalError>
        + Exchange<
            ParsedGroupedTimeScaleUnit,
            FormattedGroupedTimeScaleUnit,
            V,
            Error = TemporalError,
        > + Exchange<ParsedDateTimeFormula, FormattedDateTimeFormula, V, Error = TemporalError>
        + Exchange<ParsedDuration, FormattedDuration, V, Error = TemporalError>
        + Exchange<ParsedRecurringInterval, FormattedRecurringInterval, V, Error = TemporalError>
        + Exchange<ParsedTimeInterval, FormattedTimeInterval, V, Error = TemporalError>,
{
}
