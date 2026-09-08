//! Numeric UTC offsets and UTC-difference expressions (ISO/WD
//! 8601-1:2016(E) §4.2.5), plus the trailing-Z UTC-of-day rule. See
//! [`super::definitions`] for conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A UTC offset has a sign, an hour, and an optional minute.
    UtcOffsetCarriesSignHourAndOptionalMinute => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.5.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a numeric UTC offset carries a sign, an hour component, and optionally a minute component",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A UTC-offset hour is 00 through 23.
    UtcOffsetHourInRangeZeroToTwentyThree => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.5.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the hour component of a UTC offset lies in the inclusive range 00 to 23",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A UTC-offset minute is 00 through 59.
    UtcOffsetMinuteInRangeZeroToFiftyNine => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.5.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the minute component of a UTC offset lies in the inclusive range 00 to 59",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// UTC-difference minutes are omitted only for whole-hour offsets.
    UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.5.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the minute component of a UTC difference may be omitted only when the offset is a whole number of hours",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// The UTC-difference sign shows direction relative to UTC.
    UtcDifferenceSignEncodesDirectionRelativeToUtc => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.5.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the sign of a UTC difference indicates whether local time is ahead of (+) or behind (-) UTC",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A UTC difference attaches with no separating space.
    UtcDifferenceAppendedImmediatelyWithoutSpace => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.5.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a UTC difference is appended directly to the local-time expression, with no intervening space",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A UTC-difference expression cannot stand alone.
    UtcDifferenceExpressionIsNotSelfStanding => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.5.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a UTC-difference expression is a qualifier on a time expression, never a representation in its own right",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A UTC-of-day representation ends immediately with Z.
    UtcOfDayUsesTrailingZuluDesignatorImmediately => (
        document: "ISO 8601-1:2019",
        section: "4.2.4",
        body: Iso,
        status: Normative,
        summary: "a UTC-of-day representation is a local-time-of-day representation followed immediately by the Z designator",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );
}
