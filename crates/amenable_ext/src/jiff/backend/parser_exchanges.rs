use crate::jiff::backend::duration::nanos_to_fractional_seconds_digits;
use crate::jiff::backend::instant::complete_date_descriptor_to_jiff_date;
use crate::jiff::backend::instant::jiff_civil_datetime_to_local_date_time_descriptor;
use crate::jiff::backend::instant::jiff_parts_to_offset_date_time_descriptor;
use crate::jiff::backend::parser_helpers::jiff_time_zone_annotation_to_descriptor;
use crate::jiff::backend::parser_helpers::parse_ordinal_date_text;
use crate::jiff::backend::parser_helpers::parse_reduced_calendar_date_text;
use crate::jiff::backend::parser_helpers::parse_reduced_local_time_text;
use crate::jiff::backend::parser_helpers::parse_temporal_pieces;
use crate::jiff::backend::parser_helpers::parse_utc_offset_text;
use crate::jiff::backend::parser_helpers::parse_week_date_text;
use crate::jiff::backend::parser_helpers::pieces_to_offset_date_time;
use crate::jiff::backend::parser_helpers::reject_zone_annotation;
use crate::jiff::backend::parser_helpers::unsupported_parse;
use crate::jiff::backend::time_interval::jiff_date_to_calendar_date_descriptor;
use crate::{JiffTimeBackend, JiffVerifier};
use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    CalendarDateValid, CompleteDateDescriptor, FractionalSecondDescriptor, InvalidDescriptorSource,
    IxdtfTimestampDescriptorBuilder, IxdtfTimestampProof, LocalDateTimeProof,
    LocalTimeDescriptorBuilder, LocalTimeValid, OffsetDateTimeProof, OrdinalDateDescriptor,
    OrdinalDateValid, ParseRejectedSource, ParsedCalendarDate, ParsedCentury,
    ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade, ParsedExtendedYear,
    ParsedGroupedTimeScaleUnit, ParsedIxdtfTimestamp, ParsedLocalDateTime, ParsedLocalTime,
    ParsedOffsetDateTime, ParsedOrdinalDate, ParsedQualifiedTemporalValue,
    ParsedReducedCalendarDate, ParsedReducedLocalTime, ParsedRfc3339Timestamp,
    ParsedSeasonalTemporalExpression, ParsedSubYearGroupingExpression, ParsedTemporalSet,
    ParsedTimeOfDayWithShift, ParsedUnspecifiedComponentExpression, ParsedUtcOffset,
    ParsedWeekDate, RawInput, ReducedCalendarDateValid, ReducedLocalTimeValid,
    Rfc3339TimestampProof, TemporalError, TemporalErrorKind, TemporalInputToken, UtcOffsetValid,
    WeekDateDescriptor, WeekDateValid,
};

impl Exchange<RawInput, ParsedCalendarDate, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedCalendarDate, TemporalError> {
        let date: jiff::civil::Date = input.as_str().parse().map_err(|err| {
            TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
                "ISO 8601 calendar date".to_owned(),
                format!("{err}"),
            )))
        })?;
        let descriptor = jiff_date_to_calendar_date_descriptor(date)?;
        let token = <CalendarDateValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedCalendarDate::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedReducedCalendarDate, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedReducedCalendarDate, TemporalError> {
        let descriptor = parse_reduced_calendar_date_text(input.as_str())?;
        let token =
            <ReducedCalendarDateValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
                <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
            );
        Ok(ParsedReducedCalendarDate::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedExtendedYear, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedExtendedYear, TemporalError> {
        Err(unsupported_parse("an ISO 8601-2 extended year"))
    }
}

impl Exchange<RawInput, ParsedDecade, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedDecade, TemporalError> {
        Err(unsupported_parse("a Gregorian decade"))
    }
}

impl Exchange<RawInput, ParsedCentury, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedCentury, TemporalError> {
        Err(unsupported_parse("a Gregorian century"))
    }
}

