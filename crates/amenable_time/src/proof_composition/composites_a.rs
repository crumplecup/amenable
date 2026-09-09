//! Aggregate temporal proof propositions (B–E), ported from
//! `elicit_temporal::contracts::proof_composition`. Each aggregate is
//! folded with its `*Evidence` bundle: the aggregate's fields ARE its
//! sub-claims, and `#[derive(Witness)]` makes the composite proof the
//! structural product of its members' proofs. See [`super`] for the
//! design; leaf `Witness<V>` impls land in the backend crates.

use crate::{
    BeforeOrAfterQualificationIsLevelTwoOnly, BeforeOrOnDateUsesLeadingDoubleDotQualifier,
    BeforeYearOneValueUsesTrailingBSuffix, CalendarDateHasYearMonthDay,
    CalendarDateUsesGregorianCalendar, CalendarDayWithinMonthBounds,
    CalendarMonthInRangeOneToTwelve, CenturyOrdinalInRangeZeroToNinetyNine, CompleteDateEvidence,
    CompleteDurationEndIntervalSubstitutionEvidence,
    CompleteStartDurationIntervalSubstitutionEvidence,
    CompleteStartEndIntervalSubstitutionEvidence, ConversionPreservesRepresentedInstant,
    ConversionSourceSemanticKindDeclared, ConversionTargetSemanticKindDeclared,
    CriticalTimeZoneSuffixInconsistencyRequiresAction, DateIdentifiesPositionWithinCalendar,
    DateTimeFormulaCombinesTemporalValueWithDuration, DateTimeFormulaEvaluationModeDeclared,
    DateTimeFormulaTruncatesAtComponentBoundaries, DateTimeFormulaUsesCarryOverSemantics,
    DecadeOrdinalInRangeZeroToNineHundredNinetyNine, DurationAlternativeFormEvidence,
    DurationDesignatorRepresentationEvidence, DurationUsesPeriodDesignator,
    ElectiveTimeZoneSuffixInconsistencyMayBeHandled,
    ExplicitDateTimeTimePortionMayBeReducedPrecision,
    ExplicitDateTimeUsesDateThenTimeConcatenation,
    ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation,
    ExplicitDateWithShiftUsesDateThenShiftConcatenation, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence, ExplicitDurationUsesDurationalUnitDesignators,
    ExplicitTemporalFormValid, ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary,
    ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents, ExplicitTimeIntervalValid,
    ExplicitTimeOfDayValid, ExplicitTimeShiftValid, ExtendedIntervalBoundarySemanticsValid,
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfRecipientsMayIgnoreElectiveSuffixTags, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    IxdtfTimestampValid, LeapDayOccursOnlyInLeapYear, LossyConversionAuthorityValid,
    OnOrAfterDateUsesTrailingDoubleDotQualifier, OrdinalDateValid, PrecisionPreserved,
    QualifiedTemporalExpressionValid, RecurringIntervalFormValid, TemporalOrderingPreserved,
    UnspecifiedComponentExpressionValid, WeekDateValid,
};

/// Aggregate proof that backend-conversion semantics are explicitly declared.
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
pub struct BackendConversionSemanticsValid {
    /// The source semantic kind is explicitly declared.
    source_kind: ConversionSourceSemanticKindDeclared,
    /// The target semantic kind is explicitly declared.
    target_kind: ConversionTargetSemanticKindDeclared,
    /// The conversion preserves the represented instant.
    instant: ConversionPreservesRepresentedInstant,
    /// The conversion preserves temporal ordering.
    ordering: TemporalOrderingPreserved,
}

/// Aggregate proof that a complete calendar date is structurally valid.
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
pub struct CalendarDateValid {
    /// The representation is a Gregorian calendar date.
    calendar: CalendarDateUsesGregorianCalendar,
    /// The representation carries year, month, and day fields.
    shape: CalendarDateHasYearMonthDay,
    /// The month is within the legal ISO 8601 range.
    month: CalendarMonthInRangeOneToTwelve,
    /// The day is legal for the specific month and year.
    day: CalendarDayWithinMonthBounds,
    /// Leap-day usage satisfies the Gregorian leap-year rule.
    leap_day: LeapDayOccursOnlyInLeapYear,
}

/// Aggregate proof that a Gregorian calendar century representation is structurally valid.
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
pub struct CenturyValid {
    /// The century ordinal is within the legal ISO range.
    ordinal: CenturyOrdinalInRangeZeroToNinetyNine,
    /// Before-year-one syntax, when used, is declared by a trailing `B` suffix.
    before_year_one: Option<BeforeYearOneValueUsesTrailingBSuffix>,
}

/// Aggregate proof that complete-interval substitution semantics are structurally valid.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum CompleteIntervalSubstitutionSemanticsValid {
    /// Established via the `CompleteStartEndIntervalSubstitutionEvidence` decomposition.
    CompleteStartEndIntervalSubstitution(CompleteStartEndIntervalSubstitutionEvidence),
    /// Established via the `CompleteStartDurationIntervalSubstitutionEvidence` decomposition.
    CompleteStartDurationIntervalSubstitution(CompleteStartDurationIntervalSubstitutionEvidence),
    /// Established via the `CompleteDurationEndIntervalSubstitutionEvidence` decomposition.
    CompleteDurationEndIntervalSubstitution(CompleteDurationEndIntervalSubstitutionEvidence),
}

