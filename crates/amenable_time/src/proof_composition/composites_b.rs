//! Aggregate temporal proof propositions (E–L), ported from
//! `elicit_temporal::contracts::proof_composition`. Each aggregate is
//! folded with its `*Evidence` bundle: the aggregate's fields ARE its
//! sub-claims, and `#[derive(Witness)]` makes the composite proof the
//! structural product of its members' proofs. See [`super`] for the
//! design; leaf `Witness<V>` impls land in the backend crates.

use crate::{
    CombinedDateTimeDateComponentMustNotUseReducedAccuracy, CombinedDateTimeDateEvidence,
    CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
    CombinedDateTimeUsesTimeDesignator, ExplicitTemporalFormMayOmitZeroValuedComponents,
    ExplicitTemporalFormUsesDesignatorSymbols, ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitTimeIntervalUsesDateTimeEndpointFamily, ExplicitTimeOfDayForbidsEndOfDayRepresentation,
    ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators, ExplicitTimeOfDayUsesTimeDesignator,
    ExplicitTimeShiftBareZuluRepresentsUtcZero, ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
    ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc, ExplicitTimeShiftUsesZuluDesignator,
    ExplicitUtcRelationshipUsesZuluOrSignedShift, ExtendedYearBaseEvidence,
    ExtendedYearSignificantDigitsEvidence, FractionAppliesToLowestOrderComponent,
    FractionUsesDecimalSign, GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits,
    GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValueCarriesExplicitCoefficient, HourInRangeZeroToTwentyFour,
    IntervalDurationIsNonNegative, IntervalEndOmittedHigherOrderComponentsInheritFromStart,
    IntervalStartPrecedesEnd, IxdtfCalendarAnnotationPresent, IxdtfCalendarKeyUsesUCa,
    IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
    IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    IxdtfExperimentalSuffixKeysAreNotForInterchange,
    IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
    IxdtfExpertReviewReservesConciseGenerallyApplicableKeys, IxdtfGeneratorsMayOmitSuffixTags,
    IxdtfPermanentEntriesUseSpecificationRequiredPolicy,
    IxdtfProvisionalEntriesUseExpertReviewPolicy, IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    IxdtfRegisteredSuffixKeyCarriesChangeController, IxdtfRegisteredSuffixKeyCarriesDescription,
    IxdtfRegisteredSuffixKeyCarriesKeyIdentifier, IxdtfRegisteredSuffixKeyCarriesReference,
    IxdtfRegisteredSuffixKeyCarriesRegistrationStatus,
    IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent, IxdtfRegistryInitiallyContainsUCaEntry,
    IxdtfSuffixFollowsRfc3339Timestamp, IxdtfSuffixKeysAreLowercase,
    IxdtfSuffixTagsUseBracketedKeyValueForm,
    IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    IxdtfSuffixValuesUseHyphenDelimitedItems, IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    IxdtfUCaRegistryEntryIsPermanent, IxdtfUCaRegistryEntryReferencesSectionFive,
    IxdtfUCaRegistryEntryUsesIetfChangeController, LeapSecondOccursOnlyAtUtcBoundary,
    LocalDateTimeRequiresZoneOrOffsetForInstant, LocalTimeHasHourMinuteSecond,
    LocalTimeScaleMayBeStandardOrNonUtcBased, LocalTimeUsesLocallyApplicableTimeScale,
    MinuteInRangeZeroToFiftyNine, OpenIntervalBoundaryDeclared, Rfc3339TimestampValid,
    SecondInRangeZeroToSixty, StandardTimeValid, TimeIntervalValid,
    TwentyFourHourRequiresZeroMinuteSecondAndFraction, TwentyFourHourReservedForEndOfDay,
    UnknownIntervalBoundaryDeclared,
};

/// Aggregate proof that explicit-interval leading-shift propagation semantics are structurally valid.
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
pub struct ExplicitIntervalShiftPropagationSemanticsValid {
    /// The underlying explicit interval representation is structurally valid.
    interval: ExplicitTimeIntervalValid,
}

/// Aggregate proof that a CalConnect explicit form is structurally valid.
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
pub struct ExplicitTemporalFormValid {
    /// Explicit forms use designator symbols.
    designators: ExplicitTemporalFormUsesDesignatorSymbols,
    /// Zero-valued components may be omitted when the result remains valid.
    zero_omission: ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The lowest denoted component declares precision.
    precision: ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// UTC relationship syntax uses `Z` or a signed shift.
    utc_relationship: ExplicitUtcRelationshipUsesZuluOrSignedShift,
}

/// Aggregate proof that a CalConnect explicit time-interval form is structurally valid.
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
pub struct ExplicitTimeIntervalValid {
    /// The explicit interval still satisfies the shared two-component interval skeleton.
    interval: TimeIntervalValid,
    /// Both interval boundaries use the CalConnect `[datetimeE]` endpoint family.
    endpoints: ExplicitTimeIntervalUsesDateTimeEndpointFamily,
}

