//! Temporal-set/grouped-time-scale-unit/date-time-formula emission-proof composites.
//!
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{
    DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
};

/// Emission proof for [`FormattedTemporalSet`](crate::FormattedTemporalSet) — the `format_temporal_set` output proof(s), folded.
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
pub struct TemporalSetFormatted {
    /// The `temporal_set_expression_valid` sub-claim.
    temporal_set_expression_valid: TemporalSetExpressionValid,
    /// The `temporal_set_range_semantics_valid` sub-claim.
    temporal_set_range_semantics_valid: TemporalSetRangeSemanticsValid,
}
/// Emission proof for [`FormattedGroupedTimeScaleUnit`](crate::FormattedGroupedTimeScaleUnit) — the `format_grouped_time_scale_unit` output proof(s), folded.
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
pub struct GroupedTimeScaleUnitFormatted {
    /// The `grouped_time_scale_unit_valid` sub-claim.
    grouped_time_scale_unit_valid: GroupedTimeScaleUnitValid,
    /// The `grouped_time_scale_unit_uses_grouping_designators` sub-claim.
    grouped_time_scale_unit_uses_grouping_designators: GroupedTimeScaleUnitUsesGroupingDesignators,
    /// The `grouped_time_scale_unit_carries_one_or_more_duration_units` sub-claim.
    grouped_time_scale_unit_carries_one_or_more_duration_units:
        GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits,
    /// The `grouped_time_scale_unit_definition_is_continuous` sub-claim.
    grouped_time_scale_unit_definition_is_continuous: GroupedTimeScaleUnitDefinitionIsContinuous,
    /// The `grouped_time_scale_unit_value_carries_explicit_coefficient` sub-claim.
    grouped_time_scale_unit_value_carries_explicit_coefficient:
        GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    /// The `grouped_time_scale_unit_lower_order_units_remain_within_group_bounds` sub-claim.
    grouped_time_scale_unit_lower_order_units_remain_within_group_bounds:
        GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    /// The `grouped_time_scale_unit_date_time_may_carry_explicit_time_shift` sub-claim.
    grouped_time_scale_unit_date_time_may_carry_explicit_time_shift:
        GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    /// The `grouped_time_scale_unit_truncates_out_of_bounds_remainder` sub-claim.
    grouped_time_scale_unit_truncates_out_of_bounds_remainder:
        GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder,
    /// The `grouped_time_scale_unit_converts_to_time_interval` sub-claim.
    grouped_time_scale_unit_converts_to_time_interval: GroupedTimeScaleUnitConvertsToTimeInterval,
}
/// Emission proof for [`FormattedDateTimeFormula`](crate::FormattedDateTimeFormula) — the `format_date_time_formula` output proof(s), folded.
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
pub struct DateTimeFormulaFormatted {
    /// The `date_time_formula_valid` sub-claim.
    date_time_formula_valid: DateTimeFormulaValid,
    /// The `date_time_formula_evaluation_semantics_valid` sub-claim.
    date_time_formula_evaluation_semantics_valid: DateTimeFormulaEvaluationSemanticsValid,
}
