//! Per-method composite proof propositions — for the parse methods
//! whose `elicit_temporal` return tuple carries 2+ proof sidecars, the
//! sidecars folded into one `#[derive(Evidence, Witness)]` struct (the
//! same folding as `proof_composition`) so the output `Sidecar` keeps
//! its single-`token` shape.

use crate::{
    CompleteIntervalSubstitutionProofBranch, DateTimeFormulaEvaluationSemanticsValid,
    DateTimeFormulaValid, ExtendedIntervalBoundarySemanticsValid,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValid, GroupedTimeScaleUnitValueCarriesExplicitCoefficient,
    IntervalEndComponentInheritanceProofBranch, IntervalZoneInheritanceProofBranch,
    IxdtfAdditionalInformationProofBranch, IxdtfCalendarAnnotationProofBranch,
    IxdtfTimeZoneAnnotationProofBranch, IxdtfTimestampValid,
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, LocalDateTimeDoesNotIdentifyFixedInstant,
    LocalDateTimeValid, OffsetDateTimeValid, QualificationPlacementEvidence,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, Rfc3339TimestampValid,
    SeasonCodeDeclaresNamedSeason, SeasonCodeDeclaresSeasonScope,
    SeasonalExpressionUsesSeasonCodeInMonthSlot, SeasonalExpressionUsesYearAndSeasonForm,
    SeasonalTemporalExpressionValid, SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubYearGroupingExpressionValid,
    SubYearGroupingKindEvidence, TemporalSetExpressionValid, TemporalSetRangeSemanticsValid,
    TimeIntervalValid, TimestampRepresentsFixedInstant, UnspecifiedComponentExpressionValid,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
};

/// Proof for [`ParsedQualifiedTemporalValue`](crate::ParsedQualifiedTemporalValue) — the 3 proof sidecars `elicit_temporal`'s `parse_qualified_temporal_value` returns, folded into one composite proposition.
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
pub struct QualifiedTemporalValueProof {
    /// The `qualified_temporal_value_valid` sub-claim.
    qualified_temporal_value_valid: QualifiedTemporalValueValid,
    /// The `qualified_temporal_expression_valid` sub-claim.
    qualified_temporal_expression_valid: QualifiedTemporalExpressionValid,
    /// The `qualification_placement_evidence` sub-claim.
    qualification_placement_evidence: QualificationPlacementEvidence,
}

/// Proof for [`ParsedLocalDateTime`](crate::ParsedLocalDateTime) — the 2 proof sidecars `elicit_temporal`'s `parse_local_date_time` returns, folded into one composite proposition.
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
pub struct LocalDateTimeProof {
    /// The `local_date_time_valid` sub-claim.
    local_date_time_valid: LocalDateTimeValid,
    /// The `local_date_time_does_not_identify_fixed_instant` sub-claim.
    local_date_time_does_not_identify_fixed_instant: LocalDateTimeDoesNotIdentifyFixedInstant,
}

/// Proof for [`ParsedOffsetDateTime`](crate::ParsedOffsetDateTime) — the 2 proof sidecars `elicit_temporal`'s `parse_offset_date_time` returns, folded into one composite proposition.
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
pub struct OffsetDateTimeProof {
    /// The `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
}

/// Proof for [`ParsedRfc3339Timestamp`](crate::ParsedRfc3339Timestamp) — the 3 proof sidecars `elicit_temporal`'s `parse_rfc3339_timestamp` returns, folded into one composite proposition.
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
pub struct Rfc3339TimestampProof {
    /// The `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The `rfc3339_timestamp_valid` sub-claim.
    rfc3339_timestamp_valid: Rfc3339TimestampValid,
    /// The `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
}

