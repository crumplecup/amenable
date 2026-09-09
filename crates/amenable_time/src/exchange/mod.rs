//! The temporal exchange surface — every `elicit_temporal` trait method
//! re-expressed as an [`Exchange`](amenable_core::Exchange): a proven
//! [`Sidecar`](amenable_core::Sidecar) in, a proven `Sidecar` out.
//!
//! `amenable_time` owns the *shape* — the sidecar types, the boundary
//! tokens, the [`Establish`](amenable_core::Establish) edges, and the
//! [`TemporalParser<V>`](crate::TemporalParser) bundle (whose contract
//! *is* "be the 24 parse exchanges"). The `Exchange` **impls** live in
//! the backend crate: a downstream `Jiff` writes one inherent
//! `fn parse_x(&self, RawInput) -> Result<ParsedX, TemporalError>` per
//! method and `#[amenable_derive::capture_exchange_body]` generates its
//! `impl<V> Exchange<..> for Jiff`. `amenable_time` cannot provide those
//! — the orphan rule forbids `impl<T: …, V> Exchange<RawInput, …, V> for
//! T` (foreign trait, uncovered `Self`).
//!
//! - [`markers`] / [`tokens`] — boundary `Evidence` markers, root input token.
//! - [`establish`] — the 24 output tokens + their `Establish` edges.
//! - [`sidecars`] — [`RawInput`], the shared input sidecar.
//! - [`parse_props`] — per-method composite propositions (multi-proof methods).
//! - [`parse`] — the 24 per-method output sidecars.

mod establish;
mod format;
mod format_props;
mod markers;
mod parse;
mod parse_props;
mod sidecars;
mod tokens;