impl core::default::Default for CompleteIntervalSubstitutionSemanticsValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::CompleteStartEndIntervalSubstitution(core::default::Default::default())
    }
}

/// Aggregate proof that complete recurring-interval representation semantics are established.
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
pub struct CompleteRecurringIntervalRepresentationSemanticsValid {
    /// The recurring-interval wrapper is structurally valid.
    recurring_interval: RecurringIntervalFormValid,
}

/// Aggregate proof that a conversion is lossless.
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
pub struct ConversionLossless {
    /// Shared backend-conversion semantics are established.
    conversion: BackendConversionSemanticsValid,
    /// The conversion preserves subsecond precision.
    precision: PrecisionPreserved,
}

/// Aggregate proof that a conversion truncates subseconds.
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
pub struct ConversionTruncatesSubseconds {
    /// Shared backend-conversion semantics are established.
    conversion: BackendConversionSemanticsValid,
    /// Lossy conversion authority is explicit and fully described.
    lossy_authority: LossyConversionAuthorityValid,
}

/// Aggregate proof that a critical time-zone suffix inconsistency is handled lawfully.
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
pub struct CriticalTimeZoneInconsistencyHandlingValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The time-zone annotation uses the RFC 9557 bracketed time-zone syntax.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// Criticality is expressed with a leading `!` when present.
    critical_flag: IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    /// Critical suffixes require processing or explicit error handling.
    critical_consumption: IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    /// A critical time-zone inconsistency requires the application to act.
    must_act: CriticalTimeZoneSuffixInconsistencyRequiresAction,
}

/// Aggregate proof that an explicit temporal result was lawfully produced by date-time formula evaluation.
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
pub struct DateTimeFormulaEvaluationResultValid {
    /// The originating formula is structurally valid.
    formula: DateTimeFormulaValid,
    /// The originating formula declares a lawful evaluation family.
    semantics: DateTimeFormulaEvaluationSemanticsValid,
    /// The produced explicit temporal value is structurally valid.
    result: ExplicitTemporalFormValid,
}

/// Aggregate proof that date-time formula evaluation semantics are fully declared.
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
pub struct DateTimeFormulaEvaluationSemanticsValid {
    /// The underlying formula is structurally valid.
    formula: DateTimeFormulaValid,
    /// The formula declares its evaluation family.
    evaluation_mode: DateTimeFormulaEvaluationModeDeclared,
}

/// Aggregate proof that a date-time formula is structurally valid.
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
pub struct DateTimeFormulaValid {
    /// The formula combines an explicit temporal value with a duration.
    combination: DateTimeFormulaCombinesTemporalValueWithDuration,
    /// Overflow is carried across component boundaries.
    carry_over: DateTimeFormulaUsesCarryOverSemantics,
    /// Evaluation truncates at component boundaries when required.
    truncation: DateTimeFormulaTruncatesAtComponentBoundaries,
}

/// Aggregate proof that an ISO 8601 date representation is structurally valid.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum DateValid {
    /// Gregorian calendar-date representation.
    Calendar {
        /// The calendar-date representation is structurally valid.
        date: CalendarDateValid,
        /// A date identifies a position within the calendar.
        semantics: DateIdentifiesPositionWithinCalendar,
    },
    /// Ordinal-date representation.
    Ordinal {
        /// The ordinal-date representation is structurally valid.
        date: OrdinalDateValid,
        /// A date identifies a position within the calendar.
        semantics: DateIdentifiesPositionWithinCalendar,
    },
    /// Week-date representation.
    Week {
        /// The week-date representation is structurally valid.
        date: WeekDateValid,
        /// A date identifies a position within the calendar.
        semantics: DateIdentifiesPositionWithinCalendar,
    },
}

impl core::default::Default for DateValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Calendar {
            date: core::default::Default::default(),
            semantics: core::default::Default::default(),
        }
    }
}

/// Aggregate proof that a complete explicit-form date with shift is structurally valid.
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
pub struct DateWithShiftValid {
    /// The date branch is one of the lawful explicit complete-date families.
    date: CompleteDateEvidence,
    /// The explicit date precedes the explicit time shift.
    concatenation: ExplicitDateWithShiftUsesDateThenShiftConcatenation,
    /// The attached shift is a lawful explicit-form time shift.
    shift: ExplicitTimeShiftValid,
}

/// Aggregate proof that a Gregorian calendar decade representation is structurally valid.
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
pub struct DecadeValid {
    /// The decade ordinal is within the legal ISO range.
    ordinal: DecadeOrdinalInRangeZeroToNineHundredNinetyNine,
    /// Before-year-one syntax, when used, is declared by a trailing `B` suffix.
    before_year_one: Option<BeforeYearOneValueUsesTrailingBSuffix>,
}