/// Proof for [`ParsedIxdtfTimestamp`](crate::ParsedIxdtfTimestamp) — the 5 proof sidecars `elicit_temporal`'s `parse_ixdtf_timestamp` returns, folded into one composite proposition.
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
pub struct IxdtfTimestampProof {
    /// The `ixdtf_timestamp_valid` sub-claim.
    ixdtf_timestamp_valid: IxdtfTimestampValid,
    /// The `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
    /// The `ixdtf_time_zone_annotation_proof_branch` sub-claim.
    ixdtf_time_zone_annotation_proof_branch: IxdtfTimeZoneAnnotationProofBranch,
    /// The `ixdtf_calendar_annotation_proof_branch` sub-claim.
    ixdtf_calendar_annotation_proof_branch: IxdtfCalendarAnnotationProofBranch,
    /// The `ixdtf_additional_information_proof_branch` sub-claim.
    ixdtf_additional_information_proof_branch: IxdtfAdditionalInformationProofBranch,
}

/// Proof for [`ParsedSeasonalTemporalExpression`](crate::ParsedSeasonalTemporalExpression) — the 5 proof sidecars `elicit_temporal`'s `parse_seasonal_temporal_expression` returns, folded into one composite proposition.
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
pub struct SeasonalTemporalExpressionProof {
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

/// Proof for [`ParsedSubYearGroupingExpression`](crate::ParsedSubYearGroupingExpression) — the 4 proof sidecars `elicit_temporal`'s `parse_sub_year_grouping_expression` returns, folded into one composite proposition.
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
pub struct SubYearGroupingExpressionProof {
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

/// Proof for [`ParsedUnspecifiedComponentExpression`](crate::ParsedUnspecifiedComponentExpression) — the 5 proof sidecars `elicit_temporal`'s `parse_unspecified_component_expression` returns, folded into one composite proposition.
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
pub struct UnspecifiedComponentExpressionProof {
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

/// Proof for [`ParsedTemporalSet`](crate::ParsedTemporalSet) — the 2 proof sidecars `elicit_temporal`'s `parse_temporal_set` returns, folded into one composite proposition.
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
pub struct TemporalSetProof {
    /// The `temporal_set_expression_valid` sub-claim.
    temporal_set_expression_valid: TemporalSetExpressionValid,
    /// The `temporal_set_range_semantics_valid` sub-claim.
    temporal_set_range_semantics_valid: TemporalSetRangeSemanticsValid,
}

/// Proof for [`ParsedGroupedTimeScaleUnit`](crate::ParsedGroupedTimeScaleUnit) — the 9 proof sidecars `elicit_temporal`'s `parse_grouped_time_scale_unit` returns, folded into one composite proposition.
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
pub struct GroupedTimeScaleUnitProof {
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

/// Proof for [`ParsedDateTimeFormula`](crate::ParsedDateTimeFormula) — the 2 proof sidecars `elicit_temporal`'s `parse_date_time_formula` returns, folded into one composite proposition.
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
pub struct DateTimeFormulaProof {
    /// The `date_time_formula_valid` sub-claim.
    date_time_formula_valid: DateTimeFormulaValid,
    /// The `date_time_formula_evaluation_semantics_valid` sub-claim.
    date_time_formula_evaluation_semantics_valid: DateTimeFormulaEvaluationSemanticsValid,
}

/// Proof for [`ParsedTimeInterval`](crate::ParsedTimeInterval) — the 5 proof sidecars `elicit_temporal`'s `parse_time_interval` returns, folded into one composite proposition.
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
pub struct TimeIntervalProof {
    /// The `time_interval_valid` sub-claim.
    time_interval_valid: TimeIntervalValid,
    /// The `extended_interval_boundary_semantics_valid` sub-claim.
    extended_interval_boundary_semantics_valid: ExtendedIntervalBoundarySemanticsValid,
    /// The `interval_end_component_inheritance_proof_branch` sub-claim.
    interval_end_component_inheritance_proof_branch: IntervalEndComponentInheritanceProofBranch,
    /// The `interval_zone_inheritance_proof_branch` sub-claim.
    interval_zone_inheritance_proof_branch: IntervalZoneInheritanceProofBranch,
    /// The `complete_interval_substitution_proof_branch` sub-claim.
    complete_interval_substitution_proof_branch: CompleteIntervalSubstitutionProofBranch,
}
