use super::duration::duration_descriptor_to_jiff_span;
use amenable_time::{
    CalendarDateDescriptor, CompleteDateDescriptor, IxdtfTimeZoneAnnotationDescriptor,
    IxdtfTimestampDescriptor, LocalDateTimeDescriptor, LocalTimeDescriptor,
    OffsetDateTimeDescriptor, OrdinalDateDescriptor, QualifiedOrBareTemporalValueDescriptor,
    ReducedCalendarDateDescriptor, ReducedLocalTimeDescriptor, TemporalError, TemporalErrorKind,
    TemporalValueDescriptor, TimeIntervalEndpoint, TimeIntervalRepresentation, UnsupportedSource,
    UtcOffsetDescriptor, UtcOffsetRelationship, UtcOffsetSign, WeekDateDescriptor,
    ZonedDateTimeDescriptor,
};

// ── Formatter (Phase 10) ────────────────────────────────────────────
//
// `TemporalFormatter<JiffVerifier>` is, like `TemporalParser`, ONE
// blanket-impl'd trait over all 36 `Exchange<ParsedX, FormattedX, V>`
// edges. Every edge's INPUT is one of Parser's own already-proven
// `ParsedX` sidecars (data + a validity/proof token), so formatting is
// pure, infallible-except-where-noted text construction from already-
// trusted fields -- no re-validation needed, and (with the exceptions
// below) no fresh jiff calls needed either, since the descriptor's own
// fields already carry the real values Phase 9's parser validated.
// Every `X...FormattedToken` establishes in a single hop directly from
// the matching `ParsedX`'s own token (confirmed the same way Phase 9's
// own parse tokens were: reading `exchange/establish.rs`'s real
// `#[amenable_derive::establish]` attributes).
//
// 24 edges are real: the 9 dual-form (extended/basic) pairs for
// calendar/reduced-calendar/ordinal/week dates, local/reduced-local
// time, UTC offset, and local/offset date-time (18 edges); RFC 3339,
// IXDTF, and IXDTF-zoned timestamps (3 edges, real jiff `Span`/text
// composition); `Duration` (1 edge, real via jiff's own `Span: Display`);
// and `TimeInterval`/`RecurringInterval` (2 edges, real for every
// jiff-representable endpoint form, honestly `Unsupported` for the
// CalConnect/ISO 8601-2 extension forms AND for `Open`/`Unknown`
// boundaries -- a real, deliberate scope decision, not an oversight:
// those two boundary kinds are themselves ISO 8601-2 §10.2 constructs,
// and this backend has no independently sourced basic-ISO-8601 textual
// convention for them to commit to honestly). The other 12 edges are
// the same CalConnect/ISO 8601-2 extension family Parser already
// classified as out of scope, a real, honest `Unsupported`.

/// Report that this backend cannot format the named form.
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
pub(super) fn unsupported_format(edge: &'static str) -> TemporalError {
    TemporalError::new(TemporalErrorKind::Unsupported(UnsupportedSource::new(
        format!(
            "jiff backend cannot format {edge}: outside jiff's Level-1 Gregorian/IANA-zone model"
        ),
    )))
}

/// Zero-pad a signed calendar/week year to at least 4 digits, keeping
/// the sign character separate from the padded magnitude (so e.g.
/// `-5` becomes `"-0005"`, not `"-005"` from a naive `{:04}` on a
/// negative number).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn format_signed_year(year: i32) -> String {
    if year < 0 {
        format!("-{:04}", -year)
    } else {
        format!("{year:04}")
    }
}

/// Format a calendar date as ISO 8601 extended (`YYYY-MM-DD`) or basic
/// (`YYYYMMDD`) text.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_calendar_date(descriptor: &CalendarDateDescriptor, basic: bool) -> String {
    let year = format_signed_year(descriptor.year());
    if basic {
        format!("{year}{:02}{:02}", descriptor.month(), descriptor.day())
    } else {
        format!("{year}-{:02}-{:02}", descriptor.month(), descriptor.day())
    }
}

