//! `parse_selection_expression`/`format_selection_expression` sidecars (CalConnect extension family).

use crate::{
    FormattedTemporalText, SelectionExpressionDescriptor,
    SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionExpressionValid,
    SelectionRuleDayOfMonthUsesDayExpression, SelectionRuleHourUsesHourExpression,
    SelectionRuleMinuteUsesMinuteExpression, SelectionRuleMonthUsesMonthExpression,
    SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression, SelectionRulePositionAppliesLast,
    SelectionRulePositionUsesInstanceDesignatorSuffix, SelectionRuleSecondUsesSecondExpression,
    SelectionRuleWeekDayUsesDayOfWeekExpression, SelectionRuleWeekUsesWeekExpression,
    SelectionRulesApplyWithinSelectedResults, SelectionWithDurationUsesDurationSuffix,
};

/// Proof for [`ParsedSelectionExpression`](crate::ParsedSelectionExpression) — the 16 proof sidecars `parse_selection_expression` returns, folded.
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
pub struct SelectionExpressionProof {
    /// The `selection_expression_valid` sub-claim.
    selection_expression_valid: SelectionExpressionValid,
    /// The `selection_expression_uses_selection_delimiters` sub-claim.
    selection_expression_uses_selection_delimiters: SelectionExpressionUsesSelectionDelimiters,
    /// The `selection_expression_uses_recognized_selection_rule_vocabulary` sub-claim.
    selection_expression_uses_recognized_selection_rule_vocabulary:
        SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    /// The `selection_rule_month_uses_month_expression` sub-claim.
    selection_rule_month_uses_month_expression: SelectionRuleMonthUsesMonthExpression,
    /// The `selection_rule_week_uses_week_expression` sub-claim.
    selection_rule_week_uses_week_expression: SelectionRuleWeekUsesWeekExpression,
    /// The `selection_rule_day_of_month_uses_day_expression` sub-claim.
    selection_rule_day_of_month_uses_day_expression: SelectionRuleDayOfMonthUsesDayExpression,
    /// The `selection_rule_week_day_uses_day_of_week_expression` sub-claim.
    selection_rule_week_day_uses_day_of_week_expression:
        SelectionRuleWeekDayUsesDayOfWeekExpression,
    /// The `selection_rule_ordinal_day_of_year_uses_ordinal_day_expression` sub-claim.
    selection_rule_ordinal_day_of_year_uses_ordinal_day_expression:
        SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression,
    /// The `selection_rule_hour_uses_hour_expression` sub-claim.
    selection_rule_hour_uses_hour_expression: SelectionRuleHourUsesHourExpression,
    /// The `selection_rule_minute_uses_minute_expression` sub-claim.
    selection_rule_minute_uses_minute_expression: SelectionRuleMinuteUsesMinuteExpression,
    /// The `selection_rule_second_uses_second_expression` sub-claim.
    selection_rule_second_uses_second_expression: SelectionRuleSecondUsesSecondExpression,
    /// The `selection_rules_apply_within_selected_results` sub-claim.
    selection_rules_apply_within_selected_results: SelectionRulesApplyWithinSelectedResults,
    /// The `selection_expression_may_select_single_instance` sub-claim.
    selection_expression_may_select_single_instance: SelectionExpressionMaySelectSingleInstance,
    /// The `selection_rule_position_uses_instance_designator_suffix` sub-claim.
    selection_rule_position_uses_instance_designator_suffix:
        SelectionRulePositionUsesInstanceDesignatorSuffix,
    /// The `selection_rule_position_applies_last` sub-claim.
    selection_rule_position_applies_last: SelectionRulePositionAppliesLast,
    /// The `selection_with_duration_uses_duration_suffix` sub-claim.
    selection_with_duration_uses_duration_suffix: SelectionWithDurationUsesDurationSuffix,
}

/// Lawful token: the folded [`SelectionExpressionProof`](crate::SelectionExpressionProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SelectionExpressionProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::SelectionExpressionProof"
)]
pub struct SelectionExpressionProofToken(());

