//! Aggregate temporal proof propositions (L–S), ported from
//! `elicit_temporal::contracts::proof_composition`. Each aggregate is
//! folded with its `*Evidence` bundle: the aggregate's fields ARE its
//! sub-claims, and `#[derive(Witness)]` makes the composite proof the
//! structural product of its members' proofs. See [`super`] for the
//! design; leaf `Witness<V>` impls land in the backend crates.

use crate::{
    ApproximationQualificationDeclared, CalendarDateUsesGregorianCalendar,
    ConversionDropsSubsecondPrecision, ConversionRequiresExplicitAuthorityWhenLossy,
    FractionAppliesToLowestOrderComponent, FractionUsesDecimalSign,
    FractionalSecondDigitsAreContiguous, FractionalSecondPrecisionDeclared,
    HourInRangeZeroToTwentyFour, IxdtfTimestampValid, LocalDateTimeValid,
    MutualAgreementAuthorityScopeEvidence, NamedTimeZoneIdentifierExcludesDotSegments,
    NamedTimeZoneIdentifierIsCaseSensitive, NamedTimeZoneIsNotNumericOffsetAlias,
    NamedTimeZoneMeaningUsesCurrentTzdbRules, NamedTimeZoneRetainsCivilRuleIdentity,
    NamedTimeZoneUsesIanaIdentifier, NumericOffsetDoesNotIdentifyNamedZone,
    OffsetTimeZoneAnnotationPresent, OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset,
    OffsetTimeZoneRepeatsTimestampOffset, OffsetTimeZoneUseIsStronglyDiscouraged,
    OrdinalDateHasYearAndDayOfYear, OrdinalDayInRangeOneToThreeHundredSixtySix,
    PrecisionReductionDeclared, QualificationPlacementEvidence, QualificationScopeDeclared,
    RecurringIntervalCarriesIntervalComponent, RecurringIntervalCountIsNonNegativeWhenBounded,
    RecurringIntervalOmittedCountDenotesUnboundedOccurrences,
    RecurringIntervalUsesRepeatDesignator, ReducedCalendarDatePrecisionEvidence,
    ReducedLocalTimePrecisionEvidence, RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator, Rfc3339FractionUsesDotSeparator,
    Rfc3339FractionalSecondsAreOnlyRarelyUsedOption, Rfc3339GeneratorsShouldUseUppercaseTAndZ,
    Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
    Rfc3339LocalityDisplayMayTranslateUtcToLocalTime, Rfc3339OffsetIsUtcOrNumeric,
    Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory, Rfc3339RequiresUtcRelationship,
    Rfc3339TimestampExcludesRedundantWeekdayInformation,
    Rfc3339UnknownLocalOffsetUsesZuluDesignator, Rfc3339UnqualifiedLocalTimeForbidden,
    Rfc3339UsesExtendedCalendarDate, Rfc3339UsesFourDigitYear, Rfc3339UsesFullTime,
    RoundingModeDeclared, SeasonCodeDeclaresNamedSeason, SeasonCodeDeclaresSeasonScope,
    SeasonalExpressionUsesSeasonCodeInMonthSlot, SeasonalExpressionUsesYearAndSeasonForm,
    SubsecondDigitsPreserved, TimestampRepresentsFixedInstant,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UncertaintyAndApproximationMayBeCombined, UncertaintyQualificationDeclared,
    UnknownNamedTimeZoneIdentifierTreatedAsInconsistency, UtcDesignatorIsUppercaseZ,
    UtcOffsetValid, ZoneOffsetResolvedForRepresentedInstant, ZonedDateTimeHasNamedZone,
};

/// Aggregate proof that lossy conversion authority is explicit and fully described.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct LossyConversionAuthorityValid {
    /// Lossy conversion was explicitly authorized.
    explicit_lossy_authority: ConversionRequiresExplicitAuthorityWhenLossy,
    /// Precision reduction was declared.
    precision_reduction: PrecisionReductionDeclared,
    /// The rounding mode was declared.
    rounding: RoundingModeDeclared,
    /// The conversion dropped subsecond precision.
    dropped_digits: ConversionDropsSubsecondPrecision,
}

