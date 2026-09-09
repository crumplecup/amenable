//! Instant-identity propositions: what it takes for a representation to
//! pin down a single fixed instant on the UTC timeline, and where a
//! local date-time fails to.
//!
//! Mixed tier: RFC 3339 / RFC 9557 citations are tier A (verbatim);
//! ISO 8601-1 citations are tier C (`ParaphraseOnly`).

use crate::NormativeQuotation;

temporal_standard! {
    /// A timestamp states its UTC relationship.
    TimestampHasExplicitUtcOffset => (
        document: "RFC 3339",
        section: "4.4",
        body: Ietf,
        status: Normative,
        summary: "a timestamp carries an explicit UTC offset or the Z designator; an unqualified local time does not qualify",
        quotation: NormativeQuotation::verbatim(
            "the interoperability problems of unqualified local time are deemed unacceptable for the Internet."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.4",
        cross_check: ("RFC 3339", "5.6", Normative),
    );

    /// An offset date-time is one fixed instant.
    OffsetDateTimeIdentifiesSingleInstant => (
        document: "ISO 8601-1:2019",
        section: "5.4.2, 5.4.3",
        body: Iso,
        status: Normative,
        summary: "a date-time carrying a UTC offset identifies exactly one instant on the UTC timeline",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("RFC 3339", "5.6", Informative),
    );

    /// A local date-time needs an offset or zone to name an instant.
    LocalDateTimeRequiresZoneOrOffsetForInstant => (
        document: "RFC 3339",
        section: "4.4",
        body: Ietf,
        status: Normative,
        summary: "a date-time with no offset and no zone does not identify an instant and cannot be interoperably interpreted",
        quotation: NormativeQuotation::verbatim(
            "Since interpretation of an unqualified local time zone will fail in approximately 23/24 of the globe, the interoperability problems of unqualified local time are deemed unacceptable for the Internet."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.4",
    );

    /// A local date-time may be ambiguous at a transition.
    LocalDateTimeMayBeAmbiguousAtZoneTransition => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "converting a local date-time to an instant may yield more than one candidate instant near a daylight-saving or offset change",
        quotation: NormativeQuotation::verbatim(
            "some local times may have zero or multiple possible timestamps due to nearby daylight saving time changes or other changes to the UTC offset of that time zone."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// A local date-time may fall in a skipped gap.
    LocalDateTimeMayFallInZoneTransitionGap => (
        document: "RFC 9557",
        section: "1.2",
        body: Ietf,
        status: Normative,
        summary: "converting a local date-time to an instant may yield no candidate instant when the wall clock skips that time at a transition",
        quotation: NormativeQuotation::verbatim(
            "some local times may have zero or multiple possible timestamps due to nearby daylight saving time changes or other changes to the UTC offset of that time zone."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc9557#section-1.2",
    );

    /// Fixed instants are ordered on the UTC timeline.
    UtcTimelineOrderingAppliesToFixedInstants => (
        document: "RFC 3339",
        section: "5.1",
        body: Ietf,
        status: Normative,
        summary: "two fixed instants are totally ordered by their position on the UTC timeline",
        quotation: NormativeQuotation::verbatim(
            "the date and time strings may be sorted as strings (e.g., using the strcmp() function in C) and a time-ordered sequence will result."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.1",
    );
}
