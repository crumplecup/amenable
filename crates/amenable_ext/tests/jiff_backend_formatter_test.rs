//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 10
//! of `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalFormatter`
//! — 24 of its 36 edges are real, the other 12 (the CalConnect/
//! ISO 8601-2 extension family) are a real, honest `Unsupported`.

#![cfg(feature = "jiff")]

use amenable_core::{Establish, Exchange};
use amenable_ext::{JiffTimeBackend, JiffVerifier};
use amenable_time::{
    CalendarDateDescriptor, CalendarDateValid, CenturyValid, CompleteDateDescriptor,
    DateWithShiftValid, DecadeValid, DurationDescriptorBuilder, DurationFormValid,
    ExtendedYearValid, FormattedCalendarDateBasic, FormattedCalendarDateExtended, FormattedCentury,
    FormattedDateTimeFormula, FormattedDateWithShift, FormattedDecade, FormattedDuration,
    FormattedExtendedYear, FormattedGroupedTimeScaleUnit, FormattedIxdtfTimestamp,
    FormattedIxdtfZonedTimestamp, FormattedLocalDateTimeExtended, FormattedLocalTimeExtended,
    FormattedOffsetDateTimeBasic, FormattedOffsetDateTimeExtended, FormattedOrdinalDateBasic,
    FormattedOrdinalDateExtended, FormattedQualifiedTemporalValue, FormattedRecurringInterval,
    FormattedReducedCalendarDateBasic, FormattedReducedCalendarDateExtended,
    FormattedReducedLocalTimeExtended, FormattedRfc3339Timestamp,
    FormattedSeasonalTemporalExpression, FormattedSubYearGroupingExpression, FormattedTemporalSet,
    FormattedTimeInterval, FormattedTimeOfDayWithShift, FormattedUnspecifiedComponentExpression,
    FormattedUtcOffsetBasic, FormattedUtcOffsetExtended, FormattedWeekDateBasic,
    FormattedWeekDateExtended, GroupedTimeScaleUnitProof, IxdtfTimeZoneAnnotationDescriptor,
    IxdtfTimestampDescriptorBuilder, IxdtfTimestampProof, LocalDateTimeDescriptorBuilder,
    LocalDateTimeProof, LocalTimeDescriptorBuilder, LocalTimeValid, NamedTimeZoneDescriptorBuilder,
    OffsetDateTimeDescriptorBuilder, OffsetDateTimeProof, OrdinalDateDescriptor, OrdinalDateValid,
    ParsedCalendarDate, ParsedCentury, ParsedDateTimeFormula, ParsedDateWithShift, ParsedDecade,
    ParsedDuration, ParsedExtendedYear, ParsedGroupedTimeScaleUnit, ParsedIxdtfTimestamp,
    ParsedIxdtfZonedTimestamp, ParsedLocalDateTime, ParsedOffsetDateTime, ParsedOrdinalDate,
    ParsedQualifiedTemporalValue, ParsedRecurringInterval, ParsedReducedCalendarDate,
    ParsedReducedLocalTime, ParsedRfc3339Timestamp, ParsedSeasonalTemporalExpression,
    ParsedSubYearGroupingExpression, ParsedTemporalSet, ParsedTimeInterval,
    ParsedTimeOfDayWithShift, ParsedUnspecifiedComponentExpression, ParsedUtcOffset,
    ParsedWeekDate, QualifiedTemporalValueProof, RecurringIntervalDescriptorBuilder,
    RecurringIntervalFormValid, ReducedCalendarDateDescriptor, ReducedCalendarDateValid,
    ReducedLocalTimeDescriptor, ReducedLocalTimeValid, Rfc3339TimestampProof,
    SeasonalTemporalExpressionProof, SubYearGroupingExpressionProof, TemporalError,
    TemporalErrorKind, TemporalFormatter, TemporalInputToken, TemporalSetProof,
    TimeIntervalDescriptor, TimeIntervalEndpoint, TimeIntervalProof, TimeIntervalRepresentation,
    TimeOfDayWithShiftValid, UnspecifiedComponentExpressionProof, UtcOffsetDescriptorBuilder,
    UtcOffsetSign, UtcOffsetValid, WeekDateDescriptor, WeekDateValid,
};
use miette::{IntoDiagnostic, WrapErr};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalFormatter<JiffVerifier>` -- this alone requires all 36
// edges to be genuine `Exchange` impls.
fn _assert_formatter<T: TemporalFormatter<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_formatter::<JiffTimeBackend>;
};

macro_rules! established_token {
    ($prop:ty) => {
        <$prop as Establish<TemporalInputToken, JiffVerifier>>::establish(TemporalInputToken::new())
    };
}

fn offset_date_time_descriptor(
    (year, month, day): (i32, u8, u8),
    (hour, minute, second): (u8, u8, u8),
    (sign, offset_hours): (UtcOffsetSign, u8),
) -> miette::Result<amenable_time::OffsetDateTimeDescriptor> {
    let local = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(year, month, day),
        ))
        .time(
            LocalTimeDescriptorBuilder::default()
                .hour(hour)
                .minute(minute)
                .second(second)
                .build()
                .into_diagnostic()
                .wrap_err("valid local time")?,
        )
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;
    let offset = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(offset_hours)
        .minutes(0u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    OffsetDateTimeDescriptorBuilder::default()
        .local(local)
        .offset(offset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset date-time descriptor")
}

#[test]
fn formats_a_calendar_date_extended_and_basic() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = CalendarDateDescriptor::new(2024, 3, 10);
    let extended: FormattedCalendarDateExtended = backend
        .exchange(ParsedCalendarDate::new(
            descriptor,
            established_token!(CalendarDateValid),
        ))
        .into_diagnostic()
        .wrap_err("a real calendar date formats")?;
    assert_eq!(extended.text(), "2024-03-10");
    let basic: FormattedCalendarDateBasic = backend
        .exchange(ParsedCalendarDate::new(
            descriptor,
            established_token!(CalendarDateValid),
        ))
        .into_diagnostic()
        .wrap_err("the basic form formats too")?;
    assert_eq!(basic.text(), "20240310");
    Ok(())
}

#[test]
fn formats_a_reduced_calendar_date() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let year_only = ReducedCalendarDateDescriptor::Year { year: 2024 };
    let extended: FormattedReducedCalendarDateExtended = backend
        .exchange(ParsedReducedCalendarDate::new(
            year_only,
            established_token!(ReducedCalendarDateValid),
        ))
        .into_diagnostic()
        .wrap_err("a year-only reduced calendar date formats")?;
    assert_eq!(extended.text(), "2024");

    let year_month = ReducedCalendarDateDescriptor::YearMonth {
        year: 2024,
        month: 3,
    };
    let extended: FormattedReducedCalendarDateExtended = backend
        .exchange(ParsedReducedCalendarDate::new(
            year_month,
            established_token!(ReducedCalendarDateValid),
        ))
        .into_diagnostic()
        .wrap_err("a year-month reduced calendar date formats extended")?;
    assert_eq!(extended.text(), "2024-03");
    let basic: FormattedReducedCalendarDateBasic = backend
        .exchange(ParsedReducedCalendarDate::new(
            year_month,
            established_token!(ReducedCalendarDateValid),
        ))
        .into_diagnostic()
        .wrap_err("a year-month reduced calendar date formats basic")?;
    assert_eq!(basic.text(), "202403");
    Ok(())
}

#[test]
fn formats_an_ordinal_date_extended_and_basic() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = OrdinalDateDescriptor::new(2024, 70);
    let extended: FormattedOrdinalDateExtended = backend
        .exchange(ParsedOrdinalDate::new(
            descriptor,
            established_token!(OrdinalDateValid),
        ))
        .into_diagnostic()
        .wrap_err("a real ordinal date formats")?;
    assert_eq!(extended.text(), "2024-070");
    let basic: FormattedOrdinalDateBasic = backend
        .exchange(ParsedOrdinalDate::new(
            descriptor,
            established_token!(OrdinalDateValid),
        ))
        .into_diagnostic()
        .wrap_err("the basic form formats too")?;
    assert_eq!(basic.text(), "2024070");
    Ok(())
}

#[test]
fn formats_a_week_date_extended_and_basic() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = WeekDateDescriptor::new(2024, 10, 3);
    let extended: FormattedWeekDateExtended = backend
        .exchange(ParsedWeekDate::new(
            descriptor,
            established_token!(WeekDateValid),
        ))
        .into_diagnostic()
        .wrap_err("a real week date formats")?;
    assert_eq!(extended.text(), "2024-W10-3");
    let basic: FormattedWeekDateBasic = backend
        .exchange(ParsedWeekDate::new(
            descriptor,
            established_token!(WeekDateValid),
        ))
        .into_diagnostic()
        .wrap_err("the basic form formats too")?;
    assert_eq!(basic.text(), "2024W103");
    Ok(())
}

#[test]
fn formats_a_local_time_with_a_fractional_second() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = LocalTimeDescriptorBuilder::default()
        .hour(13u8)
        .minute(30u8)
        .second(5u8)
        .fractional_second(amenable_time::FractionalSecondDescriptor::new("123"))
        .build()
        .into_diagnostic()
        .wrap_err("valid local time descriptor")?;
    let extended: FormattedLocalTimeExtended = backend
        .exchange(amenable_time::ParsedLocalTime::new(
            descriptor,
            established_token!(LocalTimeValid),
        ))
        .into_diagnostic()
        .wrap_err("a real local time with a fraction formats")?;
    assert_eq!(extended.text(), "13:30:05.123");
    Ok(())
}

#[test]
fn formats_a_reduced_local_time() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let hour_only = ReducedLocalTimeDescriptor::Hour {
        hour: 13,
        fractional_component: None,
    };
    let extended: FormattedReducedLocalTimeExtended = backend
        .exchange(ParsedReducedLocalTime::new(
            hour_only,
            established_token!(ReducedLocalTimeValid),
        ))
        .into_diagnostic()
        .wrap_err("an hour-only reduced local time formats")?;
    assert_eq!(extended.text(), "13");
    Ok(())
}

#[test]
fn rejects_formatting_a_fractional_reduced_local_time() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = ReducedLocalTimeDescriptor::Hour {
        hour: 13,
        fractional_component: Some(amenable_time::TimeScaleUnitFractionDescriptor::new(
            amenable_time::TimeScaleUnitDescriptor::Hour,
            "5".to_owned(),
        )),
    };
    let result: Result<FormattedReducedLocalTimeExtended, TemporalError> = backend.exchange(
        ParsedReducedLocalTime::new(descriptor, established_token!(ReducedLocalTimeValid)),
    );
    let err = result.err().ok_or_else(|| {
        miette::miette!("jiff's civil time has no fractional hour representation")
    })?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn formats_a_utc_offset_extended_and_basic() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Negative)
        .hours(4u8)
        .minutes(30u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let extended: FormattedUtcOffsetExtended = backend
        .exchange(ParsedUtcOffset::new(
            descriptor,
            established_token!(UtcOffsetValid),
        ))
        .into_diagnostic()
        .wrap_err("a real UTC offset formats")?;
    assert_eq!(extended.text(), "-04:30");
    let basic: FormattedUtcOffsetBasic = backend
        .exchange(ParsedUtcOffset::new(
            descriptor,
            established_token!(UtcOffsetValid),
        ))
        .into_diagnostic()
        .wrap_err("the basic form formats too")?;
    assert_eq!(basic.text(), "-0430");
    Ok(())
}

#[test]
fn formats_a_known_zero_offset_as_the_numeric_form_not_z() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Positive)
        .hours(0u8)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let extended: FormattedUtcOffsetExtended = backend
        .exchange(ParsedUtcOffset::new(
            descriptor,
            established_token!(UtcOffsetValid),
        ))
        .into_diagnostic()
        .wrap_err("a known zero offset formats")?;
    assert_eq!(extended.text(), "+00");
    Ok(())
}

#[test]
fn formats_an_unknown_local_offset_as_negative_zero() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = UtcOffsetDescriptorBuilder::default()
        .sign(UtcOffsetSign::Negative)
        .hours(0u8)
        .relationship(amenable_time::UtcOffsetRelationship::UnknownLocalOffset)
        .build()
        .into_diagnostic()
        .wrap_err("valid offset")?;
    let extended: FormattedUtcOffsetExtended = backend
        .exchange(ParsedUtcOffset::new(
            descriptor,
            established_token!(UtcOffsetValid),
        ))
        .into_diagnostic()
        .wrap_err("the unknown-local-offset case formats")?;
    assert_eq!(extended.text(), "-00:00");
    Ok(())
}

#[test]
fn formats_a_local_date_time_extended() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = LocalDateTimeDescriptorBuilder::default()
        .date(CompleteDateDescriptor::Calendar(
            CalendarDateDescriptor::new(2024, 3, 10),
        ))
        .time(
            LocalTimeDescriptorBuilder::default()
                .hour(13u8)
                .minute(30u8)
                .second(0u8)
                .build()
                .into_diagnostic()
                .wrap_err("valid local time")?,
        )
        .build()
        .into_diagnostic()
        .wrap_err("valid local date-time")?;
    let extended: FormattedLocalDateTimeExtended = backend
        .exchange(ParsedLocalDateTime::new(
            descriptor,
            established_token!(LocalDateTimeProof),
        ))
        .into_diagnostic()
        .wrap_err("a real local date-time formats")?;
    assert_eq!(extended.text(), "2024-03-10T13:30:00");
    Ok(())
}

#[test]
fn formats_an_offset_date_time_extended_and_basic() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor =
        offset_date_time_descriptor((2024, 3, 10), (13, 30, 0), (UtcOffsetSign::Negative, 4))?;
    let extended: FormattedOffsetDateTimeExtended = backend
        .exchange(ParsedOffsetDateTime::new(
            descriptor.clone(),
            established_token!(OffsetDateTimeProof),
        ))
        .into_diagnostic()
        .wrap_err("a real offset date-time formats")?;
    assert_eq!(extended.text(), "2024-03-10T13:30:00-04:00");
    let basic: FormattedOffsetDateTimeBasic = backend
        .exchange(ParsedOffsetDateTime::new(
            descriptor,
            established_token!(OffsetDateTimeProof),
        ))
        .into_diagnostic()
        .wrap_err("the basic form formats too")?;
    assert_eq!(basic.text(), "20240310T133000-0400");
    Ok(())
}

#[test]
fn formats_an_rfc3339_timestamp() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor =
        offset_date_time_descriptor((2024, 3, 10), (17, 30, 0), (UtcOffsetSign::Positive, 0))?;
    let formatted: FormattedRfc3339Timestamp = backend
        .exchange(ParsedRfc3339Timestamp::new(
            descriptor,
            established_token!(Rfc3339TimestampProof),
        ))
        .into_diagnostic()
        .wrap_err("a real RFC 3339 timestamp formats")?;
    assert_eq!(formatted.text(), "2024-03-10T17:30:00+00:00");
    Ok(())
}

#[test]
fn formats_an_ixdtf_timestamp_with_a_named_zone_annotation() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let timestamp =
        offset_date_time_descriptor((2024, 3, 10), (13, 30, 0), (UtcOffsetSign::Negative, 4))?;
    let zone = NamedTimeZoneDescriptorBuilder::default()
        .identifier("America/New_York")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")?;
    let descriptor = IxdtfTimestampDescriptorBuilder::default()
        .timestamp(timestamp)
        .time_zone_annotation(IxdtfTimeZoneAnnotationDescriptor::Named(zone))
        .build()
        .into_diagnostic()
        .wrap_err("valid IXDTF timestamp descriptor")?;
    let formatted: FormattedIxdtfTimestamp = backend
        .exchange(ParsedIxdtfTimestamp::new(
            descriptor,
            established_token!(IxdtfTimestampProof),
        ))
        .into_diagnostic()
        .wrap_err("a real IXDTF timestamp formats")?;
    assert_eq!(
        formatted.text(),
        "2024-03-10T13:30:00-04:00[America/New_York]"
    );
    Ok(())
}

#[test]
fn formats_an_ixdtf_zoned_timestamp() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let timestamp =
        offset_date_time_descriptor((2024, 3, 10), (13, 30, 0), (UtcOffsetSign::Negative, 4))?;
    let zone = NamedTimeZoneDescriptorBuilder::default()
        .identifier("America/New_York")
        .build()
        .into_diagnostic()
        .wrap_err("valid named time zone descriptor")?;
    let descriptor = amenable_time::ZonedDateTimeDescriptorBuilder::default()
        .timestamp(timestamp)
        .zone(zone)
        .build()
        .into_diagnostic()
        .wrap_err("valid zoned date-time descriptor")?;
    let formatted: FormattedIxdtfZonedTimestamp = backend
        .exchange(ParsedIxdtfZonedTimestamp::new(
            descriptor,
            established_token!(amenable_time::IxdtfZonedTimestampProof),
        ))
        .into_diagnostic()
        .wrap_err("a real IXDTF zoned timestamp formats")?;
    assert_eq!(
        formatted.text(),
        "2024-03-10T13:30:00-04:00[America/New_York]"
    );
    Ok(())
}

#[test]
fn formats_a_duration() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let descriptor = DurationDescriptorBuilder::default()
        .years(1u32)
        .months(2u32)
        .days(3u32)
        .build()
        .into_diagnostic()
        .wrap_err("valid duration descriptor")?;
    let formatted: FormattedDuration = backend
        .exchange(ParsedDuration::new(
            descriptor,
            established_token!(DurationFormValid),
        ))
        .into_diagnostic()
        .wrap_err("a real duration formats via jiff::Span::Display")?;
    assert_eq!(formatted.text(), "P1Y2M3D");
    Ok(())
}

#[test]
fn formats_a_time_interval_of_offset_date_times() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let start = offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;
    let representation = TimeIntervalRepresentation::StartEnd {
        start: TimeIntervalEndpoint::Value(
            amenable_time::QualifiedOrBareTemporalValueDescriptor::Bare(
                amenable_time::TemporalValueDescriptor::OffsetDateTime(start),
            ),
        ),
        end: TimeIntervalEndpoint::Value(
            amenable_time::QualifiedOrBareTemporalValueDescriptor::Bare(
                amenable_time::TemporalValueDescriptor::OffsetDateTime(end),
            ),
        ),
    };
    let descriptor = TimeIntervalDescriptor::new(representation);
    let formatted: FormattedTimeInterval = backend
        .exchange(ParsedTimeInterval::new(
            descriptor,
            established_token!(TimeIntervalProof),
        ))
        .into_diagnostic()
        .wrap_err("a real time interval of offset date-times formats")?;
    assert_eq!(
        formatted.text(),
        "2024-01-01T00:00:00+00:00/2024-01-02T00:00:00+00:00"
    );
    Ok(())
}

#[test]
fn rejects_formatting_an_open_boundary() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;
    let representation = TimeIntervalRepresentation::StartEnd {
        start: TimeIntervalEndpoint::Open,
        end: TimeIntervalEndpoint::Value(
            amenable_time::QualifiedOrBareTemporalValueDescriptor::Bare(
                amenable_time::TemporalValueDescriptor::OffsetDateTime(end),
            ),
        ),
    };
    let descriptor = TimeIntervalDescriptor::new(representation);
    let result: Result<FormattedTimeInterval, TemporalError> = backend.exchange(
        ParsedTimeInterval::new(descriptor, established_token!(TimeIntervalProof)),
    );
    let err = result.err().ok_or_else(|| {
        miette::miette!("no independently sourced textual convention for Open exists")
    })?;
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    Ok(())
}

#[test]
fn formats_a_bounded_recurring_interval() -> miette::Result<()> {
    let backend = JiffTimeBackend;
    let start = offset_date_time_descriptor((2024, 1, 1), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;
    let end = offset_date_time_descriptor((2024, 1, 2), (0, 0, 0), (UtcOffsetSign::Positive, 0))?;
    let interval = TimeIntervalDescriptor::new(TimeIntervalRepresentation::StartEnd {
        start: TimeIntervalEndpoint::Value(
            amenable_time::QualifiedOrBareTemporalValueDescriptor::Bare(
                amenable_time::TemporalValueDescriptor::OffsetDateTime(start),
            ),
        ),
        end: TimeIntervalEndpoint::Value(
            amenable_time::QualifiedOrBareTemporalValueDescriptor::Bare(
                amenable_time::TemporalValueDescriptor::OffsetDateTime(end),
            ),
        ),
    });
    let descriptor = RecurringIntervalDescriptorBuilder::default()
        .repetitions(5u32)
        .interval(interval)
        .build()
        .into_diagnostic()
        .wrap_err("valid recurring interval descriptor")?;
    let formatted: FormattedRecurringInterval = backend
        .exchange(ParsedRecurringInterval::new(
            descriptor,
            established_token!(RecurringIntervalFormValid),
        ))
        .into_diagnostic()
        .wrap_err("a bounded recurring interval formats")?;
    assert_eq!(
        formatted.text(),
        "R5/2024-01-01T00:00:00+00:00/2024-01-02T00:00:00+00:00"
    );
    Ok(())
}

#[test]
fn the_calconnect_extension_family_is_honestly_unsupported_when_formatting() -> miette::Result<()> {
    let backend = JiffTimeBackend;

    let extended_year: Result<FormattedExtendedYear, TemporalError> = backend.exchange(
        ParsedExtendedYear::new(Default::default(), established_token!(ExtendedYearValid)),
    );
    let decade: Result<FormattedDecade, TemporalError> = backend.exchange(ParsedDecade::new(
        Default::default(),
        established_token!(DecadeValid),
    ));
    let century: Result<FormattedCentury, TemporalError> = backend.exchange(ParsedCentury::new(
        Default::default(),
        established_token!(CenturyValid),
    ));
    let qualified: Result<FormattedQualifiedTemporalValue, TemporalError> =
        backend.exchange(ParsedQualifiedTemporalValue::new(
            Default::default(),
            established_token!(QualifiedTemporalValueProof),
        ));
    let date_with_shift: Result<FormattedDateWithShift, TemporalError> = backend.exchange(
        ParsedDateWithShift::new(Default::default(), established_token!(DateWithShiftValid)),
    );
    let time_of_day_with_shift: Result<FormattedTimeOfDayWithShift, TemporalError> = backend
        .exchange(ParsedTimeOfDayWithShift::new(
            Default::default(),
            established_token!(TimeOfDayWithShiftValid),
        ));
    let seasonal: Result<FormattedSeasonalTemporalExpression, TemporalError> =
        backend.exchange(ParsedSeasonalTemporalExpression::new(
            Default::default(),
            established_token!(SeasonalTemporalExpressionProof),
        ));
    let sub_year_grouping: Result<FormattedSubYearGroupingExpression, TemporalError> = backend
        .exchange(ParsedSubYearGroupingExpression::new(
            Default::default(),
            established_token!(SubYearGroupingExpressionProof),
        ));
    let unspecified: Result<FormattedUnspecifiedComponentExpression, TemporalError> = backend
        .exchange(ParsedUnspecifiedComponentExpression::new(
            Default::default(),
            established_token!(UnspecifiedComponentExpressionProof),
        ));
    let temporal_set: Result<FormattedTemporalSet, TemporalError> = backend.exchange(
        ParsedTemporalSet::new(Default::default(), established_token!(TemporalSetProof)),
    );
    let grouped_unit: Result<FormattedGroupedTimeScaleUnit, TemporalError> =
        backend.exchange(ParsedGroupedTimeScaleUnit::new(
            Default::default(),
            established_token!(GroupedTimeScaleUnitProof),
        ));
    let formula: Result<FormattedDateTimeFormula, TemporalError> =
        backend.exchange(ParsedDateTimeFormula::new(
            Default::default(),
            established_token!(amenable_time::DateTimeFormulaProof),
        ));

    for result in [
        extended_year.map(|_| ()),
        decade.map(|_| ()),
        century.map(|_| ()),
        qualified.map(|_| ()),
        date_with_shift.map(|_| ()),
        time_of_day_with_shift.map(|_| ()),
        seasonal.map(|_| ()),
        sub_year_grouping.map(|_| ()),
        unspecified.map(|_| ()),
        temporal_set.map(|_| ()),
        grouped_unit.map(|_| ()),
        formula.map(|_| ()),
    ] {
        let err = result.err().ok_or_else(|| {
            miette::miette!("the CalConnect/ISO 8601-2 extension family is out of scope")
        })?;
        assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    }
    Ok(())
}
