//! Aggregate temporal proof propositions (S–Z), ported from
//! `elicit_temporal::contracts::proof_composition`. Each aggregate is
//! folded with its `*Evidence` bundle: the aggregate's fields ARE its
//! sub-claims, and `#[derive(Witness)]` makes the composite proof the
//! structural product of its members' proofs. See [`super`] for the
//! design; leaf `Witness<V>` impls land in the backend crates.

use crate::{
    ConversionPreservesTemporalOrdering, ExplicitTimeOfDayValid,
    ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation, ExplicitTimeShiftValid,
    IxdtfTimeZoneSuffixUsesBracketedNameOrOffset, IxdtfTimestampValid,
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, LocalDateTimeDoesNotIdentifyFixedInstant,
    LocalDateTimeMayBeAmbiguousAtZoneTransition, LocalDateTimeMayFallInZoneTransitionGap,
    LocalTimeValid, NamedTimeZoneIdentifierExcludesDotSegments, NamedTimeZoneUsesIanaIdentifier,
    NamedZoneAttachmentEvidence, OffsetDateTimeIdentifiesSingleInstant, OffsetDateTimeValid,
    ReducedLocalTimeValid, Rfc3339LocalOffsetNotUnknown,
    Rfc3339UnknownLocalOffsetUsesZuluDesignator, SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionRuleDayOfMonthUsesDayExpression,
    SelectionRuleHourUsesHourExpression, SelectionRuleMinuteUsesMinuteExpression,
    SelectionRuleMonthUsesMonthExpression, SelectionRulePositionAppliesLast,
    SelectionRulePositionUsesInstanceDesignatorSuffix, SelectionRuleSecondUsesSecondExpression,
    SelectionRuleWeekDayUsesDayOfWeekExpression, SelectionRuleWeekUsesWeekExpression,
    SelectionRulesApplyWithinSelectedResults, SelectionWithDurationUsesDurationSuffix,
    StandardTimeDerivedFromUtcByLocalShift, StandardTimeOfDayUsesStandardTimeScale,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubYearGroupingKindEvidence,
    TemporalSetCarriesMultipleMembers, TemporalSetForbidsInternalWhitespace,
    TemporalSetMemberSeparatorDeclared, TemporalSetOpenRangeUsesBoundaryDoubleDot,
    TemporalSetRangeNeighborhoodSharesPrecision, TemporalSetRangeUsesInclusiveDoubleDotSemantics,
    TimeIntervalBoundaryOrDurationFormDeclared, TimeIntervalHasTwoComponents,
    TimeIntervalUsesSolidusSeparator, TimeOfDayOccursWithinCalendarDay,
    TimeShiftIsConstantDurationBetweenTimeScales, TimestampHasExplicitUtcOffset,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
    UtcDesignatorIsUppercaseZ, UtcIsReferenceTimeScale, UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
    UtcOfDayUsesTrailingZuluDesignatorImmediately, UtcOffsetCarriesSignHourAndOptionalMinute,
    UtcOffsetHourInRangeZeroToTwentyThree, UtcOffsetPrecisionEvidence,
    UtcTimelineOrderingAppliesToFixedInstants, WeekDateHasWeekYearWeekAndWeekday,
    WeekNumberInRangeOneToFiftyThree, WeekdayInRangeOneToSeven,
    ZoneOffsetResolvedForRepresentedInstant, ZoneTransitionAmbiguityDeclared,
    ZoneTransitionDisambiguationAuthorityDeclared, ZoneTransitionGapDeclared,
    ZoneTransitionGapHandlingAuthorityDeclared, ZonedTimestampEvidence,
    ZuluTimeZoneSuffixAvoidsOffsetInconsistency,
};