/// Output sidecar for the `parse_selection_expression` exchange: [`SelectionExpressionDescriptor`](crate::SelectionExpressionDescriptor) + a [`SelectionExpressionProof`](crate::SelectionExpressionProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::SelectionExpressionProof", constructor = "pub")]
pub struct ParsedSelectionExpression {
    #[sidecar(primary)]
    descriptor: SelectionExpressionDescriptor,
    #[sidecar(token)]
    token: SelectionExpressionProofToken,
}
/// Emission proof for [`FormattedSelectionExpression`](crate::FormattedSelectionExpression) — the `format_selection_expression` output proof(s), folded.
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
pub struct SelectionExpressionFormatted {
    /// The re-issued `selection_expression_valid` sub-claim.
    selection_expression_valid: SelectionExpressionValid,
    /// The re-issued `selection_expression_uses_selection_delimiters` sub-claim.
    selection_expression_uses_selection_delimiters: SelectionExpressionUsesSelectionDelimiters,
    /// The re-issued `selection_expression_uses_recognized_selection_rule_vocabulary` sub-claim.
    selection_expression_uses_recognized_selection_rule_vocabulary:
        SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    /// The re-issued `selection_rule_month_uses_month_expression` sub-claim.
    selection_rule_month_uses_month_expression: SelectionRuleMonthUsesMonthExpression,
    /// The re-issued `selection_rule_week_uses_week_expression` sub-claim.
    selection_rule_week_uses_week_expression: SelectionRuleWeekUsesWeekExpression,
    /// The re-issued `selection_rule_day_of_month_uses_day_expression` sub-claim.
    selection_rule_day_of_month_uses_day_expression: SelectionRuleDayOfMonthUsesDayExpression,
    /// The re-issued `selection_rule_week_day_uses_day_of_week_expression` sub-claim.
    selection_rule_week_day_uses_day_of_week_expression:
        SelectionRuleWeekDayUsesDayOfWeekExpression,
    /// The re-issued `selection_rule_ordinal_day_of_year_uses_ordinal_day_expression` sub-claim.
    selection_rule_ordinal_day_of_year_uses_ordinal_day_expression:
        SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression,
    /// The re-issued `selection_rule_hour_uses_hour_expression` sub-claim.
    selection_rule_hour_uses_hour_expression: SelectionRuleHourUsesHourExpression,
    /// The re-issued `selection_rule_minute_uses_minute_expression` sub-claim.
    selection_rule_minute_uses_minute_expression: SelectionRuleMinuteUsesMinuteExpression,
    /// The re-issued `selection_rule_second_uses_second_expression` sub-claim.
    selection_rule_second_uses_second_expression: SelectionRuleSecondUsesSecondExpression,
    /// The re-issued `selection_rules_apply_within_selected_results` sub-claim.
    selection_rules_apply_within_selected_results: SelectionRulesApplyWithinSelectedResults,
    /// The re-issued `selection_expression_may_select_single_instance` sub-claim.
    selection_expression_may_select_single_instance: SelectionExpressionMaySelectSingleInstance,
    /// The re-issued `selection_rule_position_uses_instance_designator_suffix` sub-claim.
    selection_rule_position_uses_instance_designator_suffix:
        SelectionRulePositionUsesInstanceDesignatorSuffix,
    /// The re-issued `selection_rule_position_applies_last` sub-claim.
    selection_rule_position_applies_last: SelectionRulePositionAppliesLast,
    /// The re-issued `selection_with_duration_uses_duration_suffix` sub-claim.
    selection_with_duration_uses_duration_suffix: SelectionWithDurationUsesDurationSuffix,
}

/// Lawful token: [`SelectionExpressionFormatted`](crate::SelectionExpressionFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SelectionExpressionFormatted")]
#[amenable_derive::establish(
    credential = "crate::SelectionExpressionProofToken",
    proposition = "crate::SelectionExpressionFormatted"
)]
pub struct SelectionExpressionFormattedToken(());

/// Output sidecar for the `format_selection_expression` exchange: the emitted text + a [`SelectionExpressionFormatted`](crate::SelectionExpressionFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::SelectionExpressionFormatted",
    constructor = "pub"
)]
pub struct FormattedSelectionExpression {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: SelectionExpressionFormattedToken,
}

impl FormattedSelectionExpression {
    /// Borrow the emitted text.
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
