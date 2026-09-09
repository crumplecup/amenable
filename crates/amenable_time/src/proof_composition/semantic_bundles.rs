//! The `elicit_temporal` `*Bundle` aggregate proof bundles, ported as
//! folded `#[derive(Evidence, Witness)]` composites (the same
//! structural closure as the rest of `proof_composition`). Each
//! `Established<X>` field collapses to `X`; a `<Foo>Evidence` field
//! already merged into its `*Valid` sibling is dropped; `*ProofBranch`
//! and standalone `*Evidence` fields are kept. These pair with the
//! Phase 5 `ProvenTemporalCarrier` carrier wrapper.

use crate::{
    BackendConversionSemanticsValid, CompleteIntervalSubstitutionProofBranch, ConversionLossless,
    ConversionTruncatesSubseconds, DateTimeFormulaEvaluationResultValid,
    DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid, DurationFormValid,
    DurationRepresentationProofBranch, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit, ExplicitDurationRepresentationEvidence,
    ExplicitDurationSemanticEvidence, ExplicitDurationUsesDurationalUnitDesignators,
    ExplicitDurationValid, ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch, ExplicitTemporalFormMayOmitZeroValuedComponents,
    ExplicitTemporalFormUsesDesignatorSymbols, ExplicitTemporalFormValid,
    ExplicitTemporalPrecisionUsesLowestDenotedComponent, ExplicitTimeIntervalValid,
    ExplicitUtcRelationshipUsesZuluOrSignedShift, ExtendedIntervalBoundarySemanticsValid,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    IntervalEndComponentInheritanceProofBranch, IntervalEndpointsOrdered,
    IntervalZoneInheritanceProofBranch, LocalDateTimeDoesNotIdentifyFixedInstant,
    LocalDateTimeValid, LossyConversionAuthorityValid, NamedTimeZoneIdentityValid,
    NamedTimeZoneInterpretationTracksTzdbRevision, NamedZoneAttachmentEvidence,
    OffsetConsistentWithNamedZone, OffsetDateTimeValid, QualificationPlacementEvidence,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, RecurringIntervalFormValid,
    RecurringIntervalRepresentationProofBranch, TemporalSetExpressionValid,
    TemporalSetRangeSemanticsValid, TimeIntervalValid, TimestampRepresentsFixedInstant,
    ZoneTransitionResolutionAuthorityValid, ZonedDateTimeHasNamedZone,
};

/// Aggregate semantic bundle for one backend conversion exchange.
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
pub struct BackendConversionSemanticBundle {
    /// The `semantics` sub-claim.
    semantics: BackendConversionSemanticsValid,
}