/// Aggregate proof that a selection expression is structurally valid.
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
pub struct SelectionExpressionValid {
    /// Selection expressions use the `L...N` delimiters.
    delimiters: SelectionExpressionUsesSelectionDelimiters,
    /// Selection expressions use the standard rule vocabulary.
    vocabulary: SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    /// Month-selection rules use the month expression family.
    month_rule: SelectionRuleMonthUsesMonthExpression,
    /// Week-selection rules use the week expression family.
    week_rule: SelectionRuleWeekUsesWeekExpression,
    /// Day-of-month selection rules use the day expression family.
    day_of_month_rule: SelectionRuleDayOfMonthUsesDayExpression,
    /// Weekday selection rules use the day-of-week expression family.
    weekday_rule: SelectionRuleWeekDayUsesDayOfWeekExpression,
    /// Hour-selection rules use the hour expression family.
    hour_rule: SelectionRuleHourUsesHourExpression,
    /// Minute-selection rules use the minute expression family.
    minute_rule: SelectionRuleMinuteUsesMinuteExpression,
    /// Second-selection rules use the second expression family.
    second_rule: SelectionRuleSecondUsesSecondExpression,
    /// Subsequent components apply within previously selected results.
    nesting: SelectionRulesApplyWithinSelectedResults,
    /// Single-instance selection is explicitly modeled.
    single_instance: SelectionExpressionMaySelectSingleInstance,
    /// Position rules use an integer followed by the instance designator.
    position_syntax: SelectionRulePositionUsesInstanceDesignatorSuffix,
    /// Position rules apply after earlier selection rules.
    position: SelectionRulePositionAppliesLast,
    /// Selection-with-duration uses an explicit duration suffix.
    duration_window: SelectionWithDurationUsesDurationSuffix,
}

/// Aggregate proof that a standard-time-of-day representation is structurally valid.
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
pub struct StandardTimeOfDayValid {
    /// The wall-clock time-of-day representation is structurally valid.
    time: LocalTimeValid,
    /// The governing standard-time scale is established.
    scale: StandardTimeValid,
    /// The time-of-day interpretation uses standard time.
    semantics: StandardTimeOfDayUsesStandardTimeScale,
}

/// Aggregate proof that standard-time semantics are established.
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
pub struct StandardTimeValid {
    /// The reference time scale is UTC.
    reference: UtcTimeScaleValid,
    /// The local shift from UTC is carried explicitly.
    shift: UtcOffsetValid,
    /// Standard time is derived from UTC by an applicable local shift.
    derivation: StandardTimeDerivedFromUtcByLocalShift,
    /// The shift is a constant duration between the time scales.
    constant_shift: TimeShiftIsConstantDurationBetweenTimeScales,
}

/// Aggregate proof that a Level 2 sub-year grouping expression is structurally valid.
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
pub struct SubYearGroupingExpressionValid {
    /// The expression uses a year-and-grouping form.
    form: SubYearGroupingExpressionUsesYearAndGroupingForm,
    /// The grouping code occupies the month slot of a year-month-shaped representation.
    month_slot: SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    /// The specific grouping family declared by the code.
    grouping: SubYearGroupingKindEvidence,
}

/// Aggregate proof that a conversion preserved ordering on the UTC timeline.
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
pub struct TemporalOrderingPreserved {
    /// The source timestamp denotes a fixed instant.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// Ordering is evaluated on the UTC timeline.
    utc_timeline: UtcTimelineOrderingAppliesToFixedInstants,
    /// The conversion preserved temporal ordering.
    preserved_ordering: ConversionPreservesTemporalOrdering,
}

/// Aggregate proof that a temporal set expression is structurally valid.
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
pub struct TemporalSetExpressionValid {
    /// Member expressions are explicitly separated.
    separator: TemporalSetMemberSeparatorDeclared,
    /// The set carries multiple temporal members.
    members: TemporalSetCarriesMultipleMembers,
}

/// Aggregate proof that refined temporal-set range semantics are structurally valid.
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
pub struct TemporalSetRangeSemanticsValid {
    /// The underlying temporal-set expression is structurally valid.
    set: TemporalSetExpressionValid,
    /// No internal whitespace appears within the expression.
    no_whitespace: TemporalSetForbidsInternalWhitespace,
    /// `..` denotes the inclusive values between bounded range endpoints.
    inclusive_range: TemporalSetRangeUsesInclusiveDoubleDotSemantics,
    /// Leading or trailing `..` denotes an open-ended boundary.
    open_range: TemporalSetOpenRangeUsesBoundaryDoubleDot,
    /// Values adjacent to a range share the same precision as the range expansion.
    precision: TemporalSetRangeNeighborhoodSharesPrecision,
}