impl Exchange<RawInput, ParsedQualifiedTemporalValue, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedQualifiedTemporalValue, TemporalError> {
        Err(unsupported_parse(
            "an ISO 8601-2 explicitly-qualified temporal value",
        ))
    }
}

impl Exchange<RawInput, ParsedOrdinalDate, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedOrdinalDate, TemporalError> {
        let (year, day_of_year) = parse_ordinal_date_text(input.as_str())?;
        // Real validation: build the equivalent jiff::civil::Date via
        // the shared Phase 3 helper, discarding the value -- confirms
        // the day-of-year is genuinely valid for the (possibly leap)
        // year, not just well-formed digits.
        complete_date_descriptor_to_jiff_date(CompleteDateDescriptor::Ordinal(
            OrdinalDateDescriptor::new(year, day_of_year),
        ))?;
        let token = <OrdinalDateValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedOrdinalDate::new(
            OrdinalDateDescriptor::new(year, day_of_year),
            token,
        ))
    }
}

impl Exchange<RawInput, ParsedWeekDate, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedWeekDate, TemporalError> {
        let (week_year, week, weekday) = parse_week_date_text(input.as_str())?;
        complete_date_descriptor_to_jiff_date(CompleteDateDescriptor::Week(
            WeekDateDescriptor::new(week_year, week, weekday),
        ))?;
        let token = <WeekDateValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedWeekDate::new(
            WeekDateDescriptor::new(week_year, week, weekday),
            token,
        ))
    }
}

impl Exchange<RawInput, ParsedLocalTime, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedLocalTime, TemporalError> {
        let time: jiff::civil::Time = input.as_str().parse().map_err(|err| {
            TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
                "ISO 8601 local time".to_owned(),
                format!("{err}"),
            )))
        })?;
        let mut builder = LocalTimeDescriptorBuilder::default()
            .hour(u8::try_from(time.hour()).unwrap_or_default())
            .minute(u8::try_from(time.minute()).unwrap_or_default())
            .second(u8::try_from(time.second()).unwrap_or_default());
        let nanos = time.subsec_nanosecond();
        if nanos != 0 {
            builder = builder.fractional_second(FractionalSecondDescriptor::new(
                nanos_to_fractional_seconds_digits(i64::from(nanos)),
            ));
        }
        let descriptor = builder.build().map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not build a local time descriptor: {err}"
                )),
            ))
        })?;
        let token = <LocalTimeValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedLocalTime::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedReducedLocalTime, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedReducedLocalTime, TemporalError> {
        let descriptor = parse_reduced_local_time_text(input.as_str())?;
        let token =
            <ReducedLocalTimeValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
                <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
            );
        Ok(ParsedReducedLocalTime::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedUtcOffset, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedUtcOffset, TemporalError> {
        let descriptor = parse_utc_offset_text(input.as_str())?;
        let token = <UtcOffsetValid as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedUtcOffset::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedLocalDateTime, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedLocalDateTime, TemporalError> {
        let datetime: jiff::civil::DateTime = input.as_str().parse().map_err(|err| {
            TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
                "ISO 8601 local date-time".to_owned(),
                format!("{err}"),
            )))
        })?;
        let descriptor = jiff_civil_datetime_to_local_date_time_descriptor(datetime)?;
        let token = <LocalDateTimeProof as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedLocalDateTime::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedOffsetDateTime, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedOffsetDateTime, TemporalError> {
        let text = input.as_str();
        let pieces = parse_temporal_pieces(text, "ISO 8601 offset date-time")?;
        reject_zone_annotation(&pieces, text, "ISO 8601 offset date-time")?;
        let (local, offset) =
            pieces_to_offset_date_time(&pieces, text, "ISO 8601 offset date-time")?;
        let descriptor = jiff_parts_to_offset_date_time_descriptor(local, offset)?;
        let token = <OffsetDateTimeProof as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedOffsetDateTime::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedDateWithShift, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedDateWithShift, TemporalError> {
        Err(unsupported_parse(
            "a CalConnect explicit date-with-shift value",
        ))
    }
}

