use crate::jiff::backend::instant::jiff_offset_to_utc_offset_descriptor;
use crate::jiff::backend::instant::utc_offset_descriptor_to_jiff_offset;
use amenable_time::{
    ComponentNarrowingFailedSource, IxdtfTimeZoneAnnotationDescriptor, JiffParseRejectedSource,
    JiffTimeConstructionFailedSource, NamedTimeZoneBuilderFailedSource,
    NamedTimeZoneDescriptorBuilder, NarrowingComponent, ParseIntRejectedSource,
    ParseProfile, ParseRejectedSource, ParseRejectionReason, ReducedCalendarDateDescriptor,
    ReducedLocalTimeDescriptor, TemporalError, TemporalErrorKind, UnsupportedCase,
    UnsupportedDetail, UnsupportedSource, UtcOffsetBuilderFailedSource, UtcOffsetDescriptor,
    UtcOffsetDescriptorBuilder, UtcOffsetRelationship, UtcOffsetSign,
};

// ── Parser (Phase 9) ─────────────────────────────────────────────────
//
// `TemporalParser<JiffVerifier>` is ONE blanket-impl'd supertrait
// bundle over all 24 `Exchange<RawInput, ParsedX, JiffVerifier>` edges
// -- unlike every prior family, there is no way to claim a subset;
// every edge needs a real `Exchange` impl before the trait holds at
// all. 13 are real jiff-backed parses (calendar/ordinal/week dates,
// local time, the two reduced-precision forms Phase 3's own carriers
// anticipated wiring here, UTC offset, local/offset date-time, RFC
// 3339, IXDTF). The other 11 are the CalConnect/ISO 8601-2 extension
// family (plus `TimeInterval`, per Phase 7's own finding) -- a real,
// honest `Unsupported`, not a stand-in, mirroring the std canary's own
// `unsupported_parse` pattern for exactly this reason.
//
// Every parse proposition (`CalendarDateValid`, `OffsetDateTimeProof`,
// ...) establishes in a SINGLE hop directly from `TemporalInputToken`
// (confirmed by reading `exchange/establish.rs`'s own module doc
// comment) -- so every real edge below shares the same two-line
// Establish tail, no multi-hop chain-walking needed anywhere in this
// section.

/// Report that this backend cannot parse the named form -- the
/// CalConnect/ISO 8601-2 extension family (plus `TimeInterval`, which
/// needs the general endpoint parser this backend still lacks).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(case)))]
pub(super) fn unsupported_parse(case: UnsupportedCase) -> TemporalError {
    TemporalError::new(TemporalErrorKind::Unsupported(UnsupportedSource::closed(
        case,
    )))
}

/// Parse an ISO 8601 ordinal date string (`"YYYY-DDD"` or `"YYYYDDD"`)
/// into its year/day-of-year components. jiff's own `Date::from_str`
/// only accepts the calendar form (confirmed by reading its real
/// grammar/doctests) -- this is genuine hand-rolled digit-splitting,
/// not a jiff gap, exactly as the plan doc's own Phase 9 row predicted.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub(super) fn parse_ordinal_date_text(text: &str) -> Result<(i32, u16), TemporalError> {
    let reject = |reason: ParseRejectionReason| {
        TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
            ParseProfile::Iso8601OrdinalDate,
            reason,
            text,
        )))
    };
    if !text.is_ascii() {
        return Err(reject(ParseRejectionReason::NotAscii));
    }
    let (year_range, day_range) = match text.len() {
        7 => (0..4, 4..7),
        8 if text.as_bytes().get(4) == Some(&b'-') => (0..4, 5..8),
        _ => {
            return Err(reject(ParseRejectionReason::NotSevenOrEightCharOrdinalDate));
        }
    };
    let year_str = text
        .get(year_range)
        .ok_or_else(|| reject(ParseRejectionReason::InvalidYearSlice))?;
    let day_str = text
        .get(day_range)
        .ok_or_else(|| reject(ParseRejectionReason::InvalidDayOfYearSlice))?;
    let year = year_str.parse::<i32>().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601OrdinalDate, err),
        ))
    })?;
    let day_of_year = day_str.parse::<u16>().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601OrdinalDate, err),
        ))
    })?;
    Ok((year, day_of_year))
}