/// Aggregate proof that a time interval representation is structurally valid.
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
pub struct TimeIntervalValid {
    /// The interval uses the solidus separator between its top-level components.
    separator: TimeIntervalUsesSolidusSeparator,
    /// The interval has exactly two top-level components.
    components: TimeIntervalHasTwoComponents,
    /// The interval declares its endpoint-or-duration form.
    form: TimeIntervalBoundaryOrDurationFormDeclared,
}

/// Aggregate proof that a complete explicit-form time of day with shift is structurally valid.
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
pub struct TimeOfDayWithShiftValid {
    /// The explicit local-time-of-day branch is structurally valid.
    time: ExplicitTimeOfDayValid,
    /// The explicit time precedes the explicit time shift.
    concatenation: ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation,
    /// The attached shift is a lawful explicit-form time shift.
    shift: ExplicitTimeShiftValid,
}

/// Aggregate proof that an ISO 8601 time-of-day representation is structurally valid.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum TimeValid {
    /// Local clock-time representation.
    Local {
        /// The local clock-time representation is structurally valid.
        time: LocalTimeValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
    /// Reduced-accuracy local clock-time representation.
    ReducedLocal {
        /// The reduced local-time representation is structurally valid.
        time: ReducedLocalTimeValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
    /// UTC-of-day representation.
    Utc {
        /// The UTC-of-day representation is structurally valid.
        time: UtcOfDayValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
    /// Standard-time-of-day representation.
    Standard {
        /// The standard-time-of-day representation is structurally valid.
        time: StandardTimeOfDayValid,
        /// A time of day occurs within a calendar day.
        semantics: TimeOfDayOccursWithinCalendarDay,
    },
}

impl core::default::Default for TimeValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Local {
            time: core::default::Default::default(),
            semantics: core::default::Default::default(),
        }
    }
}

/// Aggregate proof that a timestamp denotes a single fixed instant.
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
pub struct TimestampRepresentsFixedInstant {
    /// The offset date-time representation is structurally valid.
    offset_date_time: OffsetDateTimeValid,
    /// The timestamp carries an explicit UTC relationship.
    explicit_offset: TimestampHasExplicitUtcOffset,
    /// The representation identifies a single instant on the UTC timeline.
    single_instant: OffsetDateTimeIdentifiesSingleInstant,
}

/// Aggregate proof that masked precision and unspecified-component semantics are structurally valid.
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
pub struct UnspecifiedComponentExpressionValid {
    /// Unspecified digits use the uppercase `X` placeholder.
    placeholder: UnspecifiedDigitUsesUppercaseXPlaceholder,
    /// Each `X` placeholder denotes an unspecified digit or component value.
    unspecified_value: UnspecifiedDigitsDeclareUnknownValue,
    /// Level 1 masking occupies one or more rightmost positions.
    level_one_tail_masking: LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    /// Level 2 masking may occur within a component.
    level_two_component_masking: LevelTwoUnspecifiedDigitsMayAppearWithinComponent,
}

/// Aggregate proof that a UTC-of-day representation is structurally valid.
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
pub struct UtcOfDayValid {
    /// The carried time-of-day representation is structurally valid.
    time: LocalTimeValid,
    /// The representation is anchored to the UTC reference time scale.
    scale: UtcTimeScaleValid,
    /// The value identifies a position within a UTC calendar day.
    position: UtcOfDayIdentifiesTimeWithinUtcCalendarDay,
    /// The UTC time-of-day payload is followed immediately by the `Z` designator.
    designator: UtcOfDayUsesTrailingZuluDesignatorImmediately,
    /// The UTC designator uses uppercase `Z`.
    uppercase_z: UtcDesignatorIsUppercaseZ,
}

/// Aggregate proof that the UTC relationship does not use unknown-local-offset semantics.
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
pub struct UtcOffsetKnown {
    /// The timestamp already carries a valid UTC-offset representation.
    offset: UtcOffsetValid,
    /// The timestamp uses a known local-offset relationship rather than updated unknown-offset semantics.
    known_convention: Rfc3339LocalOffsetNotUnknown,
}

/// Aggregate proof that a numeric UTC offset is structurally valid.
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
pub struct UtcOffsetValid {
    /// The offset includes a sign, an hour component, and optional minute precision.
    shape: UtcOffsetCarriesSignHourAndOptionalMinute,
    /// The offset hour is in range.
    hour: UtcOffsetHourInRangeZeroToTwentyThree,
    /// The declared precision branch is structurally valid.
    precision: UtcOffsetPrecisionEvidence,
}

