//! `JiffTimeBackend`'s real jiff-backed `Exchange` bodies — Phase 9 of
//! `docs/AMENABLE_TIME_JIFF_BACKEND_PLAN.md`: `TemporalParser` — 13 of
//! its 24 edges are real, the other 11 (the CalConnect/ISO 8601-2
//! extension family, plus `TimeInterval`) are a real, honest
//! `Unsupported`.

#![cfg(feature = "jiff")]

use amenable_core::Exchange;
use amenable_ext::{JiffTimeBackend, JiffVerifier};
use amenable_time::{
    IxdtfTimeZoneAnnotationDescriptor, ParsedCalendarDate, ParsedCentury, ParsedDateTimeFormula,
    ParsedDateWithShift, ParsedDecade, ParsedExtendedYear, ParsedGroupedTimeScaleUnit,
    ParsedIxdtfTimestamp, ParsedLocalDateTime, ParsedLocalTime, ParsedOffsetDateTime,
    ParsedOrdinalDate, ParsedQualifiedTemporalValue, ParsedReducedCalendarDate,
    ParsedReducedLocalTime, ParsedRfc3339Timestamp, ParsedSeasonalTemporalExpression,
    ParsedSubYearGroupingExpression, ParsedTemporalSet, ParsedTimeOfDayWithShift,
    ParsedUnspecifiedComponentExpression, ParsedUtcOffset, ParsedWeekDate, RawInput,
    ReducedCalendarDateDescriptor, ReducedLocalTimeDescriptor, TemporalError, TemporalErrorKind,
    TemporalParser, UtcOffsetRelationship, UtcOffsetSign,
};

// Fails to compile if `JiffTimeBackend` stops resolving as a real
// `TemporalParser<JiffVerifier>` -- this alone requires all 24 edges
// to be genuine `Exchange` impls.
fn _assert_parser<T: TemporalParser<JiffVerifier>>() {}
const _: () = {
    let _ = _assert_parser::<JiffTimeBackend>;
};

#[test]
fn parse_calendar_date_parses_a_real_iso8601_date() {
    let backend = JiffTimeBackend;
    let parsed: ParsedCalendarDate = backend
        .exchange(RawInput::received("2024-03-10"))
        .expect("a real ISO 8601 calendar date parses via jiff::civil::Date::FromStr");
    assert_eq!(parsed.descriptor().year(), 2024);
    assert_eq!(parsed.descriptor().month(), 3);
    assert_eq!(parsed.descriptor().day(), 10);
}

#[test]
fn parse_calendar_date_rejects_malformed_text() {
    let backend = JiffTimeBackend;
    let result: Result<ParsedCalendarDate, TemporalError> =
        backend.exchange(RawInput::received("not a date"));
    let err = result.expect_err("malformed text is not a real calendar date");
    assert!(matches!(&**err.kind(), TemporalErrorKind::ParseRejected(_)));
}

#[test]
fn parse_reduced_calendar_date_parses_year_only() {
    let backend = JiffTimeBackend;
    let parsed: ParsedReducedCalendarDate = backend
        .exchange(RawInput::received("2024"))
        .expect("a year-only reduced calendar date parses");
    assert_eq!(
        parsed.descriptor(),
        &ReducedCalendarDateDescriptor::Year { year: 2024 }
    );
}

#[test]
fn parse_reduced_calendar_date_parses_year_month() {
    let backend = JiffTimeBackend;
    let parsed: ParsedReducedCalendarDate = backend
        .exchange(RawInput::received("2024-03"))
        .expect("a year-month reduced calendar date parses");
    assert_eq!(
        parsed.descriptor(),
        &ReducedCalendarDateDescriptor::YearMonth {
            year: 2024,
            month: 3
        }
    );
}

#[test]
fn parse_ordinal_date_parses_the_extended_form() {
    let backend = JiffTimeBackend;
    let parsed: ParsedOrdinalDate = backend
        .exchange(RawInput::received("2024-070"))
        .expect("jiff's own Date::day_of_year validates this ordinal date is real");
    assert_eq!(parsed.descriptor().year(), 2024);
    assert_eq!(parsed.descriptor().day_of_year(), 70);
}

#[test]
fn parse_ordinal_date_parses_the_basic_form() {
    let backend = JiffTimeBackend;
    let parsed: ParsedOrdinalDate = backend
        .exchange(RawInput::received("2024070"))
        .expect("the 7-digit basic form parses too");
    assert_eq!(parsed.descriptor().year(), 2024);
    assert_eq!(parsed.descriptor().day_of_year(), 70);
}

#[test]
fn parse_ordinal_date_rejects_a_real_out_of_range_day() {
    let backend = JiffTimeBackend;
    // 2023 is not a leap year -- day 366 does not exist.
    let result: Result<ParsedOrdinalDate, TemporalError> =
        backend.exchange(RawInput::received("2023-366"));
    result.expect_err("2023 has only 365 days; jiff's own Date construction rejects this");
}