/// Format an ordinal date as ISO 8601 extended (`YYYY-DDD`) or basic
/// (`YYYYDDD`) text.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_ordinal_date(descriptor: &OrdinalDateDescriptor, basic: bool) -> String {
    let year = format_signed_year(descriptor.year());
    if basic {
        format!("{year}{:03}", descriptor.day_of_year())
    } else {
        format!("{year}-{:03}", descriptor.day_of_year())
    }
}

/// Format a week date as ISO 8601 extended (`YYYY-Www-D`) or basic
/// (`YYYYWwwD`) text.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_week_date(descriptor: &WeekDateDescriptor, basic: bool) -> String {
    let year = format_signed_year(descriptor.week_year());
    if basic {
        format!("{year}W{:02}{}", descriptor.week(), descriptor.weekday())
    } else {
        format!("{year}-W{:02}-{}", descriptor.week(), descriptor.weekday())
    }
}

/// Dispatch a complete date to its own real formatter.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
fn format_complete_date(descriptor: CompleteDateDescriptor, basic: bool) -> String {
    match descriptor {
        CompleteDateDescriptor::Calendar(d) => format_calendar_date(&d, basic),
        CompleteDateDescriptor::Ordinal(d) => format_ordinal_date(&d, basic),
        CompleteDateDescriptor::Week(d) => format_week_date(&d, basic),
    }
}

/// Format a local time as ISO 8601 extended (`HH:MM:SS[.fff]`) or
/// basic (`HHMMSS[.fff]`) text.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_local_time(descriptor: &LocalTimeDescriptor, basic: bool) -> String {
    let mut text = if basic {
        format!(
            "{:02}{:02}{:02}",
            descriptor.hour(),
            descriptor.minute(),
            descriptor.second()
        )
    } else {
        format!(
            "{:02}:{:02}:{:02}",
            descriptor.hour(),
            descriptor.minute(),
            descriptor.second()
        )
    };
    if let Some(fraction) = descriptor.fractional_second() {
        text.push('.');
        text.push_str(fraction.digits());
    }
    text
}

/// Format a reduced-precision calendar date as ISO 8601 extended
/// (`YYYY` / `YYYY-MM`) or basic (`YYYY` / `YYYYMM`) text.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_reduced_calendar_date(
    descriptor: &ReducedCalendarDateDescriptor,
    basic: bool,
) -> String {
    match descriptor {
        ReducedCalendarDateDescriptor::Year { year } => format_signed_year(*year),
        ReducedCalendarDateDescriptor::YearMonth { year, month } => {
            let year = format_signed_year(*year);
            if basic {
                format!("{year}{month:02}")
            } else {
                format!("{year}-{month:02}")
            }
        }
    }
}

/// Format a reduced-precision local time as ISO 8601 extended
/// (`HH` / `HH:MM`) or basic (`HH` / `HHMM`) text. A fractional
/// hour/minute component is a real, honest `Unsupported`: jiff's
/// civil time has no representation for it at all, the same real gap
/// Phase 9's own parser already found and rejected on the way in.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_reduced_local_time(
    descriptor: &ReducedLocalTimeDescriptor,
    basic: bool,
) -> Result<String, TemporalError> {
    match descriptor {
        ReducedLocalTimeDescriptor::Hour {
            hour,
            fractional_component: None,
        } => Ok(format!("{hour:02}")),
        ReducedLocalTimeDescriptor::HourMinute {
            hour,
            minute,
            fractional_component: None,
        } => {
            if basic {
                Ok(format!("{hour:02}{minute:02}"))
            } else {
                Ok(format!("{hour:02}:{minute:02}"))
            }
        }
        _ => Err(unsupported_format(
            "a reduced local time with a fractional hour/minute component",
        )),
    }
}

