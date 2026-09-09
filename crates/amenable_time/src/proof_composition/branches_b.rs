//! Shared `*Evidence` branch decompositions (E–Z) reused across
//! several aggregates, plus the per-form bundles behind the two
//! multi-credential aggregates. Same `#[derive(Evidence, Witness)]`
//! shape as the [`super::composites_a`] family.

use crate::{
    CalendarMonthInRangeOneToTwelve, CalendarYearThrough1582RequiresMutualAgreement,
    ComponentQualificationAppliesOnlyToMarkedComponent,
    ComponentQualificationUsesImmediateLeftPlacement, ContextDependentDurationSemanticsDeclared,
    ExactDurationSemanticsDeclared, ExpandedRepresentationRequiresAdditionalAgreement,
    ExplicitDurationCompositeRepresentationDeclared,
    ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder,
    ExplicitDurationRepresentationKindDeclared, ExponentialYearExponentIsPositiveInteger,
    ExponentialYearUsesPowerOfTenNotation,
    GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    GroupQualificationUsesImmediateRightPlacement, IxdtfTimestampValid,
    LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
    LetterPrefixedCalendarYearUsesLeadingYDesignator, MinuteInRangeZeroToFiftyNine,
    NamedTimeZoneAnnotationPresent, NamedTimeZoneIdentifierExcludesDotSegments,
    NamedTimeZoneIdentityValid, NamedTimeZoneIsNotNumericOffsetAlias,
    NamedTimeZoneRetainsCivilRuleIdentity, NamedTimeZoneUsesIanaIdentifier,
    NegativeCalendarYearUsesLeadingMinusSign,
    ProlepticGregorianDatesBefore1583RequireMutualAgreement,
    ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
    ReducedAccuracyLocalTimeUsesHourOnlyRepresentation, ReducedCalendarDateHasYearComponent,
    ReducedCalendarDateUsesYearMonthRepresentation, ReducedCalendarDateUsesYearOnlyRepresentation,
    SeasonCodeDeclaresNamedSeason, SeasonCodeDeclaresSeasonScope,
    SignificantDigitYearCountIsPositiveInteger, SignificantDigitYearUsesTrailingSSuffix,
    SpeculativeDurationSemanticsDeclared, SubYearGroupingCodeDeclaresQuadrimester,
    SubYearGroupingCodeDeclaresQuarter, SubYearGroupingCodeDeclaresSemestral,
    TimestampRepresentsFixedInstant, UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
    UtcOffsetMinuteInRangeZeroToFiftyNine,
};

/// Representation-specific evidence branch for a CalConnect explicit duration.
///
/// Normative source: CalConnect CC 18011:2018 §7.3-§7.5 — Representations
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExplicitDurationRepresentationEvidence {
    /// Simple representation semantics.
    Simple {
        /// The duration explicitly declares its representation family.
        representation_kind: ExplicitDurationRepresentationKindDeclared,
    },
    /// Composite representation semantics.
    Composite {
        /// The duration explicitly declares its representation family.
        representation_kind: ExplicitDurationRepresentationKindDeclared,
        /// The duration uses the composite representation family.
        composite: ExplicitDurationCompositeRepresentationDeclared,
    },
    /// Precedence representation semantics.
    Precedence {
        /// The duration explicitly declares its representation family.
        representation_kind: ExplicitDurationRepresentationKindDeclared,
        /// The representation preserves the declared evaluation order.
        precedence: ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder,
    },
}

impl core::default::Default for ExplicitDurationRepresentationEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Simple {
            representation_kind: core::default::Default::default(),
        }
    }
}

/// Exactness-specific evidence branch for a CalConnect explicit duration.
///
/// Normative source: CalConnect CC 18011:2018 §7.8-§7.10 — Exact, context-dependent, and speculative duration
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExplicitDurationSemanticEvidence {
    /// No exactness family is explicitly declared.
    None,
    /// Exact-duration semantics are declared.
    Exact {
        /// The duration is explicitly classified as exact.
        exact: ExactDurationSemanticsDeclared,
    },
    /// Context-dependent-duration semantics are declared.
    ContextDependent {
        /// The duration is explicitly classified as context-dependent.
        context_dependent: ContextDependentDurationSemanticsDeclared,
    },
    /// Speculative-duration semantics are declared.
    Speculative {
        /// The duration is explicitly classified as speculative.
        speculative: SpeculativeDurationSemanticsDeclared,
    },
}

impl core::default::Default for ExplicitDurationSemanticEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::None
    }
}