/// Aggregate semantic bundle for one local date-time carrier.
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
pub struct LocalDateTimeSemanticBundle {
    /// The `validity` sub-claim.
    validity: LocalDateTimeValid,
    /// The `local_semantics` sub-claim.
    local_semantics: LocalDateTimeDoesNotIdentifyFixedInstant,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one fixed-instant offset date-time carrier.
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
pub struct OffsetDateTimeSemanticBundle {
    /// The `validity` sub-claim.
    validity: OffsetDateTimeValid,
    /// The `fixed_instant` sub-claim.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one named time-zone carrier.
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
pub struct NamedTimeZoneSemanticBundle {
    /// The `identity` sub-claim.
    identity: NamedTimeZoneIdentityValid,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one named-zone-attached fixed-instant carrier.
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
pub struct ZonedDateTimeSemanticBundle {
    /// The `fixed_instant` sub-claim.
    fixed_instant: TimestampRepresentsFixedInstant,
    /// The `zone_identity` sub-claim.
    zone_identity: NamedTimeZoneIdentityValid,
    /// The `zone_attachment` sub-claim.
    zone_attachment: ZonedDateTimeHasNamedZone,
    /// The `zone_attachment_evidence` sub-claim.
    zone_attachment_evidence: NamedZoneAttachmentEvidence,
    /// The `offset_consistency` sub-claim.
    offset_consistency: OffsetConsistentWithNamedZone,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one duration carrier.
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
pub struct DurationSemanticBundle {
    /// The `validity` sub-claim.
    validity: DurationFormValid,
    /// The `representation` sub-claim.
    representation: DurationRepresentationProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one time-interval carrier.
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
pub struct TimeIntervalSemanticBundle {
    /// The `validity` sub-claim.
    validity: TimeIntervalValid,
    /// The `extended_boundaries` sub-claim.
    extended_boundaries: ExtendedIntervalBoundarySemanticsValid,
    /// The `end_component_inheritance` sub-claim.
    end_component_inheritance: IntervalEndComponentInheritanceProofBranch,
    /// The `zone_inheritance` sub-claim.
    zone_inheritance: IntervalZoneInheritanceProofBranch,
    /// The `substitution` sub-claim.
    substitution: CompleteIntervalSubstitutionProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one recurring-interval carrier.
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
pub struct RecurringIntervalSemanticBundle {
    /// The `validity` sub-claim.
    validity: RecurringIntervalFormValid,
    /// The `representation` sub-claim.
    representation: RecurringIntervalRepresentationProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one qualified temporal value carrier.
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
pub struct QualifiedTemporalValueSemanticBundle {
    /// The `validity` sub-claim.
    validity: QualifiedTemporalValueValid,
    /// The `qualification` sub-claim.
    qualification: QualifiedTemporalExpressionValid,
    /// The `placement` sub-claim.
    placement: QualificationPlacementEvidence,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit temporal form carrier.
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
pub struct ExplicitTemporalFormSemanticBundle {
    /// The `validity` sub-claim.
    validity: ExplicitTemporalFormValid,
    /// The `designators` sub-claim.
    designators: ExplicitTemporalFormUsesDesignatorSymbols,
    /// The `zero_omission` sub-claim.
    zero_omission: ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The `precision` sub-claim.
    precision: ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// The `utc_relationship` sub-claim.
    utc_relationship: ExplicitUtcRelationshipUsesZuluOrSignedShift,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit duration carrier.
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
pub struct ExplicitDurationSemanticBundle {
    /// The `validity` sub-claim.
    validity: ExplicitDurationValid,
    /// The `units` sub-claim.
    units: ExplicitDurationUsesDurationalUnitDesignators,
    /// The `representation` sub-claim.
    representation: ExplicitDurationRepresentationEvidence,
    /// The `sign` sub-claim.
    sign: ExplicitDurationMayBeNegative,
    /// The `fractional` sub-claim.
    fractional: ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// The `semantics` sub-claim.
    semantics: ExplicitDurationSemanticEvidence,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one explicit time-interval carrier.
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
pub struct ExplicitTimeIntervalSemanticBundle {
    /// The `validity` sub-claim.
    validity: ExplicitTimeIntervalValid,
    /// The `duration_substitution` sub-claim.
    duration_substitution: ExplicitIntervalDurationSubstitutionProofBranch,
    /// The `end_component_inheritance` sub-claim.
    end_component_inheritance: ExplicitIntervalEndComponentInheritanceProofBranch,
    /// The `shift_propagation` sub-claim.
    shift_propagation: ExplicitIntervalShiftPropagationProofBranch,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one grouped time-scale-unit carrier.
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
pub struct GroupedTimeScaleUnitSemanticBundle {
    /// The `validity` sub-claim.
    validity: GroupedTimeScaleUnitValid,
    /// The `designators` sub-claim.
    designators: GroupedTimeScaleUnitUsesGroupingDesignators,
    /// The `units` sub-claim.
    units: GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits,
    /// The `continuity` sub-claim.
    continuity: GroupedTimeScaleUnitDefinitionIsContinuous,
    /// The `coefficient` sub-claim.
    coefficient: GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    /// The `bounds` sub-claim.
    bounds: GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    /// The `explicit_time_shift` sub-claim.
    explicit_time_shift: GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    /// The `truncation` sub-claim.
    truncation: GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder,
    /// The `interval_semantics` sub-claim.
    interval_semantics: GroupedTimeScaleUnitConvertsToTimeInterval,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one temporal set carrier.
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
pub struct TemporalSetSemanticBundle {
    /// The `expression` sub-claim.
    expression: TemporalSetExpressionValid,
    /// The `range_semantics` sub-claim.
    range_semantics: TemporalSetRangeSemanticsValid,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one date-time formula carrier.
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
pub struct DateTimeFormulaSemanticBundle {
    /// The `validity` sub-claim.
    validity: DateTimeFormulaValid,
    /// The `evaluation_semantics` sub-claim.
    evaluation_semantics: DateTimeFormulaEvaluationSemanticsValid,
    /// The `backend_conversion` sub-claim.
    backend_conversion: BackendConversionSemanticBundle,
}

/// Aggregate semantic bundle for one date-time-formula evaluation result.
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
pub struct DateTimeFormulaEvaluationResultBundle {
    /// The `semantics` sub-claim.
    semantics: DateTimeFormulaEvaluationResultValid,
}

/// Aggregate semantic bundle for explicit local-to-zone resolution authority.
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
pub struct ZoneTransitionResolutionAuthorityBundle {
    /// The `semantics` sub-claim.
    semantics: ZoneTransitionResolutionAuthorityValid,
}

/// Aggregate semantic bundle for explicit lossy-conversion authority.
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
pub struct LossyConversionAuthorityBundle {
    /// The `semantics` sub-claim.
    semantics: LossyConversionAuthorityValid,
}

/// Aggregate semantic bundle for one lossless conversion result.
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
pub struct LosslessConversionBundle {
    /// The `semantics` sub-claim.
    semantics: ConversionLossless,
}

/// Aggregate semantic bundle for one subsecond truncation result.
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
pub struct SubsecondTruncationBundle {
    /// The `semantics` sub-claim.
    semantics: ConversionTruncatesSubseconds,
}

/// Aggregate semantic bundle for one named-zone revision interpretation result.
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
pub struct NamedTimeZoneRevisionBundle {
    /// The `semantics` sub-claim.
    semantics: NamedTimeZoneInterpretationTracksTzdbRevision,
}

/// Aggregate semantic bundle for one ordered fixed-instant interval result.
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
pub struct IntervalEndpointOrderingBundle {
    /// The `semantics` sub-claim.
    semantics: IntervalEndpointsOrdered,
}