/// Format a UTC offset as ISO 8601/RFC 9557 extended (`+HH:MM`) or
/// basic (`+HHMM`) text. The unknown-local-offset relationship always
/// emits the RFC 9557 negative-zero convention. A known zero offset
/// always emits the explicit numeric form (`+00:00`/`+0000`), never
/// `"Z"` -- Phase 9's own parser already found that `"Z"` and
/// `"+00:00"` collapse to the identical descriptor, so there is no
/// surviving information to choose `"Z"` back over the numeric form; a
/// real, honest round-trip-is-not-exact finding, not a bug.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_utc_offset(descriptor: &UtcOffsetDescriptor, basic: bool) -> String {
    if descriptor.relationship() == UtcOffsetRelationship::UnknownLocalOffset {
        return if basic {
            "-0000".to_owned()
        } else {
            "-00:00".to_owned()
        };
    }
    let sign = match descriptor.sign() {
        UtcOffsetSign::Positive => '+',
        UtcOffsetSign::Negative => '-',
    };
    match descriptor.minutes() {
        None => format!("{sign}{:02}", descriptor.hours()),
        Some(minutes) if basic => format!("{sign}{:02}{minutes:02}", descriptor.hours()),
        Some(minutes) => format!("{sign}{:02}:{minutes:02}", descriptor.hours()),
    }
}

/// Format a local date-time as `<date>T<time>`, both in the same
/// extended-or-basic form.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_local_date_time(descriptor: &LocalDateTimeDescriptor, basic: bool) -> String {
    format!(
        "{}T{}",
        format_complete_date(descriptor.date(), basic),
        format_local_time(descriptor.time(), basic)
    )
}

/// Format an offset date-time as `<local-date-time><offset>`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_offset_date_time(
    descriptor: &OffsetDateTimeDescriptor,
    basic: bool,
) -> String {
    let offset = descriptor.offset();
    format!(
        "{}{}",
        format_local_date_time(descriptor.local(), basic),
        format_utc_offset(&offset, basic)
    )
}

/// Format a zoned date-time as `<offset-date-time>[<zone>]` -- always
/// extended form, matching RFC 9557's own IXDTF grammar (no basic
/// form defined for zone-annotated timestamps).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_zoned_date_time(descriptor: &ZonedDateTimeDescriptor) -> String {
    format!(
        "{}[{}]",
        format_offset_date_time(descriptor.timestamp(), false),
        descriptor.zone().identifier()
    )
}

/// Format one additional IXDTF suffix annotation as `[!key=v1-v2]`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(annotation)))]
fn format_ixdtf_annotation(annotation: &amenable_time::IxdtfAnnotationDescriptor) -> String {
    let critical = if annotation.critical() { "!" } else { "" };
    format!(
        "[{critical}{}={}]",
        annotation.key(),
        annotation.values().join("-")
    )
}

/// Format a real IXDTF timestamp: the base offset date-time, an
/// optional zone annotation, then any additional suffix annotations in
/// source order.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_ixdtf_timestamp(descriptor: &IxdtfTimestampDescriptor) -> String {
    let mut text = format_offset_date_time(descriptor.timestamp(), false);
    if let Some(zone) = descriptor.time_zone_annotation() {
        text.push('[');
        match zone {
            IxdtfTimeZoneAnnotationDescriptor::Named(named) => text.push_str(named.identifier()),
            IxdtfTimeZoneAnnotationDescriptor::Offset(offset) => {
                text.push_str(&format_utc_offset(offset, false));
            }
        }
        text.push(']');
    }
    for annotation in descriptor.additional_annotations() {
        text.push_str(&format_ixdtf_annotation(annotation));
    }
    text
}

