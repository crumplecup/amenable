//! Time of day — hour/minute/second structure and ranges, the 24:00
//! end-of-day rules, fractions, leap seconds, and the local / standard /
//! UTC time-scale concepts. See [`super::definitions`] for conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A local time names hour, minute, and second.
    LocalTimeHasHourMinuteSecond => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.2.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a complete local time of day carries hour, minute, and second components",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A reduced-accuracy local time may stop at the minute.
    ReducedAccuracyLocalTimeUsesHourMinuteRepresentation => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.2.3 a)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a reduced-accuracy local time may identify an hour and minute, without a second",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A reduced-accuracy local time may stop at the hour.
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.2.3 b)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a reduced-accuracy local time may identify an hour alone",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// An hour is 00 through 24.
    HourInRangeZeroToTwentyFour => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "5.3.1.4, 5.3.2",
        body: Iso,
        status: Normative,
        summary: "an hour component lies in the inclusive range 00 to 24, with 24 reserved for end-of-day",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.2.1", OpenTextCrossCheck),
    );

    /// A minute is 00 through 59.
    MinuteInRangeZeroToFiftyNine => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a minute component lies in the inclusive range 00 to 59",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A second is 00 through 60.
    SecondInRangeZeroToSixty => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a second component lies in the inclusive range 00 to 60, where 60 admits a leap second",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Hour 24 marks the end of a calendar day.
    TwentyFourHourReservedForEndOfDay => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "5.3.2",
        body: Iso,
        status: Normative,
        summary: "the hour value 24 denotes the end of a calendar day, not a time within the following day",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.2.1, 4.2.3", OpenTextCrossCheck),
    );

    /// Hour 24 forces zero minutes, seconds, and fraction.
    TwentyFourHourRequiresZeroMinuteSecondAndFraction => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "5.3.1.4, 5.3.2",
        body: Iso,
        status: Normative,
        summary: "when the hour is 24 the minute, second, and any fractional part are all zero",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.2.3", OpenTextCrossCheck),
    );

    /// A leap second sits only at a UTC boundary.
    LeapSecondOccursOnlyAtUtcBoundary => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a second value of 60 is admissible only at a UTC boundary at which UTC inserts a leap second",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A 24:00 end-of-day form is interval- or recurrence-only.
    EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "5.3.2",
        body: Iso,
        status: Normative,
        summary: "a 24:00:00 end-of-day time may appear only as an interval endpoint or within a recurrence, never standalone",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.2.3 note 2", OpenTextCrossCheck),
    );

    /// A 24:00 end-of-day form never identifies one time point.
    EndOfDayTwentyFourHourForbiddenForSingleTimePoint => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "5.3.2",
        body: Iso,
        status: Normative,
        summary: "a 24:00:00 end-of-day time is not permitted as the representation of a single time point",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.2.3 note 3", OpenTextCrossCheck),
    );

    /// A fraction uses the ISO 8601 decimal sign.
    FractionUsesDecimalSign => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.2.4",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a decimal fraction is introduced by an ISO 8601 decimal sign (comma or full stop)",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A fraction attaches to the lowest-order component present.
    FractionAppliesToLowestOrderComponent => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.6, 4.2.2.4",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a decimal fraction qualifies the lowest-order date or time component that is present",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A decimal representation fractions the lowest-order component.
    DecimalRepresentationUsesLowestOrderComponentFraction => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.3.6",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a decimal representation is one that adds a fractional part to the expression's lowest-order component",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A local time uses the time designator when context is ambiguous.
    LocalTimeRequiresTimeDesignatorWhenContextAmbiguous => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.2.2.5",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a local-time expression carries the leading time designator whenever surrounding context would not otherwise identify it as a time",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Standard time is UTC shifted by a location's fixed offset.
    StandardTimeDerivedFromUtcByLocalShift => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.14",
        body: Iso,
        status: Normative,
        summary: "a standard time scale is derived from UTC by a fixed time shift established for a given location",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A standard time of day is marked on a standard time scale.
    StandardTimeOfDayUsesStandardTimeScale => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.1.15",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a standard time of day is a time of day marked on a location's standard time scale",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A local time of day is marked on a local time scale.
    LocalTimeUsesLocallyApplicableTimeScale => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.17",
        body: Iso,
        status: Normative,
        summary: "a local time of day is a time of day on the locally applicable time scale",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A local time scale may be standard or another non-UTC scale.
    LocalTimeScaleMayBeStandardOrNonUtcBased => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.15",
        body: Iso,
        status: Normative,
        summary: "a local time scale is either a standard time scale or another time scale not based directly on UTC",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A time shift is a constant duration between two scales.
    TimeShiftIsConstantDurationBetweenTimeScales => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.25",
        body: Iso,
        status: Normative,
        summary: "a time shift is a constant duration equal to the difference between two time scales",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// UTC is the reference time scale.
    UtcIsReferenceTimeScale => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.12",
        body: Iso,
        status: Normative,
        summary: "Coordinated Universal Time is the reference time scale for UTC-related representations",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// UTC of day is a time of day on the UTC scale.
    UtcOfDayIdentifiesTimeWithinUtcCalendarDay => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.13",
        body: Iso,
        status: Normative,
        summary: "UTC of day is a time of day expressed on the UTC time scale",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );
}