/// Aggregate proof that a CalConnect explicit local-time-of-day form is structurally valid.
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
pub struct ExplicitTimeOfDayValid {
    /// The representation uses the leading `T` designator.
    designator: ExplicitTimeOfDayUsesTimeDesignator,
    /// The representation uses explicit hour, minute, and second unit designators.
    units: ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators,
    /// The hour value is in range.
    hour: HourInRangeZeroToTwentyFour,
    /// The minute value is in range when present.
    minute: Option<MinuteInRangeZeroToFiftyNine>,
    /// The second value is in range when present.
    second: Option<SecondInRangeZeroToSixty>,
    /// Fraction syntax uses an ISO 8601 decimal sign when present.
    fraction_sign: Option<FractionUsesDecimalSign>,
    /// Any fraction attaches to the lowest-order present component.
    fraction_target: Option<FractionAppliesToLowestOrderComponent>,
    /// Zero-valued components may be omitted from the lexical form.
    zero_omission: ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The lowest denoted component declares the precision.
    precision: ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// Explicit local time of day does not use an end-of-day representation.
    no_end_of_day: ExplicitTimeOfDayForbidsEndOfDayRepresentation,
}

/// Aggregate proof that a CalConnect explicit time-shift form is structurally valid.
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
pub struct ExplicitTimeShiftValid {
    /// The representation uses the leading `Z` designator.
    designator: ExplicitTimeShiftUsesZuluDesignator,
    /// A leading minus sign appears only when the shift is behind UTC.
    sign: ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc,
    /// Any non-empty payload uses the explicit local-time-of-day family.
    payload_shape: ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
    /// A bare `Z` denotes UTC with zero shift.
    bare_utc: ExplicitTimeShiftBareZuluRepresentsUtcZero,
    /// The explicit time payload when present.
    time: Option<ExplicitTimeOfDayValid>,
}

/// Aggregate proof that extended interval boundary semantics are structurally valid.
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
pub struct ExtendedIntervalBoundarySemanticsValid {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// Open-boundary semantics are explicitly declared.
    open_boundary: OpenIntervalBoundaryDeclared,
    /// Unknown-boundary semantics are explicitly declared.
    unknown_boundary: UnknownIntervalBoundaryDeclared,
}

/// Aggregate proof that an ISO 8601-2 extended year form is structurally valid.
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
pub struct ExtendedYearValid {
    /// The base year form is structurally valid.
    base: ExtendedYearBaseEvidence,
    /// Optional significant-digit semantics attached to the base form.
    significant_digits: Option<ExtendedYearSignificantDigitsEvidence>,
}

/// Aggregate proof that a grouped time scale unit expression is structurally valid.
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
pub struct GroupedTimeScaleUnitValid {
    /// Grouped units use the `G...U` delimiters.
    designators: GroupedTimeScaleUnitUsesGroupingDesignators,
    /// Grouped units carry one or more duration components.
    units: GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits,
    /// Grouped-unit definitions are continuous.
    continuity: GroupedTimeScaleUnitDefinitionIsContinuous,
    /// Grouped-unit values carry explicit coefficients.
    coefficient: GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    /// Lower-order units remain within the grouped-unit bounds.
    bounds: GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    /// Grouped-unit date-time forms may append an explicit time shift.
    explicit_time_shift: GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    /// Out-of-bounds remainder truncates at the original boundary.
    truncation: GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder,
    /// Grouped-unit expressions convert into time-interval semantics.
    interval_semantics: GroupedTimeScaleUnitConvertsToTimeInterval,
}

/// Aggregate proof that inherited end-component semantics are structurally valid.
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
pub struct InheritedIntervalEndComponentsSemanticsValid {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// Omitted higher-order end components inherit from the leading component.
    inheritance: IntervalEndOmittedHigherOrderComponentsInheritFromStart,
}

/// Aggregate proof that inherited trailing-zone semantics are structurally valid.
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
pub struct InheritedIntervalZoneSemanticsValid {
    /// The underlying interval representation is structurally valid.
    interval: TimeIntervalValid,
}

/// Aggregate proof that interval endpoints are chronologically ordered.
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
pub struct IntervalEndpointsOrdered {
    /// The interval representation is structurally valid.
    interval: TimeIntervalValid,
    /// The start endpoint precedes the end endpoint.
    ordering: IntervalStartPrecedesEnd,
    /// Any duration component denotes a non-negative span.
    duration: IntervalDurationIsNonNegative,
}