#[test]
fn parse_week_date_parses_the_extended_form() {
    let backend = JiffTimeBackend;
    let parsed: ParsedWeekDate = backend
        .exchange(RawInput::received("2024-W10-3"))
        .expect("a real ISO 8601 week date parses");
    assert_eq!(parsed.descriptor().week_year(), 2024);
    assert_eq!(parsed.descriptor().week(), 10);
    assert_eq!(parsed.descriptor().weekday(), 3);
}

#[test]
fn parse_week_date_parses_the_basic_form() {
    let backend = JiffTimeBackend;
    let parsed: ParsedWeekDate = backend
        .exchange(RawInput::received("2024W103"))
        .expect("the basic form parses too");
    assert_eq!(parsed.descriptor().week_year(), 2024);
    assert_eq!(parsed.descriptor().week(), 10);
    assert_eq!(parsed.descriptor().weekday(), 3);
}

#[test]
fn parse_local_time_parses_a_real_iso8601_time() {
    let backend = JiffTimeBackend;
    let parsed: ParsedLocalTime = backend
        .exchange(RawInput::received("13:30:00"))
        .expect("a real ISO 8601 local time parses via jiff::civil::Time::FromStr");
    assert_eq!(parsed.descriptor().hour(), 13);
    assert_eq!(parsed.descriptor().minute(), 30);
}

#[test]
fn parse_reduced_local_time_parses_hour_only() {
    let backend = JiffTimeBackend;
    let parsed: ParsedReducedLocalTime = backend
        .exchange(RawInput::received("13"))
        .expect("an hour-only reduced local time parses");
    assert_eq!(
        parsed.descriptor(),
        &ReducedLocalTimeDescriptor::Hour {
            hour: 13,
            fractional_component: None
        }
    );
}

#[test]
fn parse_reduced_local_time_parses_hour_minute() {
    let backend = JiffTimeBackend;
    let parsed: ParsedReducedLocalTime = backend
        .exchange(RawInput::received("13:30"))
        .expect("an hour-minute reduced local time parses");
    assert_eq!(
        parsed.descriptor(),
        &ReducedLocalTimeDescriptor::HourMinute {
            hour: 13,
            minute: 30,
            fractional_component: None
        }
    );
}

#[test]
fn parse_reduced_local_time_rejects_a_fractional_component() {
    let backend = JiffTimeBackend;
    let result: Result<ParsedReducedLocalTime, TemporalError> =
        backend.exchange(RawInput::received("13,5"));
    let err = result.expect_err("jiff's civil time has no fractional hour representation");
    assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
}

#[test]
fn parse_utc_offset_parses_zulu() {
    let backend = JiffTimeBackend;
    let parsed: ParsedUtcOffset = backend
        .exchange(RawInput::received("Z"))
        .expect("Z is a real, known zero offset");
    assert_eq!(parsed.descriptor().sign(), UtcOffsetSign::Positive);
    assert_eq!(parsed.descriptor().hours(), 0);
    assert_eq!(
        parsed.descriptor().relationship(),
        UtcOffsetRelationship::Known
    );
}

#[test]
fn parse_utc_offset_parses_a_negative_zero_as_unknown_local_offset() {
    let backend = JiffTimeBackend;
    let parsed: ParsedUtcOffset = backend
        .exchange(RawInput::received("-00:00"))
        .expect("a negative zero offset is RFC 9557's unknown-local-offset case");
    assert_eq!(
        parsed.descriptor().relationship(),
        UtcOffsetRelationship::UnknownLocalOffset
    );
}

#[test]
fn parse_utc_offset_parses_a_positive_offset_with_minutes() {
    let backend = JiffTimeBackend;
    let parsed: ParsedUtcOffset = backend
        .exchange(RawInput::received("+05:30"))
        .expect("a real numeric offset with minutes parses");
    assert_eq!(parsed.descriptor().sign(), UtcOffsetSign::Positive);
    assert_eq!(parsed.descriptor().hours(), 5);
    assert_eq!(parsed.descriptor().minutes(), Some(30));
}

#[test]
fn parse_utc_offset_rejects_an_out_of_range_offset() {
    let backend = JiffTimeBackend;
    let result: Result<ParsedUtcOffset, TemporalError> =
        backend.exchange(RawInput::received("+99:00"));
    result.expect_err("a +99:00 offset is not real, jiff's own range check rejects it");
}

#[test]
fn parse_local_date_time_parses_a_real_iso8601_datetime() {
    let backend = JiffTimeBackend;
    let parsed: ParsedLocalDateTime = backend
        .exchange(RawInput::received("2024-03-10T13:30:00"))
        .expect("a real ISO 8601 local date-time parses via jiff::civil::DateTime::FromStr");
    assert_eq!(parsed.descriptor().time().hour(), 13);
}

