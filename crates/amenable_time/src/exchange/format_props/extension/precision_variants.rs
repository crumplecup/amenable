//! Qualified-value/seasonal/sub-year-grouping/unspecified-component emission-proof composites.
//!
//!
//! Each formatter method's output proof(s) folded into one
//! `#[derive(Evidence, Witness)]` struct (the same folding as
//! `proof_composition`), so the matching `Formatted*` output sidecar
//! keeps its single-`token` shape.

use crate::{
    LevelOneUnspecifiedDigitsOccupyRightmostPositions,
    LevelTwoUnspecifiedDigitsMayAppearWithinComponent, QualificationPlacementEvidence,
    QualifiedTemporalExpressionValid, QualifiedTemporalValueValid, SeasonCodeDeclaresNamedSeason,
    SeasonCodeDeclaresSeasonScope, SeasonalExpressionUsesSeasonCodeInMonthSlot,
    SeasonalExpressionUsesYearAndSeasonForm, SeasonalTemporalExpressionValid,
    SubYearGroupingExpressionUsesGroupingCodeInMonthSlot,
    SubYearGroupingExpressionUsesYearAndGroupingForm, SubYearGroupingExpressionValid,
    SubYearGroupingKindEvidence, UnspecifiedComponentExpressionValid,
    UnspecifiedDigitUsesUppercaseXPlaceholder, UnspecifiedDigitsDeclareUnknownValue,
};

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