/// Aggregate proof that a standards-governed mutual-agreement authority is explicit and lawful.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct MutualAgreementAuthorityValid {
    /// The agreement-governed scope carried by the authority sidecar.
    scope: MutualAgreementAuthorityScopeEvidence,
}

/// Aggregate proof that a named zone carries generic named-zone identity.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct NamedTimeZoneIdentityValid {
    /// The zone uses an IANA time-zone identifier.
    zone_identifier: NamedTimeZoneUsesIanaIdentifier,
    /// The identifier excludes the forbidden `"."` and `".."` path segments.
    zone_segments: NamedTimeZoneIdentifierExcludesDotSegments,
    /// The identifier is interpreted case-sensitively.
    case_sensitivity: NamedTimeZoneIdentifierIsCaseSensitive,
    /// The zone is not merely a numeric-offset alias.
    named_zone: NamedTimeZoneIsNotNumericOffsetAlias,
    /// The zone preserves civil-time rule identity beyond the current offset.
    civil_rules: NamedTimeZoneRetainsCivilRuleIdentity,
}

/// Aggregate proof that named-zone interpretation tracks current TZDB revision semantics.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct NamedTimeZoneInterpretationTracksTzdbRevision {
    /// The timestamp already carries named-zone identity.
    zoned: ZonedDateTimeHasNamedZone,
    /// Interpretation follows the TZDB rules current at interpretation time.
    current_rules: NamedTimeZoneMeaningUsesCurrentTzdbRules,
    /// Unknown names under revision skew are treated as inconsistencies.
    unknown_name: UnknownNamedTimeZoneIdentifierTreatedAsInconsistency,
}

/// Aggregate proof that the timestamp offset agrees with the named-zone rules.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetConsistentWithNamedZone {
    /// The timestamp denotes a fixed instant.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The timestamp carries named-zone identity.
    zoned: ZonedDateTimeHasNamedZone,
    /// The named-zone rules resolve the represented offset.
    resolved_offset: ZoneOffsetResolvedForRepresentedInstant,
}

/// Aggregate proof that an offset date-time representation is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetDateTimeValid {
    /// The local date-time component is valid.
    local: LocalDateTimeValid,
    /// The UTC relationship is represented by a valid numeric offset.
    offset: UtcOffsetValid,
    /// The UTC designator, when used instead of a numeric offset, is uppercase `Z`.
    utc_designator: UtcDesignatorIsUppercaseZ,
}

/// Aggregate proof that offset-only semantics remain weaker than named-zone semantics.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetOnlyZoneSemanticsLimited {
    /// The timestamp carries a valid numeric UTC offset.
    offset: UtcOffsetValid,
    /// A numeric offset alone does not identify a named zone.
    not_a_zone: NumericOffsetDoesNotIdentifyNamedZone,
}

/// Aggregate proof that an offset time-zone annotation is consistent with the timestamp.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OffsetTimeZoneAnnotationConsistentWithTimestamp {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The annotation uses an offset time-zone form.
    offset_zone: OffsetTimeZoneAnnotationPresent,
    /// The suffix offset repeats the RFC 3339 timestamp offset.
    repeated_offset: OffsetTimeZoneRepeatsTimestampOffset,
    /// Offset time zones are only a discouraged compatibility form.
    discouraged_use: OffsetTimeZoneUseIsStronglyDiscouraged,
    /// The offset-zone annotation was not synthesized by copying the timestamp offset.
    not_synthesized: OffsetTimeZoneMustNotBeSynthesizedFromTimestampOffset,
}

/// Aggregate proof that a complete ordinal date is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OrdinalDateValid {
    /// The representation carries year and day-of-year fields.
    shape: OrdinalDateHasYearAndDayOfYear,
    /// The ordinal day is in range for the target year.
    day: OrdinalDayInRangeOneToThreeHundredSixtySix,
}

