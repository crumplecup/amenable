//! Calendar date, ordinal date, and week date — component structure and
//! value ranges. See [`super::definitions`] for the tier-C /
//! `ParaphraseOnly` conventions this module follows.

use crate::NormativeQuotation;

temporal_standard! {
    /// A calendar date is expressed in the proleptic Gregorian calendar.
    CalendarDateUsesGregorianCalendar => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.2.1, 4.1.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar date is a proleptic Gregorian calendar representation",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A complete calendar date names a year, a month, and a day.
    CalendarDateHasYearMonthDay => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.1.9, 4.1.2.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar date carries year, month, and day-of-month components",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A reduced-precision calendar date still carries a year.
    ReducedCalendarDateHasYearComponent => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.2.3",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a reduced-precision calendar date always retains its calendar-year component",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A reduced-precision calendar date may be a bare year.
    ReducedCalendarDateUsesYearOnlyRepresentation => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.2.3 b)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a reduced-precision calendar date may be given as the year alone",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A reduced-precision calendar date may be a year and month.
    ReducedCalendarDateUsesYearMonthRepresentation => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.2.3 a)",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a reduced-precision calendar date may be given as year and month, without a day",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Reduced precision drops components from the right.
    ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.2.3",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "reducing a calendar date's precision removes lower-order components from the right-hand end, never the middle",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A calendar-date month is 01 through 12.
    CalendarMonthInRangeOneToTwelve => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.2.1, 4.1.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar-date month component lies in the inclusive range 01 to 12",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A calendar-date day is within its month's bounds.
    CalendarDayWithinMonthBounds => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.2.1, 4.1.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar-date day component is within the valid day count for that month and year",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// February 29 exists only in a leap year.
    LeapDayOccursOnlyInLeapYear => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.21 note 1",
        body: Iso,
        status: Normative,
        summary: "the 29th of February is a valid calendar date only when the year is a leap year",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "3.2.1", OpenTextCrossCheck),
    );

    /// A decade ordinal is 000 through 999.
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "4.3.11",
        body: Iso,
        status: Normative,
        summary: "a Gregorian decade ordinal lies in the inclusive range 000 to 999",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A century ordinal is 00 through 99.
    CenturyOrdinalInRangeZeroToNinetyNine => (
        document: "ISO 8601-1:2019/Amd 1:2022",
        section: "4.3.12",
        body: Iso,
        status: Normative,
        summary: "a Gregorian century ordinal lies in the inclusive range 00 to 99",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// An ordinal date names a year and a day of that year.
    OrdinalDateHasYearAndDayOfYear => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.1.10, 4.1.3.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "an ordinal date carries a year component and a day-of-year component",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// An ordinal day is 001 through 365 or 366.
    OrdinalDayInRangeOneToThreeHundredSixtySix => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.2.1, 4.1.3.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "an ordinal day-of-year lies in the range 001 to 365, or to 366 in a leap year",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// An ordinal day-of-year is three digits.
    OrdinalDayOfYearUsesThreeDigits => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.3.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "the day-of-year component of an ordinal date is written as three decimal digits",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A week date names a week-year, week number, and weekday.
    WeekDateHasWeekYearWeekAndWeekday => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.1.11, 4.1.4.2",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a week date carries week-year, week-number, and weekday components",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A week number is 01 through 53.
    WeekNumberInRangeOneToFiftyThree => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.4.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar-week number lies in the inclusive range 01 to 53",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A calendar week starts on Monday.
    CalendarWeekStartsOnMonday => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "2.2.8, 4.1.4.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar week begins on Monday",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Week numbering uses the first-Thursday rule.
    CalendarWeekNumberUsesFirstThursdayRule => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.23",
        body: Iso,
        status: Normative,
        summary: "week 01 is the week containing the year's first Thursday (equivalently, the week containing January 4th)",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "2.2.10", OpenTextCrossCheck),
    );

    /// A weekday is 1 through 7.
    WeekdayInRangeOneToSeven => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.4.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a weekday component lies in the inclusive range 1 (Monday) to 7 (Sunday)",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A reduced-accuracy week date drops the weekday.
    ReducedAccuracyWeekDateOmitsWeekdayComponent => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.4.3",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a reduced-accuracy week date omits the weekday and identifies a whole week",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A non-expanded calendar year is 0000 through 9999.
    CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "a calendar year written without an expanded-representation agreement lies in the range 0000 to 9999",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Years 0000-1582 need mutual agreement.
    CalendarYearThrough1582RequiresMutualAgreement => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "4.1.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "interchanging a calendar year in the range 0000 to 1582 requires prior agreement between the parties",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// Proleptic Gregorian dates before 1583 need mutual agreement.
    ProlepticGregorianDatesBefore1583RequireMutualAgreement => (
        document: "ISO/WD 8601-1:2016(E)",
        section: "3.2.1",
        body: Iso,
        status: OpenTextCrossCheck,
        summary: "using a proleptic Gregorian date earlier than the 1582 calendar introduction requires prior agreement",
        quotation: NormativeQuotation::ParaphraseOnly,
    );

    /// A common year has 365 days.
    CommonYearHasThreeHundredSixtyFiveCalendarDays => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.20",
        body: Iso,
        status: Normative,
        summary: "a common year contains 365 calendar days",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A leap year has 366 days.
    LeapYearHasThreeHundredSixtySixCalendarDays => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.21",
        body: Iso,
        status: Normative,
        summary: "a leap year contains 366 calendar days",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// A centennial year is divisible by 100.
    CentennialYearDivisibleByOneHundred => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.22",
        body: Iso,
        status: Normative,
        summary: "a centennial year is one whose year number is an exact multiple of 100",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
    );

    /// The Gregorian leap-year rule.
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException => (
        document: "ISO 8601-1:2019",
        section: "3.1.1.21 note 1",
        body: Iso,
        status: Normative,
        summary: "a year is a leap year if divisible by 4, except a centennial year is a leap year only if also divisible by 400",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70907.html",
        cross_check: ("ISO/WD 8601-1:2016(E)", "3.2.1", OpenTextCrossCheck),
    );
}