/// Parse an ISO 8601 week date string (`"YYYY-Www-D"` or `"YYYYWwwD"`)
/// into its week-year/week/weekday components. Same real gap as
/// [`parse_ordinal_date_text`]: no jiff `FromStr` accepts this text
/// form at all.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub(super) fn parse_week_date_text(text: &str) -> Result<(i32, u8, u8), TemporalError> {
    let reject = |reason: ParseRejectionReason| {
        TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
            ParseProfile::Iso8601WeekDate,
            reason,
            text,
        )))
    };
    if !text.is_ascii() {
        return Err(reject(ParseRejectionReason::NotAscii));
    }
    let bytes = text.as_bytes();
    let (year_range, week_range, weekday_range) = match text.len() {
        8 if bytes.get(4) == Some(&b'W') => (0..4, 5..7, 7..8),
        10 if bytes.get(4) == Some(&b'-')
            && bytes.get(5) == Some(&b'W')
            && bytes.get(8) == Some(&b'-') =>
        {
            (0..4, 6..8, 9..10)
        }
        _ => {
            return Err(reject(ParseRejectionReason::NotEightOrTenCharWeekDate));
        }
    };
    let year_str = text
        .get(year_range)
        .ok_or_else(|| reject(ParseRejectionReason::InvalidWeekYearSlice))?;
    let week_str = text
        .get(week_range)
        .ok_or_else(|| reject(ParseRejectionReason::InvalidWeekNumberSlice))?;
    let weekday_str = text
        .get(weekday_range)
        .ok_or_else(|| reject(ParseRejectionReason::InvalidWeekdaySlice))?;
    let week_year = year_str.parse::<i32>().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601WeekDate, err),
        ))
    })?;
    let week = week_str.parse::<u8>().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601WeekDate, err),
        ))
    })?;
    let weekday = weekday_str.parse::<u8>().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601WeekDate, err),
        ))
    })?;
    Ok((week_year, week, weekday))
}

