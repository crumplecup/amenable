//! RFC 3339 (*Date and Time on the Internet: Timestamps*) propositions.
//!
//! **Tier A** (`docs/AMENABLE_TIME_PLAN.md`): RFC text is freely
//! redistributable under the IETF Trust's rules, so every contract here
//! embeds the specific clause verbatim via
//! `NormativeQuotation::verbatim(..)` and links the section directly.

use crate::NormativeQuotation;

temporal_standard! {
    /// An RFC 3339 timestamp uses a four-digit year.
    Rfc3339UsesFourDigitYear => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "the year field of an RFC 3339 timestamp is exactly four digits",
        quotation: NormativeQuotation::verbatim("date-fullyear   = 4DIGIT"),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );

    /// An RFC 3339 timestamp's date part is `full-date`.
    Rfc3339UsesExtendedCalendarDate => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "the date part is the ISO 8601 extended calendar-date form YYYY-MM-DD",
        quotation: NormativeQuotation::verbatim(
            "full-date       = date-fullyear \"-\" date-month \"-\" date-mday"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );

    /// An RFC 3339 timestamp's time part is `full-time`.
    Rfc3339UsesFullTime => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "the time part is partial-time plus a mandatory time-offset, joined to the date with T",
        quotation: NormativeQuotation::verbatim(
            "full-time       = partial-time time-offset\ndate-time       = full-date \"T\" full-time"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );

    /// An RFC 3339 timestamp carries no weekday.
    Rfc3339TimestampExcludesRedundantWeekdayInformation => (
        document: "RFC 3339",
        section: "5.4",
        body: Ietf,
        status: Normative,
        summary: "the day of week is omitted because it is redundant with — and could contradict — the date",
        quotation: NormativeQuotation::verbatim(
            "Since it is not difficult to compute the day of week from a date (see Appendix B), the day of week should not be included in a date/time format."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.4",
    );

    /// An RFC 3339 timestamp states its relationship to UTC.
    Rfc3339RequiresUtcRelationship => (
        document: "RFC 3339",
        section: "4.4",
        body: Ietf,
        status: Normative,
        summary: "every timestamp is fully qualified with respect to UTC; an unqualified local time is not admissible",
        quotation: NormativeQuotation::verbatim(
            "the interoperability problems of unqualified local time are deemed unacceptable for the Internet."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.4",
    );

    /// Unqualified local time is forbidden.
    Rfc3339UnqualifiedLocalTimeForbidden => (
        document: "RFC 3339",
        section: "4.4",
        body: Ietf,
        status: Normative,
        summary: "a time with no UTC offset or Z designator is rejected",
        quotation: NormativeQuotation::verbatim(
            "Since interpretation of an unqualified local time zone will fail in approximately 23/24 of the globe, the interoperability problems of unqualified local time are deemed unacceptable for the Internet."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.4",
    );

    /// Fractional seconds use a dot.
    Rfc3339FractionUsesDotSeparator => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "the fractional-second separator is the full stop",
        quotation: NormativeQuotation::verbatim("time-secfrac    = \".\" 1*DIGIT"),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );

    /// The UTC relationship is `Z` or a numeric offset.
    Rfc3339OffsetIsUtcOrNumeric => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "time-offset is either the Z designator or a signed hour:minute numeric offset",
        quotation: NormativeQuotation::verbatim(
            "time-numoffset  = (\"+\" / \"-\") time-hour \":\" time-minute\ntime-offset     = \"Z\" / time-numoffset"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );

    /// The unknown-local-offset convention is `-00:00`.
    Rfc3339UnknownLocalOffsetUsesZuluDesignator => (
        document: "RFC 3339",
        section: "4.3",
        body: Ietf,
        status: Normative,
        summary: "when UTC is known but the local offset is not, RFC 3339 uses -00:00; RFC 9557 §2.2 revisits this convention",
        quotation: NormativeQuotation::verbatim(
            "If the time in UTC is known, but the offset to local time is unknown, this can be represented with an offset of \"-00:00\"."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.3",
        cross_check: ("RFC 9557", "2.2", Normative),
    );

    /// A timestamp not using the `-00:00` convention.
    Rfc3339LocalOffsetNotUnknown => (
        document: "RFC 3339",
        section: "4.3",
        body: Ietf,
        status: Normative,
        summary: "a Z or +00:00 offset means the local offset is not being declared unknown",
        quotation: NormativeQuotation::verbatim(
            "This differs semantically from an offset of \"Z\" or \"+00:00\", which imply that UTC is the preferred reference point for the specified time."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.3",
        cross_check: ("RFC 9557", "2.2", Normative),
    );

    /// `+00:00` declares UTC as the preferred reference point.
    Rfc3339PositiveZeroOffsetDeclaresPreferredUtcReferencePoint => (
        document: "RFC 3339",
        section: "4.3",
        body: Ietf,
        status: Normative,
        summary: "a +00:00 (or Z) offset asserts UTC is the intended reference point, unlike the -00:00 unknown-offset form",
        quotation: NormativeQuotation::verbatim(
            "This differs semantically from an offset of \"Z\" or \"+00:00\", which imply that UTC is the preferred reference point for the specified time."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-4.3",
        cross_check: ("RFC 9557", "2.2", Normative),
    );

    /// Lexical ordering needs a uniform UTC-relationship string.
    Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding => (
        document: "RFC 3339",
        section: "5.1",
        body: Ietf,
        status: Normative,
        summary: "string-sorting timestamps yields time order only if every timestamp uses the same UTC-relationship spelling (all Z, or all +00:00)",
        quotation: NormativeQuotation::verbatim(
            "Assuming that the time zones of the dates and times are the same (e.g., all in UTC), expressed using the same string (e.g., all \"Z\" or all \"+00:00\"), and all times have the same number of fractional second digits, then the date and time strings may be sorted as strings"
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.1",
    );

    /// Lexical ordering needs a uniform fractional-digit count.
    Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits => (
        document: "RFC 3339",
        section: "5.1",
        body: Ietf,
        status: Normative,
        summary: "string-sorting timestamps yields time order only if every timestamp has the same number of fractional-second digits",
        quotation: NormativeQuotation::verbatim(
            "all times have the same number of fractional second digits, then the date and time strings may be sorted as strings (e.g., using the strcmp() function in C) and a time-ordered sequence will result."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.1",
    );

    /// The profile makes most fields and punctuation mandatory.
    Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory => (
        document: "RFC 3339",
        section: "5.5",
        body: Ietf,
        status: Normative,
        summary: "the RFC 3339 profile of ISO 8601 keeps simplicity by requiring nearly every field and separator",
        quotation: NormativeQuotation::verbatim(
            "Simplicity is achieved by making most fields and punctuation mandatory."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.5",
    );

    /// Fractional seconds are the profile's one rarely-used option.
    Rfc3339FractionalSecondsAreOnlyRarelyUsedOption => (
        document: "RFC 3339",
        section: "5.3",
        body: Ietf,
        status: Normative,
        summary: "the only optional element the profile keeps is the fractional second",
        quotation: NormativeQuotation::verbatim(
            "The format defined below includes only one rarely used option: fractions of a second."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.3",
    );

    /// Clients should localise dates for display.
    Rfc3339ClientsShouldTransformDatesForLocalityDisplay => (
        document: "RFC 3339",
        section: "5.2",
        body: Ietf,
        status: Normative,
        summary: "display code should convert an RFC 3339 timestamp into a locally appropriate presentation",
        quotation: NormativeQuotation::verbatim(
            "Internet clients SHOULD be prepared to transform dates into a display format suitable for the locality."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.2",
    );

    /// Locality display may translate UTC to local time.
    Rfc3339LocalityDisplayMayTranslateUtcToLocalTime => (
        document: "RFC 3339",
        section: "5.2",
        body: Ietf,
        status: Normative,
        summary: "the locality display transformation may include converting UTC to the viewer's local time",
        quotation: NormativeQuotation::verbatim("This may include translating UTC to local time."),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.2",
    );

    /// Generators should use uppercase `T` and `Z`.
    Rfc3339GeneratorsShouldUseUppercaseTAndZ => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "although lowercase t and z parse, a generator should emit uppercase",
        quotation: NormativeQuotation::verbatim(
            "Applications that generate this format SHOULD use upper case letters."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );

    /// Applications may accept a space date/time separator.
    Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator => (
        document: "RFC 3339",
        section: "5.6",
        body: Ietf,
        status: Normative,
        summary: "an application may, for readability, accept a space in place of the T between the date and time parts",
        quotation: NormativeQuotation::verbatim(
            "Applications using this syntax may choose, for the sake of readability, to specify a full-date and full-time separated by (say) a space character."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.6",
    );

    /// Inserted leap seconds are generated only after announcement.
    Rfc3339LeapSecondGenerationRequiresPriorAnnouncement => (
        document: "RFC 3339",
        section: "5.7",
        body: Ietf,
        status: Normative,
        summary: "a generator does not emit a :60 leap-second timestamp until the IERS has announced that leap second",
        quotation: NormativeQuotation::verbatim(
            "Applications should not generate timestamps involving inserted leap seconds until after the leap seconds are announced."
        ),
        url: "https://www.rfc-editor.org/rfc/rfc3339#section-5.7",
    );
}