/// Evidence branch for the base lexical form of an ISO 8601-2 year extension.
///
/// Normative sources: ISO 8601-2:2019, 4.4.1, 4.4.2, 4.7.2, and 4.7.3.
/// Informative cross-check: public LOC EDTF Level 1 — Letter-prefixed calendar year;
/// Negative calendar year. Level 2 — Exponential year
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ExtendedYearBaseEvidence {
    /// A four-digit year form, optionally carrying the negative-year lexical law.
    FourDigit {
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<NegativeCalendarYearUsesLeadingMinusSign>,
    },
    /// A letter-prefixed year form with an integer year payload.
    LetterPrefixed {
        /// The representation uses the leading `Y` designator.
        prefix: LetterPrefixedCalendarYearUsesLeadingYDesignator,
        /// The absolute year magnitude exceeds four digits.
        magnitude: LetterPrefixedCalendarYearMagnitudeExceedsFourDigits,
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<NegativeCalendarYearUsesLeadingMinusSign>,
    },
    /// A letter-prefixed year form with exponential notation.
    Exponential {
        /// The representation uses the leading `Y` designator.
        prefix: LetterPrefixedCalendarYearUsesLeadingYDesignator,
        /// The representation uses `E` power-of-ten notation.
        exponential: ExponentialYearUsesPowerOfTenNotation,
        /// The exponent is a positive integer.
        exponent: ExponentialYearExponentIsPositiveInteger,
        /// The representation uses an explicit leading minus sign when negative.
        negative: Option<NegativeCalendarYearUsesLeadingMinusSign>,
    },
}

impl core::default::Default for ExtendedYearBaseEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::FourDigit {
            negative: core::default::Default::default(),
        }
    }
}

/// Evidence branch for the significant-digits suffix on an ISO 8601-2 year form.
///
/// Normative sources: ISO 8601-2:2019, 4.4.3 and 4.7.4.
/// Informative cross-check: public LOC EDTF Level 2 — Significant digits
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
pub struct ExtendedYearSignificantDigitsEvidence {
    /// The year form uses a trailing uppercase `S` suffix.
    suffix: SignificantDigitYearUsesTrailingSSuffix,
    /// The significant-digit count is a positive integer.
    count: SignificantDigitYearCountIsPositiveInteger,
}

/// Evidence branch for a standards-governed mutual-agreement authority scope.
///
/// Normative source: ISO 8601-1:2019, 5.2.2.
/// Open-text cross-checks: ISO/WD 8601-1:2016(E), 3.2.1, 4.1.2.1, and 4.1.2.4.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum MutualAgreementAuthorityScopeEvidence {
    /// Mutual agreement explicitly covers non-expanded calendar years through 1582.
    CalendarYearThrough1582 {
        /// The governed calendar-year restriction is established.
        clause: CalendarYearThrough1582RequiresMutualAgreement,
    },
    /// Mutual agreement explicitly covers proleptic Gregorian dates before 1583.
    ProlepticGregorianBefore1583 {
        /// The governed proleptic-date restriction is established.
        clause: ProlepticGregorianDatesBefore1583RequireMutualAgreement,
    },
    /// Mutual agreement explicitly covers expanded representations.
    ExpandedRepresentation {
        /// The governed expanded-representation restriction is established.
        clause: ExpandedRepresentationRequiresAdditionalAgreement,
    },
}

impl core::default::Default for MutualAgreementAuthorityScopeEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::CalendarYearThrough1582 {
            clause: core::default::Default::default(),
        }
    }
}

/// Evidence bundle for generic named-zone attachment to a fixed instant.
///
/// Normative sources: RFC 9557 §1.2 and §4.1.
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
pub struct NamedZoneAttachmentEvidence {
    /// The attached timestamp denotes a fixed instant.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The attached zone carries generic named-zone identity semantics.
    zone_identity: NamedTimeZoneIdentityValid,
}

/// Evidence bundle for a qualified temporal expression.
///
/// Normative source: ISO 8601-2:2019, 8.2.1, 8.2.2, 8.2.3, 8.4.4, 8.4.5,
/// 8.4.6, and 8.5.
/// Informative cross-check: public LOC EDTF Level 1 — Qualification of a date (complete);
/// Level 2 — Qualification
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum QualificationPlacementEvidence {
    /// The qualification marker appears immediately to the right and propagates leftward.
    GroupRight {
        /// The qualification marker is placed immediately to the right of the marked component.
        placement: GroupQualificationUsesImmediateRightPlacement,
        /// The qualification applies to the marked component and all more significant components.
        propagation: GroupQualificationAppliesToMarkedAndMoreSignificantComponents,
    },
    /// The qualification marker appears immediately to the left and applies only to that component.
    ComponentLeft {
        /// The qualification marker is placed immediately to the left of the marked component.
        placement: ComponentQualificationUsesImmediateLeftPlacement,
        /// The qualification applies only to the marked component.
        propagation: ComponentQualificationAppliesOnlyToMarkedComponent,
    },
}

impl core::default::Default for QualificationPlacementEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::GroupRight {
            placement: core::default::Default::default(),
            propagation: core::default::Default::default(),
        }
    }
}