/// Aggregate proof that IXDTF additional-information semantics are structurally valid.
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
pub struct IxdtfAdditionalInformationSemanticsValid {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// Suffix tags use the bracketed key-value form.
    tag_form: IxdtfSuffixTagsUseBracketedKeyValueForm,
    /// Suffix values use one or more hyphen-delimited items.
    value_items: IxdtfSuffixValuesUseHyphenDelimitedItems,
    /// Suffix keys remain lowercase.
    key_case: IxdtfSuffixKeysAreLowercase,
    /// Suffix values are case-sensitive unless a key says otherwise.
    value_case: IxdtfSuffixValuesAreCaseSensitiveUnlessOtherwiseSpecified,
    /// Generators may omit suffix tags entirely.
    optional_generation: IxdtfGeneratorsMayOmitSuffixTags,
    /// Elective suffix tags may be ignored by recipients.
    elective_consumption: IxdtfRecipientsMayIgnoreElectiveSuffixTags,
    /// Criticality is expressed with a leading `!` when present.
    critical_flag: IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    /// Critical suffix tags require processing or explicit error handling.
    critical_consumption: IxdtfCriticalSuffixTagsRequireProcessingOrErrorHandling,
    /// Experimental suffix keys use a leading underscore.
    experimental_key: IxdtfExperimentalSuffixKeysUseLeadingUnderscore,
    /// Experimental suffix keys are not valid for general interchange.
    experimental_interchange: IxdtfExperimentalSuffixKeysAreNotForInterchange,
}

/// Aggregate proof that the initial `u-ca` registry entry semantics are structurally valid.
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
pub struct IxdtfCalendarKeyRegistrySemanticsValid {
    /// The entry satisfies the generic Section 3.2 registry field set.
    entry: IxdtfSuffixKeyRegistryEntryValid,
    /// The initial registry contents include a `u-ca` entry.
    registry_entry: IxdtfRegistryInitiallyContainsUCaEntry,
    /// The entry is permanent.
    permanent: IxdtfUCaRegistryEntryIsPermanent,
    /// The entry uses `IETF` as change controller.
    change_controller: IxdtfUCaRegistryEntryUsesIetfChangeController,
    /// The entry references Section 5 of RFC 9557.
    reference: IxdtfUCaRegistryEntryReferencesSectionFive,
}

/// Aggregate proof that a permanent IXDTF suffix-key registration satisfies its reference semantics.
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
pub struct IxdtfPermanentSuffixKeyRegistrationSemanticsValid {
    /// The underlying entry carries the full Section 3.2 field set.
    entry: IxdtfSuffixKeyRegistryEntryValid,
    /// Permanent registrations use the permanent/provisional status domain law.
    status_domain: IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
}

/// Aggregate proof that a provisional IXDTF suffix-key registration satisfies its reference semantics.
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
pub struct IxdtfProvisionalSuffixKeyRegistrationSemanticsValid {
    /// The underlying entry carries the full Section 3.2 field set.
    entry: IxdtfSuffixKeyRegistryEntryValid,
    /// Provisional registrations use the permanent/provisional status domain law.
    status_domain: IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
}

/// Aggregate proof that an IXDTF suffix-key registry entry carries the required field set.
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
pub struct IxdtfSuffixKeyRegistryEntryValid {
    /// The entry carries a key identifier field.
    key_identifier: IxdtfRegisteredSuffixKeyCarriesKeyIdentifier,
    /// The entry carries a registration-status field.
    registration_status: IxdtfRegisteredSuffixKeyCarriesRegistrationStatus,
    /// The registration status is provisional or permanent.
    status_domain: IxdtfRegisteredSuffixKeyStatusIsProvisionalOrPermanent,
    /// The entry carries a description field.
    description: IxdtfRegisteredSuffixKeyCarriesDescription,
    /// The entry carries a change-controller field.
    change_controller: IxdtfRegisteredSuffixKeyCarriesChangeController,
    /// The entry carries a reference field.
    reference: IxdtfRegisteredSuffixKeyCarriesReference,
}

/// Aggregate proof that IXDTF registry-policy semantics are structurally valid.
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
pub struct IxdtfSuffixKeyRegistryPolicySemanticsValid {
    /// Permanent entries use the Specification Required registration policy.
    permanent_policy: IxdtfPermanentEntriesUseSpecificationRequiredPolicy,
    /// Provisional entries use the Expert Review registration policy.
    provisional_policy: IxdtfProvisionalEntriesUseExpertReviewPolicy,
    /// Experts reserve concise generally applicable identifiers for broad use.
    frugal_key_allocation: IxdtfExpertReviewReservesConciseGenerallyApplicableKeys,
}

