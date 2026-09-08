//! ISO 8601 temporal propositions — the largest contract module (109 in
//! `elicit_temporal`). Split by group to stay under the 500-LOC cap.
//!
//! Sources: ISO 8601-1:2019 (+ Amd 1:2022), ISO 8601-2:2019, and the
//! publicly-circulated working draft ISO/WD 8601-1:2016(E). All tier C
//! (`docs/AMENABLE_TIME_PLAN.md`): paywalled or draft, so every contract
//! carries `NormativeQuotation::ParaphraseOnly` and no ISO prose is
//! reproduced. Published-standard citations are `status: Normative`;
//! working-draft-only citations are `status: OpenTextCrossCheck`.
//!
//! Groups land one at a time — see `docs/AMENABLE_TIME_COVERAGE.md` for
//! the live count.

mod calendar;
mod combined_and_format;
mod definitions;
mod duration_units;
mod time_of_day;
mod utc_offset;

pub use combined_and_format::{
    BasicFormatUsesMinimumComponentsForRequiredAccuracy, ColonSeparatesTimeComponents,
    CombinedDateTimeDateComponentMustNotUseReducedAccuracy,
    CombinedDateTimePermitsCalendarDateComponent, CombinedDateTimePermitsOrdinalDateComponent,
    CombinedDateTimePermitsWeekDateComponent,
    CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
    CombinedDateTimeUsesTimeDesignator, CompleteRepresentationCarriesAllRequiredComponents,
    DateTimeFormatRepresentationDescribesRepresentationFamily,
    DateTimeFormatRepresentationsForbiddenInTelexRepertoire,
    DateTimeRepresentationIdentifiesPointIntervalOrRecurrence,
    ExpandedRepresentationRequiresAdditionalAgreement, FixedWidthComponentsRequireLeadingZeros,
    FormatRepresentationUsesPlaceholderCharactersForDigitsAndSigns, HyphenSeparatesDateComponents,
    LiteralFormatCharactersCopyIntoRepresentations,
    ReducedAccuracyRepresentationOmitsLowerOrderComponents,
    SpaceForbiddenUnlessExplicitlyPermitted, UnderlineFallbackPrecedesQualifiedFormatCharacter,
    UnderlinedFormatPlaceholderRepresentsZeroOrMoreDigits, UtcDesignatorIsUppercaseZ,
    WeekDateUsesWeekDesignator,
};
pub use utc_offset::{
    UtcDifferenceAppendedImmediatelyWithoutSpace, UtcDifferenceExpressionIsNotSelfStanding,
    UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
    UtcDifferenceSignEncodesDirectionRelativeToUtc, UtcOfDayUsesTrailingZuluDesignatorImmediately,
    UtcOffsetCarriesSignHourAndOptionalMinute, UtcOffsetHourInRangeZeroToTwentyThree,
    UtcOffsetMinuteInRangeZeroToFiftyNine,
};

pub use duration_units::{
    CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts,
    CalendarDayIsIntervalOfSingleCalendarDateAdvance,
    CalendarMonthIsNamedIntervalWithinCalendarYear,
    CalendarWeekIsSevenDayIntervalBeginningOnMonday,
    CalendarYearIsIntervalOfSuccessiveCalendarMonths,
    DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays, DayEqualsTwentyFourHours,
    DurationEqualsDifferenceBetweenIntervalEndpoints, HourEqualsSixtyMinutes,
    MinuteEqualsSixtySeconds, MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    MonthDurationMayRequireAgreedEndingCalendarDay,
    MonthMayBeConsideredThirtyCalendarDaysInCertainApplications,
    NominalDayDurationMayDifferFromExactElapsedTime, NominalDurationDependsOnCalendarContext,
    NominalMonthDurationDependsOnCalendarContext,
    NominalWeekDurationIsDistinctFromExactElapsedTime, NominalYearDurationDependsOnCalendarContext,
    SecondIsBaseUnitForExpressingDuration, SecondIsSiBaseUnitOfTime,
    WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    YearDurationMayRequireAgreedEndingCalendarDate,
};

pub use calendar::{
    CalendarDateHasYearMonthDay, CalendarDateUsesGregorianCalendar, CalendarDayWithinMonthBounds,
    CalendarMonthInRangeOneToTwelve, CalendarWeekNumberUsesFirstThursdayRule,
    CalendarWeekStartsOnMonday, CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CalendarYearThrough1582RequiresMutualAgreement, CentennialYearDivisibleByOneHundred,
    CenturyOrdinalInRangeZeroToNinetyNine, CommonYearHasThreeHundredSixtyFiveCalendarDays,
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException, LeapDayOccursOnlyInLeapYear,
    LeapYearHasThreeHundredSixtySixCalendarDays, OrdinalDateHasYearAndDayOfYear,
    OrdinalDayInRangeOneToThreeHundredSixtySix, OrdinalDayOfYearUsesThreeDigits,
    ProlepticGregorianDatesBefore1583RequireMutualAgreement,
    ReducedAccuracyWeekDateOmitsWeekdayComponent, ReducedCalendarDateHasYearComponent,
    ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight,
    ReducedCalendarDateUsesYearMonthRepresentation, ReducedCalendarDateUsesYearOnlyRepresentation,
    WeekDateHasWeekYearWeekAndWeekday, WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
};
pub use definitions::{
    DateIdentifiesPositionWithinCalendar, InstantIsPointOnTimeAxis,
    TimeAxisOrdersTimePointsByTemporalPosition, TimeIsMarkOnSpecifiedTimeScale,
    TimeOfDayOccursWithinCalendarDay, TimeScaleAssociatesTimePointsWithOrderedMeasure,
};
pub use time_of_day::{
    DecimalRepresentationUsesLowestOrderComponentFraction,
    EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence,
    EndOfDayTwentyFourHourForbiddenForSingleTimePoint, FractionAppliesToLowestOrderComponent,
    FractionUsesDecimalSign, HourInRangeZeroToTwentyFour, LeapSecondOccursOnlyAtUtcBoundary,
    LocalTimeHasHourMinuteSecond, LocalTimeRequiresTimeDesignatorWhenContextAmbiguous,
    LocalTimeScaleMayBeStandardOrNonUtcBased, LocalTimeUsesLocallyApplicableTimeScale,
    MinuteInRangeZeroToFiftyNine, ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation, SecondInRangeZeroToSixty,
    StandardTimeDerivedFromUtcByLocalShift, StandardTimeOfDayUsesStandardTimeScale,
    TimeShiftIsConstantDurationBetweenTimeScales,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UtcIsReferenceTimeScale, UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
};