/// Aggregate proof that other-than-complete recurring-interval semantics are established.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OtherThanCompleteRecurringIntervalRepresentationSemanticsValid {
    /// The recurring-interval wrapper is structurally valid.
    recurring_interval: RecurringIntervalFormValid,
}

/// Aggregate proof that subsecond precision was preserved.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct PrecisionPreserved {
    /// The source subsecond precision is explicitly declared.
    precision: FractionalSecondPrecisionDeclared,
    /// The source fraction digits form a contiguous decimal suffix.
    contiguous_digits: FractionalSecondDigitsAreContiguous,
    /// The exchange preserved all subsecond digits.
    preserved_digits: SubsecondDigitsPreserved,
}

/// Aggregate proof that an extended temporal qualification is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct QualifiedTemporalExpressionValid {
    /// The expression explicitly declares uncertainty qualification.
    uncertainty: UncertaintyQualificationDeclared,
    /// The expression explicitly declares approximation qualification.
    approximation: ApproximationQualificationDeclared,
    /// The qualification scope is explicitly declared.
    scope: QualificationScopeDeclared,
    /// The placement law for the qualification marker is explicitly carried.
    placement: QualificationPlacementEvidence,
    /// Combined uncertainty and approximation semantics are available when needed.
    combined: UncertaintyAndApproximationMayBeCombined,
}

/// Aggregate proof that a temporal value and its qualification sidecar form a lawful exchange.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct QualifiedTemporalValueValid {
    /// The qualification sidecar is structurally valid.
    qualification: QualifiedTemporalExpressionValid,
}

/// Aggregate proof that a recurring interval representation is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct RecurringIntervalFormValid {
    /// The representation begins with the repetition designator.
    repeat_designator: RecurringIntervalUsesRepeatDesignator,
    /// Any bounded recurrence count is non-negative.
    repeat_count: RecurringIntervalCountIsNonNegativeWhenBounded,
    /// Omitting the recurrence count denotes an unbounded occurrence stream.
    omitted_count: RecurringIntervalOmittedCountDenotesUnboundedOccurrences,
    /// The repetition prefix is followed by an interval component.
    interval_component: RecurringIntervalCarriesIntervalComponent,
}

/// Aggregate proof that a recurring interval with repeat-rule refinement is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct RecurringIntervalWithRepeatRuleValid {
    /// The recurring-interval prefix and interval component are structurally valid.
    recurring_interval: RecurringIntervalFormValid,
    /// The attached repeat rule carries its full repeat-law sidecars.
    repeat_rule: RepeatRuleValid,
}

/// Aggregate proof that a reduced-precision calendar date is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ReducedCalendarDateValid {
    /// The representation is a Gregorian calendar date.
    calendar: CalendarDateUsesGregorianCalendar,
    /// The declared precision branch is structurally valid.
    precision: ReducedCalendarDatePrecisionEvidence,
}

/// Aggregate proof that a reduced-accuracy local time representation is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct ReducedLocalTimeValid {
    /// The hour value is in range.
    hour: HourInRangeZeroToTwentyFour,
    /// The declared precision branch is structurally valid.
    precision: ReducedLocalTimePrecisionEvidence,
    /// End-of-day usage of hour 24 is legal.
    end_of_day: TwentyFourHourReservedForEndOfDay,
    /// End-of-day hour 24 uses terminal zero minute, second, and fraction values only.
    terminal_end_of_day: TwentyFourHourRequiresZeroMinuteSecondAndFraction,
    /// Fraction syntax uses an ISO 8601 decimal sign.
    fraction_sign: FractionUsesDecimalSign,
    /// Any fraction attaches to the lowest-order present component.
    fraction_target: FractionAppliesToLowestOrderComponent,
}