/// Aggregate proof that an IXDTF timestamp declares a preferred presentation calendar.
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
pub struct IxdtfTimestampHasPreferredPresentationCalendar {
    /// The underlying timestamp is a valid IXDTF timestamp.
    timestamp: IxdtfTimestampValid,
    /// A calendar-awareness annotation is present.
    calendar_annotation: IxdtfCalendarAnnotationPresent,
    /// The calendar-awareness key uses the RFC 9557 `u-ca` token.
    calendar_key: IxdtfCalendarKeyUsesUCa,
    /// The calendar value uses a Unicode calendar identifier.
    calendar_identifier: IxdtfCalendarValueUsesUnicodeCalendarIdentifier,
}

/// Aggregate proof that an RFC 9557 IXDTF timestamp is structurally valid.
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
pub struct IxdtfTimestampValid {
    /// The base timestamp is a valid RFC 3339 timestamp.
    base: Rfc3339TimestampValid,
    /// Additional information is appended to an RFC 3339 timestamp.
    suffix: IxdtfSuffixFollowsRfc3339Timestamp,
    /// The time-zone annotation uses the RFC 9557 bracketed form.
    time_zone: IxdtfTimeZoneSuffixUsesBracketedNameOrOffset,
    /// Criticality is expressed with a leading `!` when present.
    critical: IxdtfCriticalFlagIsLeadingExclamationWhenPresent,
    /// Additional-information keys obey RFC 9557 casing rules.
    key_case: IxdtfSuffixKeysAreLowercase,
}

/// Aggregate proof that a local date-time does not, by itself, identify a fixed instant.
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
pub struct LocalDateTimeDoesNotIdentifyFixedInstant {
    /// The local date-time representation is structurally valid.
    local_date_time: LocalDateTimeValid,
    /// Additional zone or offset authority is required to identify an instant.
    requires_authority: LocalDateTimeRequiresZoneOrOffsetForInstant,
}

/// Aggregate proof that a combined local date-time representation is structurally valid.
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
pub struct LocalDateTimeValid {
    /// The date component is valid and belongs to a lawful combined date-time family.
    date: CombinedDateTimeDateEvidence,
    /// The time component is valid.
    time: LocalTimeValid,
    /// The date and time parts are joined with the ISO designator.
    separator: CombinedDateTimeUsesTimeDesignator,
    /// The date branch is complete rather than reduced-accuracy.
    date_precision: CombinedDateTimeDateComponentMustNotUseReducedAccuracy,
    /// The date and time branches use a single ISO format family across the expression.
    component_format: CombinedDateTimeUsesSingleFormatAcrossDateAndTimeComponents,
}

/// Aggregate proof that a local time-scale interpretation is established.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, amenable_derive::Evidence, amenable_derive::Witness,
)]
#[evidence(basis = "Self")]
pub enum LocalTimeScaleValid {
    /// The local time uses a UTC-derived standard-time scale.
    Standard {
        /// The standard-time interpretation is established explicitly.
        standard_time: StandardTimeValid,
        /// The standard permits local time scales to be UTC-derived standard time.
        locality: LocalTimeScaleMayBeStandardOrNonUtcBased,
    },
    /// The local time uses another non-UTC-based local time scale.
    NonUtcBased {
        /// The standard permits local time scales to be non-UTC-based.
        locality: LocalTimeScaleMayBeStandardOrNonUtcBased,
    },
}

impl core::default::Default for LocalTimeScaleValid {
    #[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
    fn default() -> Self {
        Self::Standard {
            standard_time: core::default::Default::default(),
            locality: core::default::Default::default(),
        }
    }
}

/// Aggregate proof that local-time semantics are established.
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
pub struct LocalTimeSemanticsValid {
    /// The wall-clock time-of-day representation is structurally valid.
    time: LocalTimeValid,
    /// The locally applicable time scale is established explicitly.
    scale: LocalTimeScaleValid,
    /// The time is interpreted against a locally applicable time scale.
    semantics: LocalTimeUsesLocallyApplicableTimeScale,
}

/// Aggregate proof that a local time representation is structurally valid.
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
pub struct LocalTimeValid {
    /// The representation carries hour, minute, and second fields.
    shape: LocalTimeHasHourMinuteSecond,
    /// The hour value is in range.
    hour: HourInRangeZeroToTwentyFour,
    /// The minute value is in range.
    minute: MinuteInRangeZeroToFiftyNine,
    /// The second value is in range.
    second: SecondInRangeZeroToSixty,
    /// End-of-day usage of hour 24 is legal.
    end_of_day: TwentyFourHourReservedForEndOfDay,
    /// End-of-day hour 24 uses terminal zero minute, second, and fraction values only.
    terminal_end_of_day: TwentyFourHourRequiresZeroMinuteSecondAndFraction,
    /// Leap-second usage is legal.
    leap_second: LeapSecondOccursOnlyAtUtcBoundary,
    /// Fraction syntax uses an ISO 8601 decimal sign.
    fraction_sign: FractionUsesDecimalSign,
    /// Any fraction attaches to the lowest-order present component.
    fraction_target: FractionAppliesToLowestOrderComponent,
}