/// Evidence branch for the declared reduced calendar-date precision.
///
/// Normative source: ISO 8601-1:2019, 5.2.2.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.1.2.3.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ReducedCalendarDatePrecisionEvidence {
    /// The reduced calendar date uses the year-only form.
    YearOnly {
        /// The representation carries a calendar year component.
        year: ReducedCalendarDateHasYearComponent,
        /// The representation uses the year-only lexical form.
        form: ReducedCalendarDateUsesYearOnlyRepresentation,
    },
    /// The reduced calendar date uses the year-month form.
    YearMonth {
        /// The representation carries a calendar year component.
        year: ReducedCalendarDateHasYearComponent,
        /// The representation uses the year-month lexical form.
        form: ReducedCalendarDateUsesYearMonthRepresentation,
        /// The month is within the legal ISO 8601 range.
        month: CalendarMonthInRangeOneToTwelve,
    },
}

impl core::default::Default for ReducedCalendarDatePrecisionEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::YearOnly {
            year: core::default::Default::default(),
            form: core::default::Default::default(),
        }
    }
}

/// Evidence branch for the declared reduced local-time precision.
///
/// Normative source: ISO 8601-1:2019, 5.3.1.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.2.2.3.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum ReducedLocalTimePrecisionEvidence {
    /// The reduced local time uses the hour-only form.
    HourOnly {
        /// The representation uses the hour-only lexical form.
        form: ReducedAccuracyLocalTimeUsesHourOnlyRepresentation,
    },
    /// The reduced local time uses the hour-minute form.
    HourMinute {
        /// The representation uses the hour-minute lexical form.
        form: ReducedAccuracyLocalTimeUsesHourMinuteRepresentation,
        /// The minute is within the legal ISO 8601 range.
        minute: MinuteInRangeZeroToFiftyNine,
    },
}

impl core::default::Default for ReducedLocalTimePrecisionEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::HourOnly {
            form: core::default::Default::default(),
        }
    }
}

/// Branch evidence for the specific Level 2 sub-year grouping family in use.
///
/// Normative source: ISO 8601-2:2019, 4.8.1, 4.8.2, and 4.8.3.
/// Informative cross-check: public LOC EDTF Level 2 — Sub-year groupings
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum SubYearGroupingKindEvidence {
    /// Seasonal grouping semantics, including hemisphere scope when declared.
    Season {
        /// The code declares a named season.
        named_season: SeasonCodeDeclaresNamedSeason,
        /// The code declares its season scope.
        season_scope: SeasonCodeDeclaresSeasonScope,
    },
    /// Quarter grouping semantics.
    Quarter {
        /// The code declares a quarter grouping.
        quarter: SubYearGroupingCodeDeclaresQuarter,
    },
    /// Quadrimester grouping semantics.
    Quadrimester {
        /// The code declares a quadrimester grouping.
        quadrimester: SubYearGroupingCodeDeclaresQuadrimester,
    },
    /// Semestral grouping semantics.
    Semestral {
        /// The code declares a semestral grouping.
        semestral: SubYearGroupingCodeDeclaresSemestral,
    },
}

impl core::default::Default for SubYearGroupingKindEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Season {
            named_season: core::default::Default::default(),
            season_scope: core::default::Default::default(),
        }
    }
}

/// Evidence branch for the declared numeric UTC-offset precision.
///
/// Normative source: ISO 8601-1:2019, 5.3.4.
/// Open-text cross-check: ISO/WD 8601-1:2016(E), 4.2.5.1.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum UtcOffsetPrecisionEvidence {
    /// The offset is expressed with hours only.
    HourOnly {
        /// Minute omission is legal only for integral-hour offsets.
        omitted_minutes: UtcDifferenceMinutesOmittedOnlyForIntegralHourOffsets,
    },
    /// The offset is expressed with hour-and-minute precision.
    HourMinute {
        /// The offset minute is in range.
        minute: UtcOffsetMinuteInRangeZeroToFiftyNine,
    },
}

impl core::default::Default for UtcOffsetPrecisionEvidence {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::HourOnly {
            omitted_minutes: core::default::Default::default(),
        }
    }
}

/// Evidence bundle for named-zone identity.
///
/// Normative sources: RFC 9557 §1.2, §3.3, and §4.1
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
pub struct ZonedTimestampEvidence {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// The timestamp carries a named-zone annotation.
    zone_annotation: NamedTimeZoneAnnotationPresent,
    /// The zone annotation uses an IANA zone identifier.
    zone_identifier: NamedTimeZoneUsesIanaIdentifier,
    /// The identifier excludes the forbidden `"."` and `".."` segments.
    zone_segments: NamedTimeZoneIdentifierExcludesDotSegments,
    /// The annotation is not merely a numeric offset alias.
    named_zone: NamedTimeZoneIsNotNumericOffsetAlias,
    /// The named zone preserves civil-time rule identity.
    civil_rules: NamedTimeZoneRetainsCivilRuleIdentity,
}