/// Aggregate proof that a repeat rule is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct RepeatRuleValid {
    /// Repeat rules use the frequency designator.
    frequency: RepeatRuleUsesFrequencyDesignator,
    /// Eligible time intervals are explicitly declared.
    eligible_intervals: RepeatRuleDeclaresEligibleTimeIntervals,
    /// Selection rules apply within the eligible intervals.
    selection: RepeatRuleSelectionAppliesWithinEligibleIntervals,
    /// Evaluation inherits component information from the initial start date.
    inheritance: RepeatRuleEvaluationInheritsInitialStartComponentInformation,
}

/// Aggregate proof that RFC 3339 display-localization guidance is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct Rfc3339DisplayGuidanceValid {
    /// Locality-oriented display transformation may translate UTC timestamps into local time.
    utc_to_local_translation: Rfc3339LocalityDisplayMayTranslateUtcToLocalTime,
}

/// Aggregate proof that RFC 3339 generation guidance is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct Rfc3339GenerationGuidanceValid {
    /// Fractional seconds are the only rarely used profile option.
    secfrac_rarity: Rfc3339FractionalSecondsAreOnlyRarelyUsedOption,
    /// Generators should prefer uppercase `T` and `Z`.
    uppercase_tz: Rfc3339GeneratorsShouldUseUppercaseTAndZ,
    /// Application profiles may choose to allow a readability-motivated space separator.
    space_separator_option: Rfc3339ApplicationsMayAllowSpaceDateTimeSeparator,
    /// Inserted leap-second timestamps are not generated before announcement.
    leap_second_announcement: Rfc3339LeapSecondGenerationRequiresPriorAnnouncement,
}

/// Aggregate proof that RFC 3339 lexical ordering preconditions are structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct Rfc3339LexicalOrderingSemanticsValid {
    /// The underlying timestamp shape is a valid RFC 3339 timestamp.
    timestamp: Rfc3339TimestampValid,
    /// Most fields and punctuation are mandatory in the profile.
    mandatory_shape: Rfc3339ProfileMakesMostFieldsAndPunctuationMandatory,
}

/// Aggregate proof that an RFC 3339 timestamp is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct Rfc3339TimestampValid {
    /// The timestamp rests on a valid offset date-time representation.
    offset_date_time: OffsetDateTimeValid,
    /// The year uses the RFC 3339 four-digit profile.
    year: Rfc3339UsesFourDigitYear,
    /// The date uses the RFC 3339 `full-date` profile.
    full_date: Rfc3339UsesExtendedCalendarDate,
    /// The time uses the RFC 3339 `full-time` profile.
    full_time: Rfc3339UsesFullTime,
    /// The timestamp does not carry redundant weekday information.
    no_redundant_weekday: Rfc3339TimestampExcludesRedundantWeekdayInformation,
    /// The timestamp carries an explicit relationship to UTC.
    utc_relationship: Rfc3339RequiresUtcRelationship,
    /// Local time without an offset or `Z` is forbidden.
    no_unqualified_local: Rfc3339UnqualifiedLocalTimeForbidden,
    /// Fractional seconds use `.` rather than the broader ISO 8601 decimal options.
    secfrac_separator: Rfc3339FractionUsesDotSeparator,
    /// The UTC relationship is encoded as `Z` or a numeric offset.
    offset_encoding: Rfc3339OffsetIsUtcOrNumeric,
    /// Unknown local-offset semantics use the `Z` designator rather than legacy `-00:00`.
    unknown_offset: Rfc3339UnknownLocalOffsetUsesZuluDesignator,
}

/// Aggregate proof that a seasonal temporal expression is structurally valid.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct SeasonalTemporalExpressionValid {
    /// The expression uses a year-and-season form.
    form: SeasonalExpressionUsesYearAndSeasonForm,
    /// The season code occupies the month slot of a year-month-shaped representation.
    month_slot: SeasonalExpressionUsesSeasonCodeInMonthSlot,
    /// The season code declares a named season.
    named_season: SeasonCodeDeclaresNamedSeason,
    /// The season code declares its season scope.
    season_scope: SeasonCodeDeclaresSeasonScope,
}
