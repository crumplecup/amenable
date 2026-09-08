//! Temporal contract types, grouped by normative source. The module
//! layout mirrors `elicit_temporal/src/contracts/` one-to-one so bulk
//! registration stays orderly (`docs/AMENABLE_TIME_PLAN.md`,
//! "Registration strategy").
//!
//! Citation-only structural contracts are
//! [`Standard`](amenable_core::Standard)s (this module). The composed /
//! provable `*Valid` aggregates become
//! [`Evidence`](amenable_core::Evidence) in `contracts::proof_composition`
//! (plan Phase 3).

mod iso_8601;
mod precision;

pub use iso_8601::{
    CalendarDateHasYearMonthDay, CalendarDateUsesGregorianCalendar,
    CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts,
    CalendarDayIsIntervalOfSingleCalendarDateAdvance, CalendarDayWithinMonthBounds,
    CalendarMonthInRangeOneToTwelve, CalendarMonthIsNamedIntervalWithinCalendarYear,
    CalendarWeekIsSevenDayIntervalBeginningOnMonday, CalendarWeekNumberUsesFirstThursdayRule,
    CalendarWeekStartsOnMonday, CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CalendarYearIsIntervalOfSuccessiveCalendarMonths,
    CalendarYearThrough1582RequiresMutualAgreement, CentennialYearDivisibleByOneHundred,
    CenturyOrdinalInRangeZeroToNinetyNine, CommonYearHasThreeHundredSixtyFiveCalendarDays,
    DateIdentifiesPositionWithinCalendar, DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays,
    DayEqualsTwentyFourHours, DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    DecimalRepresentationUsesLowestOrderComponentFraction,
    DurationEqualsDifferenceBetweenIntervalEndpoints,
    EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence,
    EndOfDayTwentyFourHourForbiddenForSingleTimePoint, FractionAppliesToLowestOrderComponent,
    FractionUsesDecimalSign, GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    HourEqualsSixtyMinutes, HourInRangeZeroToTwentyFour, InstantIsPointOnTimeAxis,
    LeapDayOccursOnlyInLeapYear, LeapSecondOccursOnlyAtUtcBoundary,
    LeapYearHasThreeHundredSixtySixCalendarDays, LocalTimeHasHourMinuteSecond,
    LocalTimeRequiresTimeDesignatorWhenContextAmbiguous, LocalTimeScaleMayBeStandardOrNonUtcBased,
    LocalTimeUsesLocallyApplicableTimeScale, MinuteEqualsSixtySeconds,
    MinuteInRangeZeroToFiftyNine, MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    MonthDurationMayRequireAgreedEndingCalendarDay,
    MonthMayBeConsideredThirtyCalendarDaysInCertainApplications,
    NominalDayDurationMayDifferFromExactElapsedTime, NominalDurationDependsOnCalendarContext,
    NominalMonthDurationDependsOnCalendarContext,
    NominalWeekDurationIsDistinctFromExactElapsedTime, NominalYearDurationDependsOnCalendarContext,
    OrdinalDateHasYearAndDayOfYear, OrdinalDayInRangeOneToThreeHundredSixtySix,
    OrdinalDayOfYearUsesThreeDigits, ProlepticGregorianDatesBefore1583RequireMutualAgreement,
    ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation,
    ReducedAccuracyWeekDateOmitsWeekdayComponent, ReducedCalendarDateHasYearComponent,
    ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight,
    ReducedCalendarDateUsesYearMonthRepresentation, ReducedCalendarDateUsesYearOnlyRepresentation,
    SecondInRangeZeroToSixty, SecondIsBaseUnitForExpressingDuration, SecondIsSiBaseUnitOfTime,
    StandardTimeDerivedFromUtcByLocalShift, StandardTimeOfDayUsesStandardTimeScale,
    TimeAxisOrdersTimePointsByTemporalPosition, TimeIsMarkOnSpecifiedTimeScale,
    TimeOfDayOccursWithinCalendarDay, TimeScaleAssociatesTimePointsWithOrderedMeasure,
    TimeShiftIsConstantDurationBetweenTimeScales,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UtcIsReferenceTimeScale, UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
    WeekDateHasWeekYearWeekAndWeekday, WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek,
    WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    YearDurationMayRequireAgreedEndingCalendarDate,
};
pub use precision::{
    FractionalSecondDigitsAreContiguous, FractionalSecondPrecisionDeclared,
    PrecisionReductionDeclared, RoundingModeDeclared, SubsecondDigitsPreserved,
};