impl Exchange<RawInput, ParsedTimeOfDayWithShift, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedTimeOfDayWithShift, TemporalError> {
        Err(unsupported_parse(
            "a CalConnect explicit time-of-day-with-shift value",
        ))
    }
}

impl Exchange<RawInput, ParsedRfc3339Timestamp, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedRfc3339Timestamp, TemporalError> {
        let text = input.as_str();
        let pieces = parse_temporal_pieces(text, "RFC 3339 timestamp")?;
        reject_zone_annotation(&pieces, text, "RFC 3339 timestamp")?;
        let (local, offset) = pieces_to_offset_date_time(&pieces, text, "RFC 3339 timestamp")?;
        let descriptor = jiff_parts_to_offset_date_time_descriptor(local, offset)?;
        let token =
            <Rfc3339TimestampProof as Establish<TemporalInputToken, JiffVerifier>>::establish(
                <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
            );
        Ok(ParsedRfc3339Timestamp::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedIxdtfTimestamp, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: RawInput) -> Result<ParsedIxdtfTimestamp, TemporalError> {
        let text = input.as_str();
        let pieces = parse_temporal_pieces(text, "RFC 9557 IXDTF timestamp")?;
        let (local, offset) =
            pieces_to_offset_date_time(&pieces, text, "RFC 9557 IXDTF timestamp")?;
        let timestamp = jiff_parts_to_offset_date_time_descriptor(local, offset)?;
        let mut builder = IxdtfTimestampDescriptorBuilder::default().timestamp(timestamp);
        if let Some(annotation) = pieces.time_zone_annotation() {
            builder =
                builder.time_zone_annotation(jiff_time_zone_annotation_to_descriptor(annotation)?);
        }
        let descriptor = builder.build().map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not build an IXDTF timestamp descriptor: {err}"
                )),
            ))
        })?;
        let token = <IxdtfTimestampProof as Establish<TemporalInputToken, JiffVerifier>>::establish(
            <RawInput as Sidecar<JiffVerifier>>::sidecar(&input),
        );
        Ok(ParsedIxdtfTimestamp::new(descriptor, token))
    }
}

impl Exchange<RawInput, ParsedSeasonalTemporalExpression, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(
        &self,
        _input: RawInput,
    ) -> Result<ParsedSeasonalTemporalExpression, TemporalError> {
        Err(unsupported_parse("an ISO 8601-2 seasonal expression"))
    }
}

impl Exchange<RawInput, ParsedSubYearGroupingExpression, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedSubYearGroupingExpression, TemporalError> {
        Err(unsupported_parse(
            "an ISO 8601-2 sub-year grouping expression",
        ))
    }
}

impl Exchange<RawInput, ParsedUnspecifiedComponentExpression, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(
        &self,
        _input: RawInput,
    ) -> Result<ParsedUnspecifiedComponentExpression, TemporalError> {
        Err(unsupported_parse(
            "an ISO 8601-2 unspecified-component expression",
        ))
    }
}

impl Exchange<RawInput, ParsedTemporalSet, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedTemporalSet, TemporalError> {
        Err(unsupported_parse("an ISO 8601-2 temporal set"))
    }
}

impl Exchange<RawInput, ParsedGroupedTimeScaleUnit, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedGroupedTimeScaleUnit, TemporalError> {
        Err(unsupported_parse(
            "an ISO 8601-2 grouped time-scale unit expression",
        ))
    }
}

impl Exchange<RawInput, ParsedDateTimeFormula, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, _input)))]
    fn exchange(&self, _input: RawInput) -> Result<ParsedDateTimeFormula, TemporalError> {
        Err(unsupported_parse("a CalConnect date-time formula"))
    }
}
