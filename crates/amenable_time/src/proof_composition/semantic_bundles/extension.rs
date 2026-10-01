//! CalConnect/ISO 8601-2 explicit-form extension family semantic bundles.
//!
//!
//! The `elicit_temporal` `*Bundle` aggregate proof bundles, ported as
//! folded `#[derive(Evidence, Witness)]` composites (the same
//! structural closure as the rest of `proof_composition`). Each
//! `Established<X>` field collapses to `X`; a `<Foo>Evidence` field
//! already merged into its `*Valid` sibling is dropped; `*ProofBranch`
//! and standalone `*Evidence` fields are kept. These pair with the
//! Phase 5 `ProvenTemporalCarrier` carrier wrapper.

use crate::{
    BackendConversionSemanticBundle, DateTimeFormulaEvaluationResultValid,
    DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid,
    ExplicitTemporalFormMayOmitZeroValuedComponents, ExplicitTemporalFormUsesDesignatorSymbols,
    ExplicitTemporalFormValid, ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitUtcRelationshipUsesZuluOrSignedShift,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
};

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