/// Parse a bare UTC offset string (`"Z"`, `"+HH:MM"`, `"-HHMM"`,
/// `"+HH"`, ...) into a [`UtcOffsetDescriptor`]. `jiff::tz::Offset` has
/// no `FromStr` of its own at all (confirmed by grepping jiff's real
/// source for an `impl FromStr for Offset` and finding none) -- a real
/// jiff gap, not an oversight, exactly as the plan doc's own Phase 9
/// row predicted. `"-00:00"`/`"-0000"`/`"-00"` (negative zero) map to
/// RFC 9557's unknown-local-offset relationship; `"Z"` maps to a known
/// zero offset.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub(super) fn parse_utc_offset_text(text: &str) -> Result<UtcOffsetDescriptor, TemporalError> {
    let reject = |reason: ParseRejectionReason| {
        TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
            ParseProfile::Iso8601Rfc9557UtcOffset,
            reason,
            text,
        )))
    };
    if text.eq_ignore_ascii_case("z") {
        return UtcOffsetDescriptorBuilder::default()
            .sign(UtcOffsetSign::Positive)
            .hours(0u8)
            .build()
            .map_err(|err| {
                TemporalError::new(TemporalErrorKind::UtcOffsetBuilderFailed(
                    UtcOffsetBuilderFailedSource::new(err),
                ))
            });
    }
    if !text.is_ascii() || text.len() < 3 {
        return Err(reject(ParseRejectionReason::TooShortForUtcOffset));
    }
    let sign = match text.as_bytes()[0] {
        b'+' => UtcOffsetSign::Positive,
        b'-' => UtcOffsetSign::Negative,
        _ => return Err(reject(ParseRejectionReason::MustStartWithSignOrZ)),
    };
    let rest = text
        .get(1..)
        .ok_or_else(|| reject(ParseRejectionReason::TooShortForUtcOffset))?;
    let (hours_str, minutes_str) = if let Some(idx) = rest.find(':') {
        (
            rest.get(..idx)
                .ok_or_else(|| reject(ParseRejectionReason::InvalidOffsetHoursSlice))?,
            Some(
                rest.get(idx + 1..)
                    .ok_or_else(|| reject(ParseRejectionReason::InvalidOffsetMinutesSlice))?,
            ),
        )
    } else if rest.len() == 4 {
        (
            rest.get(..2)
                .ok_or_else(|| reject(ParseRejectionReason::InvalidOffsetHoursSlice))?,
            Some(
                rest.get(2..)
                    .ok_or_else(|| reject(ParseRejectionReason::InvalidOffsetMinutesSlice))?,
            ),
        )
    } else {
        (rest, None)
    };
    let hours: u8 = hours_str.parse().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601Rfc9557UtcOffset, err),
        ))
    })?;
    let minutes: Option<u8> = match minutes_str {
        None => None,
        Some(m) => Some(m.parse().map_err(|err| {
            TemporalError::new(TemporalErrorKind::ParseIntRejected(
                ParseIntRejectedSource::new(ParseProfile::Iso8601Rfc9557UtcOffset, err),
            ))
        })?),
    };
    let mut builder = UtcOffsetDescriptorBuilder::default()
        .sign(sign)
        .hours(hours);
    if let Some(minutes) = minutes {
        builder = builder.minutes(minutes);
    }
    if sign == UtcOffsetSign::Negative && hours == 0 && minutes.unwrap_or(0) == 0 {
        builder = builder.relationship(UtcOffsetRelationship::UnknownLocalOffset);
    }
    let descriptor = builder.build().map_err(|err| {
        TemporalError::new(TemporalErrorKind::UtcOffsetBuilderFailed(
            UtcOffsetBuilderFailedSource::new(err),
        ))
    })?;
    // Real validation: reuse Phase 2's own offset conversion as the
    // range oracle (rejects e.g. hours = 99), rather than re-deriving
    // jiff's own component bounds by hand. Skipped for the unknown-
    // local-offset case: Phase 2 already established that jiff::tz::
    // Offset has NO representation for it at all, so that oracle would
    // always reject a descriptor this parse edge's own job is just to
    // recognize as lawfully PARSED text -- "not realizable by this
    // backend" is a distinct, later-stage limitation, not a parse
    // failure.
    if descriptor.relationship() != UtcOffsetRelationship::UnknownLocalOffset {
        utc_offset_descriptor_to_jiff_offset(descriptor)?;
    }
    Ok(descriptor)
}

/// Parse a reduced-precision calendar date string (`"YYYY"` or
/// `"YYYY-MM"`) into a [`ReducedCalendarDateDescriptor`]. The real
/// carrier this feeds (`JiffReducedCalendarDate`) was built in Phase 3
/// but deliberately left unwired until a real edge needed it -- this
/// is that edge.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub(super) fn parse_reduced_calendar_date_text(
    text: &str,
) -> Result<ReducedCalendarDateDescriptor, TemporalError> {
    let reject = |reason: ParseRejectionReason| {
        TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
            ParseProfile::Iso8601ReducedPrecisionCalendarDate,
            reason,
            text,
        )))
    };
    if !text.is_ascii() {
        return Err(reject(ParseRejectionReason::NotAscii));
    }
    if let Some((year_str, month_str)) = text.split_once('-') {
        let year: i32 = year_str.parse().map_err(|err| {
            TemporalError::new(TemporalErrorKind::ParseIntRejected(
                ParseIntRejectedSource::new(ParseProfile::Iso8601ReducedPrecisionCalendarDate, err),
            ))
        })?;
        let month: u8 = month_str.parse().map_err(|err| {
            TemporalError::new(TemporalErrorKind::ParseIntRejected(
                ParseIntRejectedSource::new(ParseProfile::Iso8601ReducedPrecisionCalendarDate, err),
            ))
        })?;
        i16::try_from(year).map_err(|err| {
            TemporalError::new(TemporalErrorKind::ComponentNarrowingFailed(
                ComponentNarrowingFailedSource::new(NarrowingComponent::Year, i64::from(year), err),
            ))
        })?;
        if !(1..=12).contains(&month) {
            return Err(reject(ParseRejectionReason::MonthOutOfRange(month)));
        }
        return Ok(ReducedCalendarDateDescriptor::YearMonth { year, month });
    }
    let year: i32 = text.parse().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601ReducedPrecisionCalendarDate, err),
        ))
    })?;
    i16::try_from(year).map_err(|err| {
        TemporalError::new(TemporalErrorKind::ComponentNarrowingFailed(
            ComponentNarrowingFailedSource::new(NarrowingComponent::Year, i64::from(year), err),
        ))
    })?;
    Ok(ReducedCalendarDateDescriptor::Year { year })
}

