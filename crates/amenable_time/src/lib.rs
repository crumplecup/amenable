//! Temporal contract interface: standards-anchored propositions for date,
//! time, offset, and timestamp interchange, expressed in `amenable`'s
//! trait family.
//!
//! Every citation-only contract is a [`Standard`](amenable_core::Standard)
//! carrying a [`TemporalProvenance`] record that cites its normative
//! source (ISO 8601, RFC 3339, RFC 9557, CalConnect, IANA TZDB, SI/BIPM,
//! LoC EDTF). Composed / provable propositions are
//! [`Evidence`](amenable_core::Evidence); trait methods become
//! [`Exchange`](amenable_core::Exchange)s. See `docs/AMENABLE_TIME_PLAN.md`
//! for the full design and the migration phases.
//!
//! This crate carries **no checked-in standards corpus** — the relevant
//! normative clause lives embedded in each contract's provenance
//! metadata, governed by a three-tier redistributability rule (see
//! [`provenance_vocab`]). Nothing paywalled is reproduced.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[macro_use]
mod standard_macro;

mod contracts;
mod error;
mod provenance;
mod provenance_vocab;
mod traits;
mod types;

pub use contracts::{
    ApproximationQualificationDeclared, BasicFormatUsesMinimumComponentsForRequiredAccuracy,
    BeforeOrAfterQualificationIsLevelTwoOnly, BeforeOrOnDateUsesLeadingDoubleDotQualifier,
    BeforeYearOneValueUsesTrailingBSuffix, CalendarDateHasYearMonthDay,
    CalendarDateUsesGregorianCalendar,
    CalendarDayDurationMayBeModifiedByLeapSecondsOrLocalTimeShifts,
    CalendarDayIsIntervalOfSingleCalendarDateAdvance, CalendarDayWithinMonthBounds,
    CalendarMonthInRangeOneToTwelve, CalendarMonthIsNamedIntervalWithinCalendarYear,
    CalendarWeekIsSevenDayIntervalBeginningOnMonday, CalendarWeekNumberUsesFirstThursdayRule,
    CalendarWeekStartsOnMonday, CalendarYearInRangeZeroToNineThousandNineHundredNinetyNine,
    CalendarYearIsIntervalOfSuccessiveCalendarMonths,
    CalendarYearThrough1582RequiresMutualAgreement, CentennialYearDivisibleByOneHundred,
    CenturyOrdinalInRangeZeroToNinetyNine, ColonSeparatesTimeComponents,
    CombinedDateTimeDateComponentMustNotUseReducedAccuracy,
    CombinedDateTimePermitsCalendarDateComponent, CombinedDateTimePermitsOrdinalDateComponent,
    CombinedDateTimePermitsWeekDateComponent,
    CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
    CombinedDateTimeUsesTimeDesignator, CommonYearHasThreeHundredSixtyFiveCalendarDays,
    CompleteRepresentationCarriesAllRequiredComponents,
    ComponentQualificationAppliesOnlyToMarkedComponent,
    ComponentQualificationUsesImmediateLeftPlacement, DateIdentifiesPositionWithinCalendar,
    DateTimeFormatRepresentationDescribesRepresentationFamily,
    DateTimeFormatRepresentationsForbiddenInTelexRepertoire,
    DateTimeRepresentationIdentifiesPointIntervalOrRecurrence,
    DayDurationMaySpanSameTimeOfDayOnAdjacentCalendarDays, DayEqualsTwentyFourHours,
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    DecimalRepresentationUsesLowestOrderComponentFraction,
    DurationEqualsDifferenceBetweenIntervalEndpoints,
    EndOfDayTwentyFourHourAllowedOnlyWithinIntervalOrRecurrence,
    EndOfDayTwentyFourHourForbiddenForSingleTimePoint,
    EnhancedIntervalLevelOnePermitsTerminalBoundaryQualification,
    EnhancedIntervalLevelTwoPermitsBeforeOrAfterBoundaryQualification,
    EnhancedIntervalLevelTwoPermitsInternalBoundaryQualification,
    EnhancedIntervalLevelTwoPermitsInternalBoundaryUnspecifiedDigits,
    ExpandedRepresentationRequiresAdditionalAgreement, ExponentialYearExponentIsPositiveInteger,
    ExponentialYearUsesPowerOfTenNotation, FixedWidthComponentsRequireLeadingZeros,
    FormatRepresentationUsesPlaceholderCharactersForDigitsAndSigns,
    FractionAppliesToLowestOrderComponent, FractionUsesDecimalSign,
    FractionalSecondDigitsAreContiguous, FractionalSecondPrecisionDeclared,
    GregorianLeapYearUsesDivisibleByFourAndFourHundredException,
    GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    GroupQualificationUsesImmediateRightPlacement, HourEqualsSixtyMinutes,
    HourInRangeZeroToTwentyFour, HyphenSeparatesDateComponents, InstantIsPointOnTimeAxis,
    LeapDayOccursOnlyInLeapYear, LeapSecondOccursOnlyAtUtcBoundary,
    LeapYearHasThreeHundredSixtySixCalendarDays,
    LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
    LetterPrefixedCalendarYearUsesLeadingYDesignator,
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent,
    LiteralFormatCharactersCopyIntoRepresentations, LocalTimeHasHourMinuteSecond,
    LocalTimeRequiresTimeDesignatorWhenContextAmbiguous, LocalTimeScaleMayBeStandardOrNonUtcBased,
    LocalTimeUsesLocallyApplicableTimeScale, MinuteEqualsSixtySeconds,
    MinuteInRangeZeroToFiftyNine, MonthDurationInRangeTwentyEightToThirtyOneCalendarDays,
    MonthDurationMayRequireAgreedEndingCalendarDay,
    MonthMayBeConsideredThirtyCalendarDaysInCertainApplications,
    NegativeCalendarYearUsesLeadingMinusSign, NominalDayDurationMayDifferFromExactElapsedTime,
    NominalDurationDependsOnCalendarContext, NominalMonthDurationDependsOnCalendarContext,
    NominalWeekDurationIsDistinctFromExactElapsedTime, NominalYearDurationDependsOnCalendarContext,
    OnOrAfterDateUsesTrailingDoubleDotQualifier, OpenIntervalBoundaryDeclared,
    OrdinalDateHasYearAndDayOfYear, OrdinalDayInRangeOneToThreeHundredSixtySix,
    OrdinalDayOfYearUsesThreeDigits, PrecisionReductionDeclared,
    ProlepticGregorianDatesBefore1583RequireMutualAgreement, QualificationScopeDeclared,
    ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation,
    ReducedAccuracyRepresentationOmitsLowerOrderComponents,
    ReducedAccuracyWeekDateOmitsWeekdayComponent, ReducedCalendarDateHasYearComponent,
    ReducedCalendarDateOmitsLowerOrderDigitsFromExtremeRight,
    ReducedCalendarDateUsesYearMonthRepresentation, ReducedCalendarDateUsesYearOnlyRepresentation,
    Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator,
    Rfc3339ClientsShouldTransformDatesForLocalityDisplay, Rfc3339FractionUsesDotSeparator,
    Rfc3339FractionalSecondsAreOnlyRarelyUsedOption, Rfc3339GeneratorsShouldUseUppercaseTAndZ,
    Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
    Rfc3339LexicalOrderingRequiresUniformFractionalSecondDigits,
    Rfc3339LexicalOrderingRequiresUniformUtcRelationshipEncoding, Rfc3339LocalOffsetNotUnknown,
    Rfc3339LocalityDisplayMayTranslateUtcToLocalTime, Rfc3339OffsetIsUtcOrNumeric,
    Rfc3339PositiveZeroOffsetDeclaresPreferredUtcReferencePoint,
    Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory, Rfc3339RequiresUtcRelationship,
    Rfc3339TimestampExcludesRedundantWeekdayInformation,
    Rfc3339UnknownLocalOffsetUsesZuluDesignator, Rfc3339UnqualifiedLocalTimeForbidden,
    Rfc3339UsesExtendedCalendarDate, Rfc3339UsesFourDigitYear, Rfc3339UsesFullTime,
    RoundingModeDeclared, SeasonCodeDeclaresNamedSeason, SeasonCodeDeclaresSeasonScope,
    SeasonalExpressionUsesSeasonCodeInMonthSlot, SeasonalExpressionUsesYearAndSeasonForm,
    SecondInRangeZeroToSixty, SecondIsBaseUnitForExpressingDuration, SecondIsSiBaseUnitOfTime,
    SignificantDigitYearCountIsPositiveInteger, SignificantDigitYearUsesTrailingSSuffix,
    SpaceForbiddenUnlessExplicitlyPermitted, StandardTimeDerivedFromUtcByLocalShift,
    StandardTimeOfDayUsesStandardTimeScale, SubYearGroupingCodeDeclaresQuadrimester,
    SubYearGroupingCodeDeclaresQuarter, SubYearGroupingCodeDeclaresSemestral,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubsecondDigitsPreserved,
    TemporalChoiceSetUsesSquareBrackets, TemporalInclusiveSetUsesCurlyBraces,
    TemporalSetCarriesMultipleMembers, TemporalSetDeclaresAlternativeSemantics,
    TemporalSetDeclaresInclusiveMemberSemantics, TemporalSetForbidsInternalWhitespace,
    TemporalSetMemberSeparatorDeclared, TemporalSetOpenRangeUsesBoundaryDoubleDot,
    TemporalSetRangeNeighborhoodSharesPrecision, TemporalSetRangeUsesInclusiveDoubleDotSemantics,
    TimeAxisOrdersTimePointsByTemporalPosition, TimeIsMarkOnSpecifiedTimeScale,
    TimeOfDayOccursWithinCalendarDay, TimeScaleAssociatesTimePointsWithOrderedMeasure,
    TimeShiftIsConstantDurationBetweenTimeScales,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UncertaintyAndApproximationMayBeCombined, UncertaintyQualificationDeclared,
    UnderlineFallbackPrecedesQualifiedFormatCharacter,
    UnderlinedFormatPlaceholderRepresentsZeroOrMoreDigits, UnknownIntervalBoundaryDeclared,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
    UtcDesignatorIsUppercaseZ, UtcDifferenceAppendedImmediatelyWithoutSpace,
    UtcDifferenceExpressionIsNotSelfStanding,
    UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
    UtcDifferenceSignEncodesDirectionRelativeToUtc, UtcIsReferenceTimeScale,
    UtcOfDayIdentifiesTimeWithinUtcCalendarDay, UtcOfDayUsesTrailingZuluDesignatorImmediately,
    UtcOffsetCarriesSignHourAndOptionalMinute, UtcOffsetHourInRangeZeroToTwentyThree,
    UtcOffsetMinuteInRangeZeroToFiftyNine, WeekDateHasWeekYearWeekAndWeekday,
    WeekDateUsesWeekDesignator, WeekDurationMaySpanSameTimeOfDayInNextCalendarWeek,
    WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
    YearDurationInRangeThreeHundredSixtyFiveToThreeHundredSixtySixCalendarDays,
    YearDurationMayRequireAgreedEndingCalendarDate,
};
pub use error::{TemporalError, TemporalErrorKind, TemporalResult};
pub use provenance::TemporalProvenance;
pub use provenance_vocab::{
    CrossCheck, NormativeDocument, NormativeQuotation, NormativeSection, NormativeStatus,
    StandardsBody,
};
pub use traits::TemporalReporter;
pub use types::{SerializationProfile, TemporalComponent};
