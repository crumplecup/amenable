//! Time interval, duration, and recurring-interval propositions (ISO
//! 8601-1:2019 §4.4-§4.5). Tier C throughout → `ParaphraseOnly`,
//! `status: Normative`, catalog URL; CalConnect CC 18011 / 18012 and the
//! ISO/WD 8601-1:2016(E) working draft appear as cross-checks.

use crate::NormativeQuotation;

temporal_standard! {
    /// A duration begins with `P`.
    DurationUsesPeriodDesignator => (
        document: "ISO 8601-1:2019",
        section: "4.4.2 b)",
        body: Iso,
        status: Normative,
        summary: "a duration representation is introduced by the period designator P",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "7.3", Informative),
    );

    /// A duration's time components follow `T`.
    DurationTimeComponentsFollowTimeDesignator => (
        document: "ISO 8601-1:2019",
        section: "4.4.3.2",
        body: Iso,
        status: Normative,
        summary: "the hour/minute/second components of a duration are introduced by the time designator T",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "7.3", Informative),
    );

    /// A week-form duration uses only the week unit.
    DurationWeekFormUsesSingleWeekUnit => (
        document: "ISO 8601-1:2019",
        section: "4.4.3.2",
        body: Iso,
        status: Normative,
        summary: "a week-form duration carries a single count of weeks with the W unit and nothing else",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "7.3", Informative),
    );

    /// A week-form duration is never mixed with other units.
    DurationWeekFormNotMixedWithCalendarOrClockUnits => (
        document: "ISO 8601-1:2019",
        section: "4.4.3.2",
        body: Iso,
        status: Normative,
        summary: "a week-form duration is not combined with year, month, day, hour, minute, or second units",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "7.3", Informative),
    );

    /// The alternative duration form needs partner agreement.
    DurationAlternativeFormRequiresPartnerAgreement => (
        document: "ISO 8601-1:2019",
        section: "4.4.3.3",
        body: Iso,
        status: Normative,
        summary: "the alternative (date-and-time-shaped) duration form is used only when the interchange partners have agreed to it",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.2.2", OpenTextCrossCheck),
    );

    /// The alternative duration form uses date/time slots.
    DurationAlternativeFormUsesDateAndTimeComponentSlots => (
        document: "ISO 8601-1:2019",
        section: "4.4.3.3",
        body: Iso,
        status: Normative,
        summary: "the alternative duration form is written in the calendar-date and time-of-day component slots, not with unit designators",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.2.2", OpenTextCrossCheck),
    );

    /// The alternative duration form carries every component.
    DurationAlternativeFormCarriesCompleteCalendarAndClockComponents => (
        document: "ISO 8601-1:2019",
        section: "4.4.3.3",
        body: Iso,
        status: Normative,
        summary: "the alternative duration form carries a complete set of calendar and clock components, none omitted",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.2.2", OpenTextCrossCheck),
    );

    /// A time interval separates its parts with `/`.
    TimeIntervalUsesSolidusSeparator => (
        document: "ISO 8601-1:2019",
        section: "4.4.2 a)",
        body: Iso,
        status: Normative,
        summary: "the two components of a time interval are joined by the solidus /",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "6.14", Informative),
    );

    /// A time interval has exactly two components.
    TimeIntervalHasTwoComponents => (
        document: "ISO 8601-1:2019",
        section: "4.4.1, 4.4.2 a)",
        body: Iso,
        status: Normative,
        summary: "a time interval representation has exactly two top-level components either side of the solidus",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "6.14", Informative),
    );

    /// A time interval declares its boundary form.
    TimeIntervalBoundaryOrDurationFormDeclared => (
        document: "ISO 8601-1:2019",
        section: "4.4.1",
        body: Iso,
        status: Normative,
        summary: "a time interval is explicitly one of start/end, start/duration, or duration/end",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "6.14", Informative),
    );

    /// An interval's endpoints run earlier to later.
    IntervalStartPrecedesEnd => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.6, 3.1.1.8",
        body: Iso,
        status: Normative,
        summary: "the first endpoint of an interval is no later than the second on the relevant timeline",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "8", Informative),
    );

    /// An interval duration is non-negative.
    IntervalDurationIsNonNegative => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.8",
        body: Iso,
        status: Normative,
        summary: "the duration between an interval's endpoints is zero or positive, never negative",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18011:2018", "8", Informative),
    );

    /// An interval calendar date may be swapped for an ordinal date.
    CompleteIntervalCalendarDateMayBeSubstitutedByOrdinalDate => (
        document: "ISO 8601-1:2019",
        section: "4.4.4.5",
        body: Iso,
        status: Normative,
        summary: "a complete-interval time-point's calendar date may instead be written as a complete ordinal date",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.5", OpenTextCrossCheck),
    );

    /// An interval calendar date may be swapped for a week date.
    CompleteIntervalCalendarDateMayBeSubstitutedByWeekDate => (
        document: "ISO 8601-1:2019",
        section: "4.4.4.5",
        body: Iso,
        status: Normative,
        summary: "a complete-interval time-point's calendar date may instead be written as a complete week date",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.5", OpenTextCrossCheck),
    );

    /// An interval local time may be swapped for UTC-of-day.
    CompleteIntervalLocalTimeMayBeSubstitutedByUtcOfDay => (
        document: "ISO 8601-1:2019",
        section: "4.4.4.5",
        body: Iso,
        status: Normative,
        summary: "a complete-interval time-point's local time may instead be written as a complete UTC-of-day",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.5", OpenTextCrossCheck),
    );

    /// An interval local time may be swapped for local time + UTC diff.
    CompleteIntervalLocalTimeMayBeSubstitutedByLocalTimeAndUtcDifference => (
        document: "ISO 8601-1:2019",
        section: "4.4.4.5",
        body: Iso,
        status: Normative,
        summary: "a complete-interval time-point's local time may instead be written as local time with an explicit UTC difference",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.5", OpenTextCrossCheck),
    );

    /// An interval duration may use the week form as a substitute.
    CompleteIntervalDurationMaySubstituteWeekForm => (
        document: "ISO 8601-1:2019",
        section: "4.4.4.5",
        body: Iso,
        status: Normative,
        summary: "a complete-interval duration in designator form may instead be written in week form",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.4.5", OpenTextCrossCheck),
    );

    /// An omitted higher-order end component inherits from the start.
    IntervalEndOmittedHigherOrderComponentsInheritFromStart => (
        document: "ISO 8601-1:2019",
        section: "4.4.5",
        body: Iso,
        status: Normative,
        summary: "a higher-order component omitted from an interval's end takes the value of the corresponding start component",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.5", OpenTextCrossCheck),
    );

    /// An omitted trailing zone/UTC designator inherits from the leading.
    IntervalTrailingComponentInheritsZoneOrUtcFromLeadingComponentWhenOmitted => (
        document: "ISO 8601-1:2019",
        section: "4.4.5",
        body: Iso,
        status: Normative,
        summary: "when the trailing interval component omits its zone or UTC designator, it inherits the leading component's",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.4.5", OpenTextCrossCheck),
    );

    /// A recurring interval begins with `R`.
    RecurringIntervalUsesRepeatDesignator => (
        document: "ISO 8601-1:2019",
        section: "4.5.2",
        body: Iso,
        status: Normative,
        summary: "a recurring interval is introduced by the repetition designator R",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18012:2018", "6.3, 6.4", Informative),
    );

    /// A bounded recurrence count is non-negative.
    RecurringIntervalCountIsNonNegativeWhenBounded => (
        document: "ISO 8601-1:2019",
        section: "4.5.1",
        body: Iso,
        status: Normative,
        summary: "when a recurring interval states a recurrence count, that count is zero or greater",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18012:2018", "6.3", Informative),
    );

    /// An omitted recurrence count means unbounded.
    RecurringIntervalOmittedCountDenotesUnboundedOccurrences => (
        document: "ISO 8601-1:2019",
        section: "4.5.1",
        body: Iso,
        status: Normative,
        summary: "a recurring interval with no recurrence count recurs without bound",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18012:2018", "6.4", Informative),
    );

    /// A recurring interval carries an interval component.
    RecurringIntervalCarriesIntervalComponent => (
        document: "ISO 8601-1:2019",
        section: "4.5.1, 4.5.2",
        body: Iso,
        status: Normative,
        summary: "after the repetition prefix, a recurring interval carries a time-interval component",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("CalConnect CC 18012:2018", "6.4", Informative),
    );

    /// A complete recurring interval embeds a complete interval.
    RecurringIntervalUsesCompleteTimeIntervalRepresentation => (
        document: "ISO 8601-1:2019",
        section: "4.5.3",
        body: Iso,
        status: Normative,
        summary: "a complete recurring interval embeds a complete time-interval representation",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.5.3", OpenTextCrossCheck),
    );

    /// An other-than-complete recurring interval embeds a §4.4.5 interval.
    RecurringIntervalUsesOtherThanCompleteTimeIntervalRepresentation => (
        document: "ISO 8601-1:2019",
        section: "4.5.4",
        body: Iso,
        status: Normative,
        summary: "an other-than-complete recurring interval embeds a §4.4.5 (component-inheriting) time-interval representation",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "4.5.4", OpenTextCrossCheck),
    );
}