pub use establish::{
    CalendarDateBasicFormattedToken, CalendarDateExtendedFormattedToken, CalendarDateValidToken,
    CenturyFormattedToken, CenturyValidToken, DateTimeFormulaFormattedToken,
    DateTimeFormulaProofToken, DateWithShiftFormattedToken, DateWithShiftValidToken,
    DecadeFormattedToken, DecadeValidToken, DurationFormValidToken, DurationFormattedToken,
    ExtendedYearFormattedToken, ExtendedYearValidToken, GroupedTimeScaleUnitFormattedToken,
    GroupedTimeScaleUnitProofToken, IxdtfTimestampFormattedToken, IxdtfTimestampProofToken,
    IxdtfZonedTimestampFormattedToken, IxdtfZonedTimestampProofToken,
    LocalDateTimeBasicFormattedToken, LocalDateTimeExtendedFormattedToken, LocalDateTimeProofToken,
    LocalTimeBasicFormattedToken, LocalTimeExtendedFormattedToken, LocalTimeValidToken,
    OffsetDateTimeBasicFormattedToken, OffsetDateTimeExtendedFormattedToken,
    OffsetDateTimeProofToken, OrdinalDateBasicFormattedToken, OrdinalDateExtendedFormattedToken,
    OrdinalDateValidToken, QualifiedTemporalValueFormattedToken, QualifiedTemporalValueProofToken,
    RecurringIntervalFormValidToken, RecurringIntervalFormattedToken,
    ReducedCalendarDateBasicFormattedToken, ReducedCalendarDateExtendedFormattedToken,
    ReducedCalendarDateValidToken, ReducedLocalTimeBasicFormattedToken,
    ReducedLocalTimeExtendedFormattedToken, ReducedLocalTimeValidToken,
    Rfc3339TimestampFormattedToken, Rfc3339TimestampProofToken,
    SeasonalTemporalExpressionFormattedToken, SeasonalTemporalExpressionProofToken,
    SubYearGroupingExpressionFormattedToken, SubYearGroupingExpressionProofToken,
    TemporalSetFormattedToken, TemporalSetProofToken, TimeIntervalFormattedToken,
    TimeIntervalProofToken, TimeOfDayWithShiftFormattedToken, TimeOfDayWithShiftValidToken,
    UnspecifiedComponentExpressionFormattedToken, UnspecifiedComponentExpressionProofToken,
    UtcOffsetBasicFormattedToken, UtcOffsetExtendedFormattedToken, UtcOffsetValidToken,
    WeekDateBasicFormattedToken, WeekDateExtendedFormattedToken, WeekDateValidToken,
};
pub use format::{
    FormattedCalendarDateBasic, FormattedCalendarDateExtended, FormattedCentury,
    FormattedDateTimeFormula, FormattedDateWithShift, FormattedDecade, FormattedDuration,
    FormattedExtendedYear, FormattedGroupedTimeScaleUnit, FormattedIxdtfTimestamp,
    FormattedIxdtfZonedTimestamp, FormattedLocalDateTimeBasic, FormattedLocalDateTimeExtended,
    FormattedLocalTimeBasic, FormattedLocalTimeExtended, FormattedOffsetDateTimeBasic,
    FormattedOffsetDateTimeExtended, FormattedOrdinalDateBasic, FormattedOrdinalDateExtended,
    FormattedQualifiedTemporalValue, FormattedRecurringInterval, FormattedReducedCalendarDateBasic,
    FormattedReducedCalendarDateExtended, FormattedReducedLocalTimeBasic,
    FormattedReducedLocalTimeExtended, FormattedRfc3339Timestamp,
    FormattedSeasonalTemporalExpression, FormattedSubYearGroupingExpression, FormattedTemporalSet,
    FormattedTimeInterval, FormattedTimeOfDayWithShift, FormattedUnspecifiedComponentExpression,
    FormattedUtcOffsetBasic, FormattedUtcOffsetExtended, FormattedWeekDateBasic,
    FormattedWeekDateExtended,
};
pub use format_props::{
    CalendarDateBasicFormatted, CalendarDateExtendedFormatted, CenturyFormatted,
    DateTimeFormulaFormatted, DateWithShiftFormatted, DecadeFormatted, DurationFormatted,
    ExtendedYearFormatted, GroupedTimeScaleUnitFormatted, IxdtfTimestampFormatted,
    IxdtfZonedTimestampFormatted, LocalDateTimeBasicFormatted, LocalDateTimeExtendedFormatted,
    LocalTimeBasicFormatted, LocalTimeExtendedFormatted, OffsetDateTimeBasicFormatted,
    OffsetDateTimeExtendedFormatted, OrdinalDateBasicFormatted, OrdinalDateExtendedFormatted,
    QualifiedTemporalValueFormatted, RecurringIntervalFormatted, ReducedCalendarDateBasicFormatted,
    ReducedCalendarDateExtendedFormatted, ReducedLocalTimeBasicFormatted,
    ReducedLocalTimeExtendedFormatted, Rfc3339TimestampFormatted,
    SeasonalTemporalExpressionFormatted, SubYearGroupingExpressionFormatted, TemporalSetFormatted,
    TimeIntervalFormatted, TimeOfDayWithShiftFormatted, UnspecifiedComponentExpressionFormatted,
    UtcOffsetBasicFormatted, UtcOffsetExtendedFormatted, WeekDateBasicFormatted,
    WeekDateExtendedFormatted,
};
pub use markers::{FormattedTemporalText, RawTemporalText, TemporalInputReceived};
pub use parse::{
    ParsedCalendarDate, ParsedCentury, ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade,
    ParsedDuration, ParsedExtendedYear, ParsedGroupedTimeScaleUnit, ParsedIxdtfTimestamp,
    ParsedIxdtfZonedTimestamp, ParsedLocalDateTime, ParsedLocalTime, ParsedOffsetDateTime,
    ParsedOrdinalDate, ParsedQualifiedTemporalValue, ParsedRecurringInterval,
    ParsedReducedCalendarDate, ParsedReducedLocalTime, ParsedRfc3339Timestamp,
    ParsedSeasonalTemporalExpression, ParsedSubYearGroupingExpression, ParsedTemporalSet,
    ParsedTimeInterval, ParsedTimeOfDayWithShift, ParsedUnspecifiedComponentExpression,
    ParsedUtcOffset, ParsedWeekDate,
};
pub use parse_props::{
    DateTimeFormulaProof, GroupedTimeScaleUnitProof, IxdtfTimestampProof, IxdtfZonedTimestampProof,
    LocalDateTimeProof, OffsetDateTimeProof, QualifiedTemporalValueProof, Rfc3339TimestampProof,
    SeasonalTemporalExpressionProof, SubYearGroupingExpressionProof, TemporalSetProof,
    TimeIntervalProof, UnspecifiedComponentExpressionProof,
};
pub use sidecars::RawInput;
pub use tokens::TemporalInputToken;
