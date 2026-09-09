//! ISO 8601-2:2019 extended year forms (letter-prefixed, negative,
//! exponential, significant-digit) and seasonal / sub-year-grouping
//! expressions. See [`super::qualification`] for the tier-C conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A letter-prefixed year uses a leading `Y`.
    LetterPrefixedCalendarYearUsesLeadingYDesignator => (
        document: "ISO 8601-2:2019",
        section: "4.7.2",
        body: Iso,
        status: Normative,
        summary: "a letter-prefixed calendar year is introduced by a leading uppercase Y designator",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Letter-prefixed calendar year", Informative),
    );

    /// The `Y` form is only for years wider than four digits.
    LetterPrefixedCalendarYearMagnitudeExceedsFourDigits => (
        document: "ISO 8601-2:2019",
        section: "4.7.2",
        body: Iso,
        status: Normative,
        summary: "the letter-prefixed year form is used only when the absolute value of the year needs more than four digits",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Letter-prefixed calendar year", Informative),
    );

    /// A negative year uses an explicit leading minus.
    NegativeCalendarYearUsesLeadingMinusSign => (
        document: "ISO 8601-2:2019",
        section: "4.4.1",
        body: Iso,
        status: Normative,
        summary: "a calendar year before the common era is written with an explicit leading minus sign",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Negative calendar year", Informative),
    );

    /// A year before year one uses a trailing `B`.
    BeforeYearOneValueUsesTrailingBSuffix => (
        document: "ISO 8601-2:2019",
        section: "3.2.5, 4.4.1",
        body: Iso,
        status: Normative,
        summary: "a value before year one is written with a trailing uppercase B suffix",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("CalConnect CC 18011:2018", "suffix designator for years before year one", Informative),
    );

    /// An exponential year uses `E` power-of-ten notation.
    ExponentialYearUsesPowerOfTenNotation => (
        document: "ISO 8601-2:2019",
        section: "4.4.2, 4.7.3",
        body: Iso,
        status: Normative,
        summary: "an exponential year writes a signed significand followed by E and a power-of-ten exponent",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Exponential year", Informative),
    );

    /// An exponential year's exponent is a positive integer.
    ExponentialYearExponentIsPositiveInteger => (
        document: "ISO 8601-2:2019",
        section: "3.2.4, 4.4.2, 4.7.3",
        body: Iso,
        status: Normative,
        summary: "the exponent of an exponential year is a positive integer",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Exponential year", Informative),
    );

    /// A significant-digit year uses a trailing `S`.
    SignificantDigitYearUsesTrailingSSuffix => (
        document: "ISO 8601-2:2019",
        section: "3.2.5, 4.4.3",
        body: Iso,
        status: Normative,
        summary: "a significant-digit year is written with a trailing uppercase S suffix followed by the digit count",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Significant digits", Informative),
    );

    /// A significant-digit year's count is a positive integer.
    SignificantDigitYearCountIsPositiveInteger => (
        document: "ISO 8601-2:2019",
        section: "3.2.4, 4.4.3",
        body: Iso,
        status: Normative,
        summary: "the significant-digit count declared by a significant-digit year is a positive integer",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Significant digits", Informative),
    );

    /// A seasonal expression is a year-and-season form.
    SeasonalExpressionUsesYearAndSeasonForm => (
        document: "ISO 8601-2:2019",
        section: "4.8.1, 4.8.3",
        body: Iso,
        status: Normative,
        summary: "a seasonal temporal expression pairs a year with a season, shaped like a year-month form",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Seasons", Informative),
    );

    /// A seasonal expression puts the season code in the month slot.
    SeasonalExpressionUsesSeasonCodeInMonthSlot => (
        document: "ISO 8601-2:2019",
        section: "4.8.1, 4.8.3",
        body: Iso,
        status: Normative,
        summary: "in a seasonal expression the season code occupies the position a month number would hold in a year-month form",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Seasons", Informative),
    );

    /// A season code declares a named season.
    SeasonCodeDeclaresNamedSeason => (
        document: "ISO 8601-2:2019",
        section: "4.8.1",
        body: Iso,
        status: Normative,
        summary: "a season code names a season such as spring, summer, autumn, or winter",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 1 - Seasons; Level 2 - Sub-year groupings", Informative),
    );

    /// A season code declares its scope (location-independent or hemisphere).
    SeasonCodeDeclaresSeasonScope => (
        document: "ISO 8601-2:2019",
        section: "4.8.1",
        body: Iso,
        status: Normative,
        summary: "a season code declares whether its named season is location-independent or qualified to a hemisphere",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Sub-year groupings", Informative),
    );

    /// A sub-year grouping is a year-and-grouping form.
    SubYearGroupingExpressionUsesYearAndGroupingForm => (
        document: "ISO 8601-2:2019",
        section: "4.8.2, 4.8.3",
        body: Iso,
        status: Normative,
        summary: "a Level 2 sub-year grouping pairs a year with a grouping code, shaped like a year-month form",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Sub-year groupings", Informative),
    );

    /// A sub-year grouping puts the grouping code in the month slot.
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot => (
        document: "ISO 8601-2:2019",
        section: "4.8.2, 4.8.3",
        body: Iso,
        status: Normative,
        summary: "in a sub-year grouping the grouping code occupies the month position of a year-month-shaped form",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Sub-year groupings", Informative),
    );

    /// A grouping code may declare a quarter.
    SubYearGroupingCodeDeclaresQuarter => (
        document: "ISO 8601-2:2019",
        section: "4.8.1, 4.8.2",
        body: Iso,
        status: Normative,
        summary: "a sub-year grouping code may denote a calendar quarter (a three-month grouping)",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Sub-year groupings", Informative),
    );

    /// A grouping code may declare a quadrimester.
    SubYearGroupingCodeDeclaresQuadrimester => (
        document: "ISO 8601-2:2019",
        section: "4.8.1, 4.8.2",
        body: Iso,
        status: Normative,
        summary: "a sub-year grouping code may denote a quadrimester (a four-month grouping)",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Sub-year groupings", Informative),
    );

    /// A grouping code may declare a semester.
    SubYearGroupingCodeDeclaresSemestral => (
        document: "ISO 8601-2:2019",
        section: "4.8.1, 4.8.2",
        body: Iso,
        status: Normative,
        summary: "a sub-year grouping code may denote a semester (a six-month grouping)",
        quotation: NormativeQuotation::ParaphraseOnly,
        url: "https://www.iso.org/standard/70908.html",
        cross_check: ("LOC EDTF", "Level 2 - Sub-year groupings", Informative),
    );
}
