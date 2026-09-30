//! Emission-proof composites for the CalConnect/ISO 8601-2 extension family.
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{
    CenturyValid, DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid,
    DateWithShiftValid, DecadeValid, ExtendedYearValid,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, QualificationPlacementEvidence,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, SeasonCodeDeclaresNamedSeason,
    SeasonCodeDeclaresSeasonScope, SeasonalExpressionUsesSeasonCodeInMonthSlot,
    SeasonalExpressionUsesYearAndSeasonForm, SeasonalTemporalExpressionValid,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubYearGroupingExpressionValid,
    SubYearGroupingKindEvidence, TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
    TimeOfDayWithShiftValid, UnspecifiedComponentExpressionValid,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
};

/// Emission proof for [`FormattedDateWithShift`](crate::FormattedDateWithShift) — the `format_date_with_shift` output proof(s), folded.
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
pub struct DateWithShiftFormatted {
    /// The `date_with_shift_valid` sub-claim.
    date_with_shift_valid: DateWithShiftValid,
}

/// Emission proof for [`FormattedTimeOfDayWithShift`](crate::FormattedTimeOfDayWithShift) — the `format_time_of_day_with_shift` output proof(s), folded.
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
pub struct TimeOfDayWithShiftFormatted {
    /// The `time_of_day_with_shift_valid` sub-claim.
    time_of_day_with_shift_valid: TimeOfDayWithShiftValid,
}

/// Emission proof for [`FormattedExtendedYear`](crate::FormattedExtendedYear) — the `format_extended_year` output proof(s), folded.
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
pub struct ExtendedYearFormatted {
    /// The `extended_year_valid` sub-claim.
    extended_year_valid: ExtendedYearValid,
}

/// Emission proof for [`FormattedDecade`](crate::FormattedDecade) — the `format_decade` output proof(s), folded.
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
pub struct DecadeFormatted {
    /// The `decade_valid` sub-claim.
    decade_valid: DecadeValid,
}

/// Emission proof for [`FormattedCentury`](crate::FormattedCentury) — the `format_century` output proof(s), folded.
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
pub struct CenturyFormatted {
    /// The `century_valid` sub-claim.
    century_valid: CenturyValid,
}

/// Emission proof for [`FormattedQualifiedTemporalValue`](crate::FormattedQualifiedTemporalValue) — the `format_qualified_temporal_value` output proof(s), folded.
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
pub struct QualifiedTemporalValueFormatted {
    /// The `qualified_temporal_value_valid` sub-claim.
    qualified_temporal_value_valid: QualifiedTemporalValueValid,
    /// The `qualified_temporal_expression_valid` sub-claim.
    qualified_temporal_expression_valid: QualifiedTemporalExpressionValid,
    /// The `qualification_placement_evidence` sub-claim.
    qualification_placement_evidence: QualificationPlacementEvidence,
}

/// Emission proof for [`FormattedSeasonalTemporalExpression`](crate::FormattedSeasonalTemporalExpression) — the `format_seasonal_temporal_expression` output proof(s), folded.
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
pub struct SeasonalTemporalExpressionFormatted {
    /// The `seasonal_temporal_expression_valid` sub-claim.
    seasonal_temporal_expression_valid: SeasonalTemporalExpressionValid,
    /// The `seasonal_expression_uses_year_and_season_form` sub-claim.
    seasonal_expression_uses_year_and_season_form: SeasonalExpressionUsesYearAndSeasonForm,
    /// The `seasonal_expression_uses_season_code_in_month_slot` sub-claim.
    seasonal_expression_uses_season_code_in_month_slot: SeasonalExpressionUsesSeasonCodeInMonthSlot,
    /// The `season_code_declares_named_season` sub-claim.
    season_code_declares_named_season: SeasonCodeDeclaresNamedSeason,
    /// The `season_code_declares_season_scope` sub-claim.
    season_code_declares_season_scope: SeasonCodeDeclaresSeasonScope,
}

/// Emission proof for [`FormattedSubYearGroupingExpression`](crate::FormattedSubYearGroupingExpression) — the `format_sub_year_grouping_expression` output proof(s), folded.
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
pub struct SubYearGroupingExpressionFormatted {
    /// The `sub_year_grouping_expression_valid` sub-claim.
    sub_year_grouping_expression_valid: SubYearGroupingExpressionValid,
    /// The `sub_year_grouping_expression_uses_year_and_grouping_form` sub-claim.
    sub_year_grouping_expression_uses_year_and_grouping_form:
        SubYearGroupingExpressionUsesYearAndGroupingForm,
    /// The `sub_year_grouping_expression_uses_grouping_code_in_month_slot` sub-claim.
    sub_year_grouping_expression_uses_grouping_code_in_month_slot:
        SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    /// The `sub_year_grouping_kind_evidence` sub-claim.
    sub_year_grouping_kind_evidence: SubYearGroupingKindEvidence,
}

/// Emission proof for [`FormattedUnspecifiedComponentExpression`](crate::FormattedUnspecifiedComponentExpression) — the `format_unspecified_component_expression` output proof(s), folded.
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
pub struct UnspecifiedComponentExpressionFormatted {
    /// The `unspecified_component_expression_valid` sub-claim.
    unspecified_component_expression_valid: UnspecifiedComponentExpressionValid,
    /// The `unspecified_digit_uses_uppercase_x_placeholder` sub-claim.
    unspecified_digit_uses_uppercase_x_placeholder: UnspecifiedDigitUsesUppercaseXPlaceholder,
    /// The `unspecified_digits_declare_unknown_value` sub-claim.
    unspecified_digits_declare_unknown_value: UnspecifiedDigitsDeclareUnknownValue,
    /// The `level_one_unspecified_digits_occupy_rightmost_positions` sub-claim.
    level_one_unspecified_digits_occupy_rightmost_positions:
        LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    /// The `level_two_unspecified_digits_may_appear_within_component` sub-claim.
    level_two_unspecified_digits_may_appear_within_component:
        LevelTwoUnspecifiedDigitsMayAppearWithinComponent,
}

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