/// Parse a reduced-precision local time string (`"HH"`, `"HH:MM"`, or
/// `"HHMM"`) into a [`ReducedLocalTimeDescriptor`]. A fractional suffix
/// on the hour/minute component (e.g. `"12,5"`) is a real, honest
/// `Unsupported` -- `jiff::civil::Time` has no representation for it at
/// all, exactly as Phase 3's own doc comment on `JiffReducedLocalTime`
/// anticipated.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub(super) fn parse_reduced_local_time_text(
    text: &str,
) -> Result<ReducedLocalTimeDescriptor, TemporalError> {
    let reject = |reason: ParseRejectionReason| {
        TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
            ParseProfile::Iso8601ReducedPrecisionLocalTime,
            reason,
            text,
        )))
    };
    if !text.is_ascii() {
        return Err(reject(ParseRejectionReason::NotAscii));
    }
    if text.contains(['.', ',']) {
        #[cfg(not(kani))]
        let detail = UnsupportedDetail::FractionalHourMinuteComponent(text.to_owned());
        #[cfg(kani)]
        let detail = UnsupportedDetail::FractionalHourMinuteComponent;
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(detail),
        )));
    }
    let (hour_str, minute_str) = if let Some((h, m)) = text.split_once(':') {
        (h, Some(m.to_owned()))
    } else if text.len() == 4 && text.bytes().all(|b| b.is_ascii_digit()) {
        (
            text.get(0..2)
                .ok_or_else(|| reject(ParseRejectionReason::InvalidHourSlice))?,
            Some(
                text.get(2..4)
                    .ok_or_else(|| reject(ParseRejectionReason::InvalidMinuteSlice))?
                    .to_owned(),
            ),
        )
    } else {
        (text, None)
    };
    let hour: u8 = hour_str.parse().map_err(|err| {
        TemporalError::new(TemporalErrorKind::ParseIntRejected(
            ParseIntRejectedSource::new(ParseProfile::Iso8601ReducedPrecisionLocalTime, err),
        ))
    })?;
    match minute_str {
        None => {
            jiff::civil::Time::new(i8::try_from(hour).unwrap_or(i8::MAX), 0, 0, 0).map_err(
                |err| {
                    TemporalError::new(TemporalErrorKind::JiffTimeConstructionFailed(
                        JiffTimeConstructionFailedSource::new(err),
                    ))
                },
            )?;
            Ok(ReducedLocalTimeDescriptor::Hour {
                hour,
                fractional_component: None,
            })
        }
        Some(minute_str) => {
            let minute: u8 = minute_str.parse().map_err(|err| {
                TemporalError::new(TemporalErrorKind::ParseIntRejected(
                    ParseIntRejectedSource::new(ParseProfile::Iso8601ReducedPrecisionLocalTime, err),
                ))
            })?;
            jiff::civil::Time::new(
                i8::try_from(hour).unwrap_or(i8::MAX),
                i8::try_from(minute).unwrap_or(i8::MAX),
                0,
                0,
            )
            .map_err(|err| {
                TemporalError::new(TemporalErrorKind::JiffTimeConstructionFailed(
                    JiffTimeConstructionFailedSource::new(err),
                ))
            })?;
            Ok(ReducedLocalTimeDescriptor::HourMinute {
                hour,
                minute,
                fractional_component: None,
            })
        }
    }
}