#[test]
fn parse_offset_date_time_parses_a_real_offset_date_time() {
    let backend = JiffTimeBackend;
    let parsed: ParsedOffsetDateTime = backend
        .exchange(RawInput::received("2024-03-10T13:30:00-04:00"))
        .expect("a real offset date-time parses via jiff's real Pieces decomposition");
    assert_eq!(parsed.descriptor().offset().sign(), UtcOffsetSign::Negative);
    assert_eq!(parsed.descriptor().offset().hours(), 4);
}

#[test]
fn parse_offset_date_time_rejects_a_zone_annotation() {
    let backend = JiffTimeBackend;
    let result: Result<ParsedOffsetDateTime, TemporalError> = backend.exchange(RawInput::received(
        "2024-03-10T13:30:00-04:00[America/New_York]",
    ));
    let err =
        result.expect_err("plain ISO 8601 offset date-times have no [...] zone-bracket syntax");
    assert!(matches!(&**err.kind(), TemporalErrorKind::ParseRejected(_)));
}

#[test]
fn parse_rfc3339_timestamp_parses_a_real_timestamp() {
    let backend = JiffTimeBackend;
    let parsed: ParsedRfc3339Timestamp = backend
        .exchange(RawInput::received("2024-03-10T17:30:00Z"))
        .expect("a real RFC 3339 timestamp parses");
    assert_eq!(parsed.descriptor().offset().hours(), 0);
}

#[test]
fn parse_ixdtf_timestamp_parses_a_named_zone_annotation() {
    let backend = JiffTimeBackend;
    let parsed: ParsedIxdtfTimestamp = backend
        .exchange(RawInput::received(
            "2024-03-10T13:30:00-04:00[America/New_York]",
        ))
        .expect("a real IXDTF timestamp with a named zone annotation parses");
    match parsed.descriptor().time_zone_annotation() {
        Some(IxdtfTimeZoneAnnotationDescriptor::Named(named)) => {
            assert_eq!(named.identifier(), "America/New_York");
        }
        other => panic!("expected a named zone annotation, got {other:?}"),
    }
}

#[test]
fn parse_ixdtf_timestamp_parses_an_offset_annotation() {
    let backend = JiffTimeBackend;
    let parsed: ParsedIxdtfTimestamp = backend
        .exchange(RawInput::received("2024-03-10T13:30:00-04:00[-04:00]"))
        .expect("a real IXDTF timestamp with an offset annotation parses");
    match parsed.descriptor().time_zone_annotation() {
        Some(IxdtfTimeZoneAnnotationDescriptor::Offset(offset)) => {
            assert_eq!(offset.hours(), 4);
        }
        other => panic!("expected an offset annotation, got {other:?}"),
    }
}

#[test]
fn parse_ixdtf_timestamp_without_an_annotation_still_parses() {
    let backend = JiffTimeBackend;
    let parsed: ParsedIxdtfTimestamp = backend
        .exchange(RawInput::received("2024-03-10T13:30:00-04:00"))
        .expect("a zone annotation is optional under IXDTF");
    assert!(parsed.descriptor().time_zone_annotation().is_none());
}

#[test]
fn the_calconnect_extension_family_is_honestly_unsupported() {
    let backend = JiffTimeBackend;

    let extended_year: Result<ParsedExtendedYear, TemporalError> =
        backend.exchange(RawInput::received("Y10000"));
    let decade: Result<ParsedDecade, TemporalError> = backend.exchange(RawInput::received("202"));
    let century: Result<ParsedCentury, TemporalError> = backend.exchange(RawInput::received("20"));
    let qualified: Result<ParsedQualifiedTemporalValue, TemporalError> =
        backend.exchange(RawInput::received("2024?"));
    let date_with_shift: Result<ParsedDateWithShift, TemporalError> =
        backend.exchange(RawInput::received("2024-03-10S+01"));
    let time_of_day_with_shift: Result<ParsedTimeOfDayWithShift, TemporalError> =
        backend.exchange(RawInput::received("13:30S+01"));
    let seasonal: Result<ParsedSeasonalTemporalExpression, TemporalError> =
        backend.exchange(RawInput::received("2024-24"));
    let sub_year_grouping: Result<ParsedSubYearGroupingExpression, TemporalError> =
        backend.exchange(RawInput::received("2024-Q1"));
    let unspecified: Result<ParsedUnspecifiedComponentExpression, TemporalError> =
        backend.exchange(RawInput::received("2024-XX"));
    let temporal_set: Result<ParsedTemporalSet, TemporalError> =
        backend.exchange(RawInput::received("{2024-01-01,2024-01-02}"));
    let grouped_unit: Result<ParsedGroupedTimeScaleUnit, TemporalError> =
        backend.exchange(RawInput::received("2024-24"));
    let formula: Result<ParsedDateTimeFormula, TemporalError> =
        backend.exchange(RawInput::received("EASTER(2024)"));

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
        let err = result.expect_err("the CalConnect/ISO 8601-2 extension family is out of scope");
        assert!(matches!(&**err.kind(), TemporalErrorKind::Unsupported(_)));
    }
}
