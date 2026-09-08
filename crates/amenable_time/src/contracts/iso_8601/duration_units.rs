//! Unit-of-time definitions (ISO 8601-1:2019 §2.2) and nominal-duration
//! semantics (§3.1.1.8). See [`super::definitions`] for conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// The second is the SI base unit of time.
    SecondIsSiBaseUnitOfTime => (
        document: "ISO 8601-1:2019",
        section: "2.2.1",
        body: Iso,
        status: Normative,
        summary: "the second is the base unit of time in the International System of Units",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// The second is the base unit for durations.
    SecondIsBaseUnitForExpressingDuration => (
        document: "ISO 8601-1:2019",
        section: "2.2.1",
        body: Iso,
        status: Normative,
        summary: "the second is the base unit in which durations are expressed",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A minute is sixty seconds.
    MinuteEqualsSixtySeconds => (
        document: "ISO 8601-1:2019",
        section: "2.2.3",
        body: Iso,
        status: Normative,
        summary: "a minute, as a unit, is exactly sixty seconds",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// An hour is sixty minutes.
    HourEqualsSixtyMinutes => (
        document: "ISO 8601-1:2019",
        section: "2.2.4",
        body: Iso,
        status: Normative,
        summary: "an hour, as a unit, is exactly sixty minutes",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A day, as a unit, is twenty-four hours.
    DayEqualsTwentyFourHours => (
        document: "ISO 8601-1:2019",
        section: "2.2.5",
        body: Iso,
        status: Normative,
        summary: "a day, taken as a unit of time, is exactly twenty-four hours",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A duration is the span between an interval's endpoints.
    DurationEqualsDifferenceBetweenIntervalEndpoints => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.8",
        body: Iso,
        status: Normative,
        summary: "a duration is the quantity of time between the final and initial instants of a time interval",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A calendar day is one calendar-date advance.
    CalendarDayIsIntervalOfSingleCalendarDateAdvance => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.2.6",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar day is the time interval spanned by advancing the calendar date by one",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A calendar day's real duration flexes with leap seconds and shifts.
    CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts => (
        document: "ISO 8601-1:2019",
        section: "2.2.6",
        body: Iso,
        status: Normative,
        summary: "the elapsed time of a calendar day may differ from twenty-four hours because of leap seconds or local-time-scale changes",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A day as a nominal duration need not equal exact elapsed time.
    NominalDayDurationMayDifferFromExactElapsedTime => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.8 note 3",
        body: Iso,
        status: Normative,
        summary: "a day used as a nominal duration is a calendar-context quantity, not necessarily a fixed count of elapsed seconds",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "2.2.7", OpenTextCrossCheck),
    );

    /// A day duration runs to the same clock time next day.
    DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays => (
        document: "ISO 8601-1:2019",
        section: "2.2.7",
        body: Iso,
        status: Normative,
        summary: "a day, as a duration, may run from a time of day to the same time of day on the next calendar day",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A calendar week is seven days starting Monday.
    CalendarWeekIsSevenDayIntervalBeginningOnMonday => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.2.8",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar week is a seven-day calendar interval that begins on Monday",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A week as a nominal duration differs from exact elapsed time.
    NominalWeekDurationIsDistinctFromExactElapsedTime => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.8 note 3",
        body: Iso,
        status: Normative,
        summary: "a week used as a nominal duration is a calendar-context quantity distinct from a fixed elapsed-time measure",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "2.2.9", OpenTextCrossCheck),
    );

    /// A week duration runs to the same weekday and time next week.
    WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek => (
        document: "ISO 8601-1:2019",
        section: "2.2.9",
        body: Iso,
        status: Normative,
        summary: "a week, as a duration, may run from a time of day to the same weekday and time of day in the next calendar week",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A calendar month is a named interval within a calendar year.
    CalendarMonthIsNamedIntervalWithinCalendarYear => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.2.11",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar month is one of the named month-length intervals that partition a calendar year",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A month as a nominal duration depends on calendar context.
    NominalMonthDurationDependsOnCalendarContext => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.8 note 3",
        body: Iso,
        status: Normative,
        summary: "a month used as a nominal duration is resolved against calendar context, not a fixed number of days",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "2.2.12", OpenTextCrossCheck),
    );

    /// A month duration is 28 to 31 calendar days.
    MonthDurationInRangeTwentyEightToThirtyOneCalendarDays => (
        document: "ISO 8601-1:2019",
        section: "2.2.12",
        body: Iso,
        status: Normative,
        summary: "a month's duration is 28, 29, 30, or 31 calendar days according to the month and year",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A month duration may need an agreed ending day.
    MonthDurationMayRequireAgreedEndingCalendarDay => (
        document: "ISO 8601-1:2019",
        section: "2.2.12",
        body: Iso,
        status: Normative,
        summary: "adding a month may require the parties to agree on the ending day when the starting day does not exist in the target month",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// Some applications treat a month as thirty days.
    MonthMayBeConsideredThirtyCalendarDaysInCertainApplications => (
        document: "ISO 8601-1:2019",
        section: "2.2.12",
        body: Iso,
        status: Normative,
        summary: "certain applications may, by agreement, treat a month as a fixed duration of thirty calendar days",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A calendar year is its successive calendar months.
    CalendarYearIsIntervalOfSuccessiveCalendarMonths => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.2.13",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar year is the interval formed by its twelve successive calendar months",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A year as a nominal duration depends on calendar context.
    NominalYearDurationDependsOnCalendarContext => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.8 note 3",
        body: Iso,
        status: Normative,
        summary: "a year used as a nominal duration is resolved against calendar context, not a fixed number of days",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "2.2.14", OpenTextCrossCheck),
    );

    /// A year duration is 365 or 366 calendar days.
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays => (
        document: "ISO 8601-1:2019",
        section: "2.2.14",
        body: Iso,
        status: Normative,
        summary: "a year's duration is 365 or 366 calendar days according to whether the year is a leap year",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A year duration may need an agreed ending date.
    YearDurationMayRequireAgreedEndingCalendarDate => (
        document: "ISO 8601-1:2019",
        section: "2.2.14",
        body: Iso,
        status: Normative,
        summary: "adding a year may require the parties to agree on the ending date when the starting date does not exist in the target year (e.g. 29 February)",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A nominal duration depends on calendar context, not fixed seconds.
    NominalDurationDependsOnCalendarContext => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.8 note 3",
        body: Iso,
        status: Normative,
        summary: "a nominal duration is interpreted against calendar context rather than as a fixed count of elapsed seconds",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );
}
