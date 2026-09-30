use super::duration::duration_descriptor_to_jiff_span;
use super::formatter_helpers::{
    format_calendar_date, format_ixdtf_timestamp, format_local_date_time, format_local_time,
    format_offset_date_time, format_ordinal_date, format_recurring_interval,
    format_reduced_calendar_date, format_reduced_local_time, format_time_interval_representation,
    format_utc_offset, format_week_date, format_zoned_date_time, unsupported_format,
};
use super::types::{JiffTimeBackend, JiffVerifier};
use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    CalendarDateBasicFormatted, CalendarDateExtendedFormatted, DurationFormatted,
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
    FormattedTemporalText, FormattedTimeInterval, FormattedTimeOfDayWithShift,
    FormattedUnspecifiedComponentExpression, FormattedUtcOffsetBasic, FormattedUtcOffsetExtended,
    FormattedWeekDateBasic, FormattedWeekDateExtended, IxdtfTimestampFormatted,
    IxdtfZonedTimestampFormatted, LocalDateTimeBasicFormatted, LocalDateTimeExtendedFormatted,
    LocalTimeBasicFormatted, LocalTimeExtendedFormatted, OffsetDateTimeBasicFormatted,
    OffsetDateTimeExtendedFormatted, OrdinalDateBasicFormatted, OrdinalDateExtendedFormatted,
    ParsedCalendarDate, ParsedCentury, ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade,
    ParsedDuration, ParsedExtendedYear, ParsedGroupedTimeScaleUnit, ParsedIxdtfTimestamp,
    ParsedIxdtfZonedTimestamp, ParsedLocalDateTime, ParsedLocalTime, ParsedOffsetDateTime,
    ParsedOrdinalDate, ParsedQualifiedTemporalValue, ParsedRecurringInterval,
    ParsedReducedCalendarDate, ParsedReducedLocalTime, ParsedRfc3339Timestamp,
    ParsedSeasonalTemporalExpression, ParsedSubYearGroupingExpression, ParsedTemporalSet,
    ParsedTimeInterval, ParsedTimeOfDayWithShift, ParsedUnspecifiedComponentExpression,
    ParsedUtcOffset, ParsedWeekDate, RecurringIntervalFormatted, ReducedCalendarDateBasicFormatted,
    ReducedCalendarDateExtendedFormatted, ReducedLocalTimeBasicFormatted,
    ReducedLocalTimeExtendedFormatted, Rfc3339TimestampFormatted, TemporalError,
    TimeIntervalFormatted, UtcOffsetBasicFormatted, UtcOffsetExtendedFormatted,
    WeekDateBasicFormatted, WeekDateExtendedFormatted,
};

macro_rules! jiff_backend_format_edge {
    ($parsed:ident, $output:ident, $prop:ident, |$desc:ident| $body:expr) => {
        impl Exchange<$parsed, $output, JiffVerifier> for JiffTimeBackend {
            type Error = TemporalError;

            #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
            fn exchange(&self, input: $parsed) -> Result<$output, TemporalError> {
                let text: String = {
                    let $desc = input.descriptor();
                    $body
                }?;
                let credential = <$parsed as Sidecar<JiffVerifier>>::sidecar(&input);
                let token = <$prop as Establish<_, JiffVerifier>>::establish(credential);
                Ok($output::new(FormattedTemporalText::new(text), token))
            }
        }
    };
}

macro_rules! jiff_backend_format_unsupported {
    ($($parsed:ident => $output:ident, $label:literal;)+) => {
        $(
            impl Exchange<$parsed, $output, JiffVerifier> for JiffTimeBackend {
                type Error = TemporalError;

                #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
                fn exchange(&self, _input: $parsed) -> Result<$output, TemporalError> {
                    Err(unsupported_format($label))
                }
            }
        )+
    };
}