/// Dispatch a bare temporal value to its own real formatter, matching
/// exactly the jiff-representable form set Phase 8's own endpoint
/// conversion already established (calendar/ordinal/week dates fold
/// through their own real formatters; local/offset/zoned date-times
/// keep their own). Every other form is the CalConnect/ISO 8601-2
/// extension family, a real, honest `Unsupported`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(value)))]
fn format_temporal_value(value: &TemporalValueDescriptor) -> Result<String, TemporalError> {
    match value {
        TemporalValueDescriptor::CalendarDate(d) => Ok(format_calendar_date(d, false)),
        TemporalValueDescriptor::OrdinalDate(d) => Ok(format_ordinal_date(d, false)),
        TemporalValueDescriptor::WeekDate(d) => Ok(format_week_date(d, false)),
        TemporalValueDescriptor::LocalDateTime(d) => Ok(format_local_date_time(d, false)),
        TemporalValueDescriptor::OffsetDateTime(d) => Ok(format_offset_date_time(d, false)),
        TemporalValueDescriptor::ZonedDateTime(d) => Ok(format_zoned_date_time(d)),
        other => Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(format!(
                "jiff backend cannot format the {other:?} interval-endpoint form \
             (ISO 8601-2 / CalConnect extension family, out of scope)"
            )),
        ))),
    }
}

/// Format one time-interval endpoint. `Open`/`Unknown` are a real,
/// deliberate `Unsupported`: both are themselves ISO 8601-2 §10.2
/// constructs (confirmed by reading `contracts/extended/qualification.rs`'s
/// own real citations), and this backend has no independently sourced
/// basic-ISO-8601 textual convention for either to commit to honestly
/// -- guessing one (e.g. `".."`) would be exactly the kind of
/// unsourced assumption this whole backend has avoided everywhere
/// else.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(endpoint)))]
fn format_time_interval_endpoint(endpoint: &TimeIntervalEndpoint) -> Result<String, TemporalError> {
    match endpoint {
        TimeIntervalEndpoint::Value(QualifiedOrBareTemporalValueDescriptor::Bare(value)) => {
            format_temporal_value(value)
        }
        TimeIntervalEndpoint::Value(QualifiedOrBareTemporalValueDescriptor::Qualified(_)) => Err(
            unsupported_format("an explicitly qualified (ISO 8601-2) interval-endpoint value"),
        ),
        TimeIntervalEndpoint::Open | TimeIntervalEndpoint::Unknown => Err(unsupported_format(
            "an Open/Unknown interval boundary (no independently sourced textual convention)",
        )),
    }
}

/// Format a time-interval representation as `<start>/<end>`,
/// `<start>/<duration>`, or `<duration>/<end>`.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(representation)))]
pub(super) fn format_time_interval_representation(
    representation: &TimeIntervalRepresentation,
) -> Result<String, TemporalError> {
    match representation {
        TimeIntervalRepresentation::StartEnd { start, end } => Ok(format!(
            "{}/{}",
            format_time_interval_endpoint(start)?,
            format_time_interval_endpoint(end)?
        )),
        TimeIntervalRepresentation::StartDuration { start, duration } => Ok(format!(
            "{}/{}",
            format_time_interval_endpoint(start)?,
            duration_descriptor_to_jiff_span(duration)?
        )),
        TimeIntervalRepresentation::DurationEnd { duration, end } => Ok(format!(
            "{}/{}",
            duration_descriptor_to_jiff_span(duration)?,
            format_time_interval_endpoint(end)?
        )),
    }
}

/// Format a recurring interval as `R[n]/<interval>` (an absent
/// repetition count denotes ISO 8601's unbounded `R/` form).
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(descriptor)))]
pub(super) fn format_recurring_interval(
    descriptor: &amenable_time::RecurringIntervalDescriptor,
) -> Result<String, TemporalError> {
    let repetitions = descriptor
        .repetitions()
        .map(|n| n.to_string())
        .unwrap_or_default();
    let interval = format_time_interval_representation(descriptor.interval().representation())?;
    Ok(format!("R{repetitions}/{interval}"))
}