/// Parse text into jiff's real `Pieces` decomposition, wrapping any
/// parse failure under `profile`.
#[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(profile)))]
pub(super) fn parse_temporal_pieces<'i>(
    text: &'i str,
    profile: ParseProfile,
) -> Result<jiff::fmt::temporal::Pieces<'i>, TemporalError> {
    jiff::fmt::temporal::Pieces::parse(text).map_err(|err| {
        TemporalError::new(TemporalErrorKind::JiffParseRejected(
            JiffParseRejectedSource::new(profile, err),
        ))
    })
}

/// Decompose real jiff `Pieces` into a local date-time + offset pair,
/// requiring both a time-of-day and a UTC offset to be present (an
/// offset with no time makes no sense under any of the profiles this
/// helper backs).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(pieces, profile)))]
pub(super) fn pieces_to_offset_date_time(
    pieces: &jiff::fmt::temporal::Pieces<'_>,
    text: &str,
    profile: ParseProfile,
) -> Result<(jiff::civil::DateTime, jiff::tz::Offset), TemporalError> {
    let reject = |reason: ParseRejectionReason| {
        TemporalError::new(TemporalErrorKind::ParseRejected(ParseRejectedSource::new(
            profile, reason, text,
        )))
    };
    let time = pieces
        .time()
        .ok_or_else(|| reject(ParseRejectionReason::NoTimeOfDayComponent))?;
    let offset = pieces
        .offset()
        .ok_or_else(|| reject(ParseRejectionReason::NoUtcOffset))?;
    Ok((
        jiff::civil::DateTime::from_parts(pieces.date(), time),
        offset.to_numeric_offset(),
    ))
}

/// Reject a zone annotation on `pieces` -- neither plain ISO 8601
/// offset date-times nor RFC 3339 timestamps have `[...]` zone-bracket
/// syntax at all; only RFC 9557 IXDTF does.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(pieces, profile)))]
pub(super) fn reject_zone_annotation(
    pieces: &jiff::fmt::temporal::Pieces<'_>,
    text: &str,
    profile: ParseProfile,
) -> Result<(), TemporalError> {
    if pieces.time_zone_annotation().is_some() {
        return Err(TemporalError::new(TemporalErrorKind::ParseRejected(
            ParseRejectedSource::new(profile, ParseRejectionReason::CarriesZoneAnnotation, text),
        )));
    }
    Ok(())
}

/// Decompose a real jiff time-zone annotation into a neutral IXDTF
/// zone-annotation descriptor. The annotation's own name is not
/// validated against the real IANA tzdb here -- jiff's own IXDTF
/// grammar treats the annotation as descriptive metadata, not a
/// zone-identity claim the instant depends on (confirmed by its own
/// doctest accepting `"Australia/Bluey"`, not a real zone).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(annotation)))]
pub(super) fn jiff_time_zone_annotation_to_descriptor(
    annotation: &jiff::fmt::temporal::TimeZoneAnnotation<'_>,
) -> Result<IxdtfTimeZoneAnnotationDescriptor, TemporalError> {
    match annotation.kind() {
        jiff::fmt::temporal::TimeZoneAnnotationKind::Named(name) => {
            let descriptor = NamedTimeZoneDescriptorBuilder::default()
                .identifier(name.as_str())
                .build()
                .map_err(|err| {
                    TemporalError::new(TemporalErrorKind::NamedTimeZoneBuilderFailed(
                        NamedTimeZoneBuilderFailedSource::new(err),
                    ))
                })?;
            Ok(IxdtfTimeZoneAnnotationDescriptor::Named(descriptor))
        }
        jiff::fmt::temporal::TimeZoneAnnotationKind::Offset(offset) => {
            let descriptor = jiff_offset_to_utc_offset_descriptor(*offset)?;
            Ok(IxdtfTimeZoneAnnotationDescriptor::Offset(descriptor))
        }
        other => Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(UnsupportedDetail::UnrecognizedTimeZoneAnnotationKind(
                other.clone().into_owned(),
            )),
        ))),
    }
}