/// Aggregate proof that the UTC reference time scale semantics are established.
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
pub struct UtcTimeScaleValid {
    /// UTC acts as the reference time scale.
    reference: UtcIsReferenceTimeScale,
}

/// Aggregate proof that a complete week date is structurally valid.
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
pub struct WeekDateValid {
    /// The representation carries week-year, week, and weekday fields.
    shape: WeekDateHasWeekYearWeekAndWeekday,
    /// The week number is in the legal ISO 8601 range.
    week: WeekNumberInRangeOneToFiftyThree,
    /// The weekday is in the legal ISO 8601 range.
    weekday: WeekdayInRangeOneToSeven,
}

/// Aggregate proof that ambiguous local-time resolution semantics are explicit and lawful.
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
pub struct ZoneTransitionAmbiguitySemanticsValid {
    /// The local date-time is still awaiting zone authority to identify an instant.
    local_semantics: LocalDateTimeDoesNotIdentifyFixedInstant,
    /// The local date-time can be ambiguous at a transition boundary.
    ambiguity_possibility: LocalDateTimeMayBeAmbiguousAtZoneTransition,
    /// The producer explicitly declares that a transition ambiguity is in play.
    ambiguity_declared: ZoneTransitionAmbiguityDeclared,
    /// The selected disambiguation and gap-handling policy is explicit.
    resolution_authority: ZoneTransitionResolutionAuthorityValid,
}

/// Aggregate proof that skipped local-time gap semantics are explicit and lawful.
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
pub struct ZoneTransitionGapSemanticsValid {
    /// The local date-time is still awaiting zone authority to identify an instant.
    local_semantics: LocalDateTimeDoesNotIdentifyFixedInstant,
    /// The local date-time can fall inside a skipped transition gap.
    gap_possibility: LocalDateTimeMayFallInZoneTransitionGap,
    /// The producer explicitly declares that a transition gap is in play.
    gap_declared: ZoneTransitionGapDeclared,
    /// The selected disambiguation and gap-handling policy is explicit.
    resolution_authority: ZoneTransitionResolutionAuthorityValid,
}

/// Aggregate proof that explicit local-to-zone resolution authority is fully declared.
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
pub struct ZoneTransitionResolutionAuthorityValid {
    /// The named zone is carried by an IANA time-zone identifier.
    zone_identifier: NamedTimeZoneUsesIanaIdentifier,
    /// The zone identifier excludes the forbidden `"."` and `".."` segments.
    zone_segments: NamedTimeZoneIdentifierExcludesDotSegments,
    /// Ambiguous local times use explicit disambiguation authority.
    disambiguation: ZoneTransitionDisambiguationAuthorityDeclared,
    /// Skipped local times use explicit gap-handling authority.
    gap_handling: ZoneTransitionGapHandlingAuthorityDeclared,
}

/// Aggregate proof that a timestamp carries named-zone identity.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ZonedDateTimeHasNamedZone {
    /// Established via the `NamedZoneAttachmentEvidence` decomposition.
    NamedZoneAttachment(NamedZoneAttachmentEvidence),
    /// Established via the `ZonedTimestampEvidence` decomposition.
    ZonedTimestamp(ZonedTimestampEvidence),
}

impl core::default::Default for ZonedDateTimeHasNamedZone {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::NamedZoneAttachment(core::default::Default::default())
    }
}

/// Aggregate proof that a `Z`-based time-zone timestamp avoids offset inconsistency.
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
pub struct ZuluTimeZoneInconsistencyAvoidanceValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The RFC 3339 portion uses `Z` to express that local offset information is unknown.
    unknown_local_offset: Rfc3339UnknownLocalOffsetUsesZuluDesignator,
    /// The time-zone annotation uses the RFC 9557 bracketed time-zone syntax.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// The timestamp therefore avoids asserting a conflicting local offset.
    no_inconsistency: ZuluTimeZoneSuffixAvoidsOffsetInconsistency,
    /// The named-zone rules resolve the represented offset for the instant.
    resolved_offset: ZoneOffsetResolvedForRepresentedInstant,
}