/// Aggregate proof that a duration representation is structurally valid.
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
pub struct DurationFormValid {
    /// The representation begins with the duration designator.
    period: DurationUsesPeriodDesignator,
    /// The concrete representation family and its associated laws.
    representation: DurationRepresentationSemanticsValid,
}

/// Aggregate proof that a duration representation family is explicit and lawful.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum DurationRepresentationSemanticsValid {
    /// The duration uses the designator-based representation.
    Designator(DurationDesignatorRepresentationEvidence),
    /// The duration uses the alternative complete representation.
    Alternative(DurationAlternativeFormEvidence),
}

impl core::default::Default for DurationRepresentationSemanticsValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Designator(core::default::Default::default())
    }
}

/// Aggregate proof that an elective time-zone suffix inconsistency is handled lawfully.
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
pub struct ElectiveTimeZoneInconsistencyHandlingValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The time-zone annotation uses the RFC 9557 bracketed time-zone syntax.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// Elective time-zone suffixes may be ignored by recipients.
    elective_consumption: IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    /// An elective time-zone inconsistency permits, but does not require, action.
    may_act: ElectiveTimeZoneSuffixInconsistencyMayBeHandled,
}

/// Aggregate proof that Level 1 enhanced-interval semantics are structurally valid.
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
pub struct EnhancedIntervalLevelOneSemanticsValid {
    /// The underlying extended-interval boundary semantics are structurally valid.
    boundaries: ExtendedIntervalBoundarySemanticsValid,
    /// Boundary-date qualification semantics are available.
    qualification: QualifiedTemporalExpressionValid,
}

/// Aggregate proof that Level 2 enhanced-interval semantics are structurally valid.
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
pub struct EnhancedIntervalLevelTwoSemanticsValid {
    /// Level 2 semantics extend the Level 1 enhanced-interval surface.
    level_one: EnhancedIntervalLevelOneSemanticsValid,
    /// Unspecified-component semantics are available for boundary dates.
    unspecified_components: UnspecifiedComponentExpressionValid,
    /// Before-or-after qualification is a Level 2 feature.
    before_or_after_level: BeforeOrAfterQualificationIsLevelTwoOnly,
    /// The start boundary may use the leading `..` before-or-on qualifier.
    before_or_on_start: BeforeOrOnDateUsesLeadingDoubleDotQualifier,
    /// The end boundary may use the trailing `..` on-or-after qualifier.
    on_or_after_end: OnOrAfterDateUsesTrailingDoubleDotQualifier,
}

/// Aggregate proof that a CalConnect explicit date-time form is structurally valid.
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
pub struct ExplicitDateTimeValid {
    /// The date branch is one of the lawful explicit complete-date families.
    date: CompleteDateEvidence,
    /// The time branch is a lawful explicit local-time-of-day form.
    time: ExplicitTimeOfDayValid,
    /// The explicit date precedes the explicit time in the combined representation.
    concatenation: ExplicitDateTimeUsesDateThenTimeConcatenation,
    /// The time branch may use reduced precision.
    reduced_precision: ExplicitDateTimeTimePortionMayBeReducedPrecision,
}

/// Aggregate proof that a CalConnect explicit date-time-with-shift form is structurally valid.
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
pub struct ExplicitDateTimeWithShiftValid {
    /// The explicit local date-time branch is structurally valid.
    local: ExplicitDateTimeValid,
    /// The explicit date-time precedes the explicit time shift.
    concatenation: ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation,
    /// The attached shift is a lawful explicit-form time shift.
    shift: ExplicitTimeShiftValid,
}

/// Aggregate proof that a CalConnect explicit duration form is structurally valid.
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
pub struct ExplicitDurationValid {
    /// Durational unit designators are used.
    units: ExplicitDurationUsesDurationalUnitDesignators,
    /// The explicit duration declares its representation family.
    representation: ExplicitDurationRepresentationEvidence,
    /// Signed negative-duration semantics are explicitly available.
    sign: ExplicitDurationMayBeNegative,
    /// Fractional lowest-order unit semantics are explicitly available.
    fractional: ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// Exactness-family semantics are explicitly carried when needed.
    semantics: ExplicitDurationSemanticEvidence,
}

/// Aggregate proof that explicit-interval duration-substitution semantics are structurally valid.
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
pub struct ExplicitIntervalDurationSubstitutionSemanticsValid {
    /// The underlying explicit interval representation is structurally valid.
    interval: ExplicitTimeIntervalValid,
    /// A missing boundary is inferable from the substituted explicit duration.
    substitution: ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary,
}

/// Aggregate proof that explicit-interval trailing-end inheritance semantics are structurally valid.
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
pub struct ExplicitIntervalEndComponentInheritanceSemanticsValid {
    /// The underlying explicit interval representation is structurally valid.
    interval: ExplicitTimeIntervalValid,
    /// Omitted higher-order trailing-end components inherit from the leading endpoint.
    inheritance: ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents,
}