jiff_backend_format_edge!(
    ParsedCalendarDate,
    FormattedCalendarDateExtended,
    CalendarDateExtendedFormatted,
    |d| Ok(format_calendar_date(d, false))
);
jiff_backend_format_edge!(
    ParsedCalendarDate,
    FormattedCalendarDateBasic,
    CalendarDateBasicFormatted,
    |d| Ok(format_calendar_date(d, true))
);
jiff_backend_format_edge!(
    ParsedReducedCalendarDate,
    FormattedReducedCalendarDateExtended,
    ReducedCalendarDateExtendedFormatted,
    |d| Ok(format_reduced_calendar_date(d, false))
);
jiff_backend_format_edge!(
    ParsedReducedCalendarDate,
    FormattedReducedCalendarDateBasic,
    ReducedCalendarDateBasicFormatted,
    |d| Ok(format_reduced_calendar_date(d, true))
);
jiff_backend_format_edge!(
    ParsedOrdinalDate,
    FormattedOrdinalDateExtended,
    OrdinalDateExtendedFormatted,
    |d| Ok(format_ordinal_date(d, false))
);
jiff_backend_format_edge!(
    ParsedOrdinalDate,
    FormattedOrdinalDateBasic,
    OrdinalDateBasicFormatted,
    |d| Ok(format_ordinal_date(d, true))
);
jiff_backend_format_edge!(
    ParsedWeekDate,
    FormattedWeekDateExtended,
    WeekDateExtendedFormatted,
    |d| Ok(format_week_date(d, false))
);
jiff_backend_format_edge!(
    ParsedWeekDate,
    FormattedWeekDateBasic,
    WeekDateBasicFormatted,
    |d| Ok(format_week_date(d, true))
);
jiff_backend_format_edge!(
    ParsedLocalTime,
    FormattedLocalTimeExtended,
    LocalTimeExtendedFormatted,
    |d| Ok(format_local_time(d, false))
);
jiff_backend_format_edge!(
    ParsedLocalTime,
    FormattedLocalTimeBasic,
    LocalTimeBasicFormatted,
    |d| Ok(format_local_time(d, true))
);
jiff_backend_format_edge!(
    ParsedReducedLocalTime,
    FormattedReducedLocalTimeExtended,
    ReducedLocalTimeExtendedFormatted,
    |d| format_reduced_local_time(d, false)
);
jiff_backend_format_edge!(
    ParsedReducedLocalTime,
    FormattedReducedLocalTimeBasic,
    ReducedLocalTimeBasicFormatted,
    |d| format_reduced_local_time(d, true)
);
jiff_backend_format_edge!(
    ParsedUtcOffset,
    FormattedUtcOffsetExtended,
    UtcOffsetExtendedFormatted,
    |d| Ok(format_utc_offset(d, false))
);
jiff_backend_format_edge!(
    ParsedUtcOffset,
    FormattedUtcOffsetBasic,
    UtcOffsetBasicFormatted,
    |d| Ok(format_utc_offset(d, true))
);
jiff_backend_format_edge!(
    ParsedLocalDateTime,
    FormattedLocalDateTimeExtended,
    LocalDateTimeExtendedFormatted,
    |d| Ok(format_local_date_time(d, false))
);
jiff_backend_format_edge!(
    ParsedLocalDateTime,
    FormattedLocalDateTimeBasic,
    LocalDateTimeBasicFormatted,
    |d| Ok(format_local_date_time(d, true))
);
jiff_backend_format_edge!(
    ParsedOffsetDateTime,
    FormattedOffsetDateTimeExtended,
    OffsetDateTimeExtendedFormatted,
    |d| Ok(format_offset_date_time(d, false))
);
jiff_backend_format_edge!(
    ParsedOffsetDateTime,
    FormattedOffsetDateTimeBasic,
    OffsetDateTimeBasicFormatted,
    |d| Ok(format_offset_date_time(d, true))
);
jiff_backend_format_edge!(
    ParsedRfc3339Timestamp,
    FormattedRfc3339Timestamp,
    Rfc3339TimestampFormatted,
    |d| Ok(format_offset_date_time(d, false))
);
jiff_backend_format_edge!(
    ParsedIxdtfTimestamp,
    FormattedIxdtfTimestamp,
    IxdtfTimestampFormatted,
    |d| Ok(format_ixdtf_timestamp(d))
);
jiff_backend_format_edge!(
    ParsedIxdtfZonedTimestamp,
    FormattedIxdtfZonedTimestamp,
    IxdtfZonedTimestampFormatted,
    |d| Ok(format_zoned_date_time(d))
);
jiff_backend_format_edge!(ParsedDuration, FormattedDuration, DurationFormatted, |d| {
    duration_descriptor_to_jiff_span(d).map(|span| span.to_string())
});
jiff_backend_format_edge!(
    ParsedTimeInterval,
    FormattedTimeInterval,
    TimeIntervalFormatted,
    |d| format_time_interval_representation(d.representation())
);
jiff_backend_format_edge!(
    ParsedRecurringInterval,
    FormattedRecurringInterval,
    RecurringIntervalFormatted,
    |d| format_recurring_interval(d)
);

jiff_backend_format_unsupported! {
    ParsedDateWithShift => FormattedDateWithShift, "a CalConnect explicit date-with-shift value";
    ParsedTimeOfDayWithShift => FormattedTimeOfDayWithShift, "a CalConnect explicit time-of-day-with-shift value";
    ParsedExtendedYear => FormattedExtendedYear, "an ISO 8601-2 extended year";
    ParsedDecade => FormattedDecade, "a Gregorian decade";
    ParsedCentury => FormattedCentury, "a Gregorian century";
    ParsedQualifiedTemporalValue => FormattedQualifiedTemporalValue, "an ISO 8601-2 explicitly-qualified temporal value";
    ParsedSeasonalTemporalExpression => FormattedSeasonalTemporalExpression, "an ISO 8601-2 seasonal expression";
    ParsedSubYearGroupingExpression => FormattedSubYearGroupingExpression, "an ISO 8601-2 sub-year grouping expression";
    ParsedUnspecifiedComponentExpression => FormattedUnspecifiedComponentExpression, "an ISO 8601-2 unspecified-component expression";
    ParsedTemporalSet => FormattedTemporalSet, "an ISO 8601-2 temporal set";
    ParsedGroupedTimeScaleUnit => FormattedGroupedTimeScaleUnit, "an ISO 8601-2 grouped time-scale unit expression";
    ParsedDateTimeFormula => FormattedDateTimeFormula, "a CalConnect date-time formula";
}
