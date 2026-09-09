//! The temporal exchange surface — every `elicit_temporal` trait method
//! re-expressed as an [`Exchange`](amenable_core::Exchange): a proven
//! [`Sidecar`](amenable_core::Sidecar) in, a proven `Sidecar` out.
//!
//! `amenable_time` owns the *shape* — the sidecar types, the boundary
//! tokens, the [`Establish`](amenable_core::Establish) edges, and the
//! per-seam trait bundles ([`TemporalParser<V>`](crate::TemporalParser),
//! [`TemporalFormatter<V>`](crate::TemporalFormatter), and the zone /
//! conversion / interval factories) whose contract *is* "be these
//! exchanges". The `Exchange` **impls** live in the backend crate: a
//! downstream `Jiff` writes one inherent
//! `fn parse_x(&self, RawInput) -> Result<ParsedX, TemporalError>` per
//! method and `#[amenable_derive::capture_exchange_body]` generates its
//! `impl<V> Exchange<..> for Jiff`. `amenable_time` cannot provide those
//! — the orphan rule forbids `impl<T: …, V> Exchange<RawInput, …, V> for
//! T` (foreign trait, uncovered `Self`).
//!
//! - [`markers`] / [`tokens`] — boundary `Evidence` markers, root input token.
//! - [`establish`] / [`factory_establish`] — output tokens + their `Establish` edges.
//! - [`sidecars`] — [`RawInput`], the shared input sidecar.
//! - [`parse_props`] — per-method composite propositions (multi-proof methods).
//! - [`parse`] / [`format`] — the per-method parse / format output sidecars.
//! - [`zone`] / [`conversion`] / [`interval`] — the factory transition sidecars.

mod conversion;
mod establish;
mod factory_establish;
mod format;
mod format_props;
mod interval;
mod markers;
mod parse;
mod parse_props;
mod sidecars;
mod tokens;
mod zone;

pub use conversion::{
    AdjustPrecisionLosslesslyEstablished, AdjustPrecisionLosslesslyInput,
    AdjustPrecisionLosslesslyOutput, AdjustPrecisionLosslesslyPreconditions,
    AdjustPrecisionLosslesslyRequest, NormalizeToUtcEstablished, NormalizeToUtcInput,
    NormalizeToUtcOutput, NormalizeToUtcPreconditions, NormalizeToUtcRequest,
    StripNamedZoneEstablished, StripNamedZoneInput, StripNamedZoneOutput,
    StripNamedZonePreconditions, StripNamedZoneRequest, TruncateSubsecondsEstablished,
    TruncateSubsecondsInput, TruncateSubsecondsOutput, TruncateSubsecondsPreconditions,
    TruncateSubsecondsRequest,
};
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
pub use factory_establish::{
    AdjustPrecisionLosslesslyEstablishedToken, AdjustPrecisionLosslesslyPreconditionsToken,
    AttachNamedZoneEstablishedToken, AttachNamedZonePreconditionsToken,
    ConfirmNamedZoneRevisionEstablishedToken, ConfirmNamedZoneRevisionPreconditionsToken,
    ConfirmZoneAuthorityEstablishedToken, ConfirmZoneAuthorityPreconditionsToken,
    NamedTimeZoneIdentityValidToken, NormalizeToUtcEstablishedToken,
    NormalizeToUtcPreconditionsToken, OrderOffsetEndpointsEstablishedToken,
    OrderOffsetEndpointsPreconditionsToken, ResolveLocalDateTimeEstablishedToken,
    ResolveLocalDateTimePreconditionsToken, StripNamedZoneEstablishedToken,
    StripNamedZonePreconditionsToken, TruncateSubsecondsEstablishedToken,
    TruncateSubsecondsPreconditionsToken,
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
pub use interval::{
    OrderOffsetEndpointsEstablished, OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput,
    OrderOffsetEndpointsPreconditions, OrderOffsetEndpointsRequest,
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
pub use zone::{
    AttachNamedZoneEstablished, AttachNamedZoneInput, AttachNamedZoneOutput,
    AttachNamedZonePreconditions, AttachNamedZoneRequest, ConfirmNamedZoneRevisionEstablished,
    ConfirmNamedZoneRevisionInput, ConfirmNamedZoneRevisionOutput,
    ConfirmNamedZoneRevisionPreconditions, ConfirmNamedZoneRevisionRequest,
    ConfirmZoneAuthorityEstablished, ConfirmZoneAuthorityInput, ConfirmZoneAuthorityOutput,
    ConfirmZoneAuthorityPreconditions, ConfirmZoneAuthorityRequest,
    ResolveLocalDateTimeEstablished, ResolveLocalDateTimeInput, ResolveLocalDateTimeOutput,
    ResolveLocalDateTimePreconditions, ResolveLocalDateTimeRequest, ResolvedNamedTimeZone,
};
