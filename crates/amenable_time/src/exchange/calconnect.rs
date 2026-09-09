//! `TemporalCalConnectFactory` exchange surface (Phase 4 Step 5).
//! Self-contained: the 6 CalConnect-only parse methods + their 6
//! emit counterparts + `evaluate_date_time_formula`, each folded the
//! `proof_composition` way (`*Proof` for a parse, `*Formatted` for an
//! emit, `*Preconditions`/`*Established` for the transition). The 6
//! shared families (qualified value, grouped unit, date-time formula)
//! reuse the parser / formatter sidecars. `Exchange` impls: backend.

use crate::{
    DateTimeFormulaDescriptor, DateTimeFormulaEvaluationResultValid,
    DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid, ExplicitDurationDescriptor,
    ExplicitDurationMayBeNegative, ExplicitDurationMayUseFractionalLowestOrderUnit,
    ExplicitDurationRepresentationEvidence, ExplicitDurationSemanticEvidence,
    ExplicitDurationUsesDurationalUnitDesignators, ExplicitDurationValid,
    ExplicitIntervalDurationSubstitutionProofBranch,
    ExplicitIntervalEndComponentInheritanceProofBranch,
    ExplicitIntervalShiftPropagationProofBranch, ExplicitTemporalFormDescriptor,
    ExplicitTemporalFormMayOmitZeroValuedComponents, ExplicitTemporalFormUsesDesignatorSymbols,
    ExplicitTemporalFormValid, ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitTemporalValueDescriptor, ExplicitTimeIntervalDescriptor, ExplicitTimeIntervalValid,
    ExplicitUtcRelationshipUsesZuluOrSignedShift, FormattedTemporalText,
    RecurringIntervalWithRepeatRuleDescriptor, RecurringIntervalWithRepeatRuleIntervalProofBranch,
    RecurringIntervalWithRepeatRuleValid, RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleDescriptor, RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    RepeatRuleValid, SelectionExpressionDescriptor, SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionExpressionValid,
    SelectionRuleDayOfMonthUsesDayExpression, SelectionRuleHourUsesHourExpression,
    SelectionRuleMinuteUsesMinuteExpression, SelectionRuleMonthUsesMonthExpression,
    SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression, SelectionRulePositionAppliesLast,
    SelectionRulePositionUsesInstanceDesignatorSuffix, SelectionRuleSecondUsesSecondExpression,
    SelectionRuleWeekDayUsesDayOfWeekExpression, SelectionRuleWeekUsesWeekExpression,
    SelectionRulesApplyWithinSelectedResults, SelectionWithDurationUsesDurationSuffix,
};

/// Proof for [`ParsedExplicitTemporalForm`](crate::ParsedExplicitTemporalForm) — the 5 proof sidecars `parse_explicit_temporal_form` returns, folded.
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
pub struct ExplicitTemporalFormProof {
    /// The `explicit_temporal_form_valid` sub-claim.
    explicit_temporal_form_valid: ExplicitTemporalFormValid,
    /// The `explicit_temporal_form_uses_designator_symbols` sub-claim.
    explicit_temporal_form_uses_designator_symbols: ExplicitTemporalFormUsesDesignatorSymbols,
    /// The `explicit_temporal_form_may_omit_zero_valued_components` sub-claim.
    explicit_temporal_form_may_omit_zero_valued_components:
        ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The `explicit_temporal_precision_uses_lowest_denoted_component` sub-claim.
    explicit_temporal_precision_uses_lowest_denoted_component:
        ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// The `explicit_utc_relationship_uses_zulu_or_signed_shift` sub-claim.
    explicit_utc_relationship_uses_zulu_or_signed_shift:
        ExplicitUtcRelationshipUsesZuluOrSignedShift,
}

/// Lawful token: the folded [`ExplicitTemporalFormProof`](crate::ExplicitTemporalFormProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTemporalFormProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExplicitTemporalFormProof"
)]
pub struct ExplicitTemporalFormProofToken(());

/// Output sidecar for the `parse_explicit_temporal_form` exchange: [`ExplicitTemporalFormDescriptor`](crate::ExplicitTemporalFormDescriptor) + a [`ExplicitTemporalFormProof`](crate::ExplicitTemporalFormProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExplicitTemporalFormProof", constructor = "pub")]
pub struct ParsedExplicitTemporalForm {
    #[sidecar(primary)]
    descriptor: ExplicitTemporalFormDescriptor,
    #[sidecar(token)]
    token: ExplicitTemporalFormProofToken,
}

impl ParsedExplicitTemporalForm {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExplicitTemporalFormDescriptor {
        &self.descriptor
    }
}

/// Proof for [`ParsedExplicitDuration`](crate::ParsedExplicitDuration) — the 6 proof sidecars `parse_explicit_duration` returns, folded.
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
pub struct ExplicitDurationProof {
    /// The `explicit_duration_valid` sub-claim.
    explicit_duration_valid: ExplicitDurationValid,
    /// The `explicit_duration_uses_durational_unit_designators` sub-claim.
    explicit_duration_uses_durational_unit_designators:
        ExplicitDurationUsesDurationalUnitDesignators,
    /// The `explicit_duration_representation_evidence` sub-claim.
    explicit_duration_representation_evidence: ExplicitDurationRepresentationEvidence,
    /// The `explicit_duration_may_be_negative` sub-claim.
    explicit_duration_may_be_negative: ExplicitDurationMayBeNegative,
    /// The `explicit_duration_may_use_fractional_lowest_order_unit` sub-claim.
    explicit_duration_may_use_fractional_lowest_order_unit:
        ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// The `explicit_duration_semantic_evidence` sub-claim.
    explicit_duration_semantic_evidence: ExplicitDurationSemanticEvidence,
}

/// Lawful token: the folded [`ExplicitDurationProof`](crate::ExplicitDurationProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitDurationProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExplicitDurationProof"
)]
pub struct ExplicitDurationProofToken(());

/// Output sidecar for the `parse_explicit_duration` exchange: [`ExplicitDurationDescriptor`](crate::ExplicitDurationDescriptor) + a [`ExplicitDurationProof`](crate::ExplicitDurationProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExplicitDurationProof", constructor = "pub")]
pub struct ParsedExplicitDuration {
    #[sidecar(primary)]
    descriptor: ExplicitDurationDescriptor,
    #[sidecar(token)]
    token: ExplicitDurationProofToken,
}

impl ParsedExplicitDuration {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExplicitDurationDescriptor {
        &self.descriptor
    }
}

/// Proof for [`ParsedExplicitTimeInterval`](crate::ParsedExplicitTimeInterval) — the 4 proof sidecars `parse_explicit_time_interval` returns, folded.
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
pub struct ExplicitTimeIntervalProof {
    /// The `explicit_time_interval_valid` sub-claim.
    explicit_time_interval_valid: ExplicitTimeIntervalValid,
    /// The `explicit_interval_duration_substitution_proof_branch` sub-claim.
    explicit_interval_duration_substitution_proof_branch:
        ExplicitIntervalDurationSubstitutionProofBranch,
    /// The `explicit_interval_end_component_inheritance_proof_branch` sub-claim.
    explicit_interval_end_component_inheritance_proof_branch:
        ExplicitIntervalEndComponentInheritanceProofBranch,
    /// The `explicit_interval_shift_propagation_proof_branch` sub-claim.
    explicit_interval_shift_propagation_proof_branch: ExplicitIntervalShiftPropagationProofBranch,
}

/// Lawful token: the folded [`ExplicitTimeIntervalProof`](crate::ExplicitTimeIntervalProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTimeIntervalProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ExplicitTimeIntervalProof"
)]
pub struct ExplicitTimeIntervalProofToken(());

/// Output sidecar for the `parse_explicit_time_interval` exchange: [`ExplicitTimeIntervalDescriptor`](crate::ExplicitTimeIntervalDescriptor) + a [`ExplicitTimeIntervalProof`](crate::ExplicitTimeIntervalProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExplicitTimeIntervalProof", constructor = "pub")]
pub struct ParsedExplicitTimeInterval {
    #[sidecar(primary)]
    descriptor: ExplicitTimeIntervalDescriptor,
    #[sidecar(token)]
    token: ExplicitTimeIntervalProofToken,
}

impl ParsedExplicitTimeInterval {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExplicitTimeIntervalDescriptor {
        &self.descriptor
    }
}

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
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::SelectionExpressionProof", constructor = "pub")]
pub struct ParsedSelectionExpression {
    #[sidecar(primary)]
    descriptor: SelectionExpressionDescriptor,
    #[sidecar(token)]
    token: SelectionExpressionProofToken,
}

impl ParsedSelectionExpression {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &SelectionExpressionDescriptor {
        &self.descriptor
    }
}

/// Proof for [`ParsedRepeatRule`](crate::ParsedRepeatRule) — the 5 proof sidecars `parse_repeat_rule` returns, folded.
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
pub struct RepeatRuleProof {
    /// The `repeat_rule_valid` sub-claim.
    repeat_rule_valid: RepeatRuleValid,
    /// The `repeat_rule_uses_frequency_designator` sub-claim.
    repeat_rule_uses_frequency_designator: RepeatRuleUsesFrequencyDesignator,
    /// The `repeat_rule_declares_eligible_time_intervals` sub-claim.
    repeat_rule_declares_eligible_time_intervals: RepeatRuleDeclaresEligibleTimeIntervals,
    /// The `repeat_rule_selection_applies_within_eligible_intervals` sub-claim.
    repeat_rule_selection_applies_within_eligible_intervals:
        RepeatRuleSelectionAppliesWithinEligibleIntervals,
    /// The `repeat_rule_evaluation_inherits_initial_start_component_information` sub-claim.
    repeat_rule_evaluation_inherits_initial_start_component_information:
        RepeatRuleEvaluationInheritsInitialStartComponentInformation,
}

/// Lawful token: the folded [`RepeatRuleProof`](crate::RepeatRuleProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RepeatRuleProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::RepeatRuleProof"
)]
pub struct RepeatRuleProofToken(());

/// Output sidecar for the `parse_repeat_rule` exchange: [`RepeatRuleDescriptor`](crate::RepeatRuleDescriptor) + a [`RepeatRuleProof`](crate::RepeatRuleProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::RepeatRuleProof", constructor = "pub")]
pub struct ParsedRepeatRule {
    #[sidecar(primary)]
    descriptor: RepeatRuleDescriptor,
    #[sidecar(token)]
    token: RepeatRuleProofToken,
}

impl ParsedRepeatRule {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &RepeatRuleDescriptor {
        &self.descriptor
    }
}

/// Proof for [`ParsedRecurringIntervalWithRepeatRule`](crate::ParsedRecurringIntervalWithRepeatRule) — the 6 proof sidecars `parse_recurring_interval_with_repeat_rule` returns, folded.
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
pub struct RecurringIntervalWithRepeatRuleProof {
    /// The `recurring_interval_with_repeat_rule_valid` sub-claim.
    recurring_interval_with_repeat_rule_valid: RecurringIntervalWithRepeatRuleValid,
    /// The `recurring_interval_with_repeat_rule_interval_proof_branch` sub-claim.
    recurring_interval_with_repeat_rule_interval_proof_branch:
        RecurringIntervalWithRepeatRuleIntervalProofBranch,
    /// The `repeat_rule_uses_frequency_designator` sub-claim.
    repeat_rule_uses_frequency_designator: RepeatRuleUsesFrequencyDesignator,
    /// The `repeat_rule_declares_eligible_time_intervals` sub-claim.
    repeat_rule_declares_eligible_time_intervals: RepeatRuleDeclaresEligibleTimeIntervals,
    /// The `repeat_rule_selection_applies_within_eligible_intervals` sub-claim.
    repeat_rule_selection_applies_within_eligible_intervals:
        RepeatRuleSelectionAppliesWithinEligibleIntervals,
    /// The `repeat_rule_evaluation_inherits_initial_start_component_information` sub-claim.
    repeat_rule_evaluation_inherits_initial_start_component_information:
        RepeatRuleEvaluationInheritsInitialStartComponentInformation,
}

/// Lawful token: the folded [`RecurringIntervalWithRepeatRuleProof`](crate::RecurringIntervalWithRepeatRuleProof) was established from a received temporal input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RecurringIntervalWithRepeatRuleProof")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::RecurringIntervalWithRepeatRuleProof"
)]
pub struct RecurringIntervalWithRepeatRuleProofToken(());

/// Output sidecar for the `parse_recurring_interval_with_repeat_rule` exchange: [`RecurringIntervalWithRepeatRuleDescriptor`](crate::RecurringIntervalWithRepeatRuleDescriptor) + a [`RecurringIntervalWithRepeatRuleProof`](crate::RecurringIntervalWithRepeatRuleProof) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::RecurringIntervalWithRepeatRuleProof",
    constructor = "pub"
)]
pub struct ParsedRecurringIntervalWithRepeatRule {
    #[sidecar(primary)]
    descriptor: RecurringIntervalWithRepeatRuleDescriptor,
    #[sidecar(token)]
    token: RecurringIntervalWithRepeatRuleProofToken,
}

impl ParsedRecurringIntervalWithRepeatRule {
    /// Borrow the parsed descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &RecurringIntervalWithRepeatRuleDescriptor {
        &self.descriptor
    }
}

/// Emission proof for [`FormattedExplicitTemporalForm`](crate::FormattedExplicitTemporalForm) — the `format_explicit_temporal_form` output proof(s), folded.
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
pub struct ExplicitTemporalFormFormatted {
    /// The re-issued `explicit_temporal_form_valid` sub-claim.
    explicit_temporal_form_valid: ExplicitTemporalFormValid,
    /// The re-issued `explicit_temporal_form_uses_designator_symbols` sub-claim.
    explicit_temporal_form_uses_designator_symbols: ExplicitTemporalFormUsesDesignatorSymbols,
    /// The re-issued `explicit_temporal_form_may_omit_zero_valued_components` sub-claim.
    explicit_temporal_form_may_omit_zero_valued_components:
        ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The re-issued `explicit_temporal_precision_uses_lowest_denoted_component` sub-claim.
    explicit_temporal_precision_uses_lowest_denoted_component:
        ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// The re-issued `explicit_utc_relationship_uses_zulu_or_signed_shift` sub-claim.
    explicit_utc_relationship_uses_zulu_or_signed_shift:
        ExplicitUtcRelationshipUsesZuluOrSignedShift,
}

/// Lawful token: [`ExplicitTemporalFormFormatted`](crate::ExplicitTemporalFormFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTemporalFormFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExplicitTemporalFormProofToken",
    proposition = "crate::ExplicitTemporalFormFormatted"
)]
pub struct ExplicitTemporalFormFormattedToken(());

/// Output sidecar for the `format_explicit_temporal_form` exchange: the emitted text + a [`ExplicitTemporalFormFormatted`](crate::ExplicitTemporalFormFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ExplicitTemporalFormFormatted",
    constructor = "pub"
)]
pub struct FormattedExplicitTemporalForm {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: ExplicitTemporalFormFormattedToken,
}

impl FormattedExplicitTemporalForm {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Emission proof for [`FormattedExplicitDuration`](crate::FormattedExplicitDuration) — the `format_explicit_duration` output proof(s), folded.
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
pub struct ExplicitDurationFormatted {
    /// The re-issued `explicit_duration_valid` sub-claim.
    explicit_duration_valid: ExplicitDurationValid,
    /// The re-issued `explicit_duration_uses_durational_unit_designators` sub-claim.
    explicit_duration_uses_durational_unit_designators:
        ExplicitDurationUsesDurationalUnitDesignators,
    /// The re-issued `explicit_duration_representation_evidence` sub-claim.
    explicit_duration_representation_evidence: ExplicitDurationRepresentationEvidence,
    /// The re-issued `explicit_duration_may_be_negative` sub-claim.
    explicit_duration_may_be_negative: ExplicitDurationMayBeNegative,
    /// The re-issued `explicit_duration_may_use_fractional_lowest_order_unit` sub-claim.
    explicit_duration_may_use_fractional_lowest_order_unit:
        ExplicitDurationMayUseFractionalLowestOrderUnit,
    /// The re-issued `explicit_duration_semantic_evidence` sub-claim.
    explicit_duration_semantic_evidence: ExplicitDurationSemanticEvidence,
}

/// Lawful token: [`ExplicitDurationFormatted`](crate::ExplicitDurationFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitDurationFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExplicitDurationProofToken",
    proposition = "crate::ExplicitDurationFormatted"
)]
pub struct ExplicitDurationFormattedToken(());

/// Output sidecar for the `format_explicit_duration` exchange: the emitted text + a [`ExplicitDurationFormatted`](crate::ExplicitDurationFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::ExplicitDurationFormatted", constructor = "pub")]
pub struct FormattedExplicitDuration {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: ExplicitDurationFormattedToken,
}

impl FormattedExplicitDuration {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Emission proof for [`FormattedExplicitTimeInterval`](crate::FormattedExplicitTimeInterval) — the `format_explicit_time_interval` output proof(s), folded.
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
pub struct ExplicitTimeIntervalFormatted {
    /// The re-issued `explicit_time_interval_valid` sub-claim.
    explicit_time_interval_valid: ExplicitTimeIntervalValid,
    /// The re-issued `explicit_interval_duration_substitution_proof_branch` sub-claim.
    explicit_interval_duration_substitution_proof_branch:
        ExplicitIntervalDurationSubstitutionProofBranch,
    /// The re-issued `explicit_interval_end_component_inheritance_proof_branch` sub-claim.
    explicit_interval_end_component_inheritance_proof_branch:
        ExplicitIntervalEndComponentInheritanceProofBranch,
    /// The re-issued `explicit_interval_shift_propagation_proof_branch` sub-claim.
    explicit_interval_shift_propagation_proof_branch: ExplicitIntervalShiftPropagationProofBranch,
}

/// Lawful token: [`ExplicitTimeIntervalFormatted`](crate::ExplicitTimeIntervalFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTimeIntervalFormatted")]
#[amenable_derive::establish(
    credential = "crate::ExplicitTimeIntervalProofToken",
    proposition = "crate::ExplicitTimeIntervalFormatted"
)]
pub struct ExplicitTimeIntervalFormattedToken(());

/// Output sidecar for the `format_explicit_time_interval` exchange: the emitted text + a [`ExplicitTimeIntervalFormatted`](crate::ExplicitTimeIntervalFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ExplicitTimeIntervalFormatted",
    constructor = "pub"
)]
pub struct FormattedExplicitTimeInterval {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: ExplicitTimeIntervalFormattedToken,
}

impl FormattedExplicitTimeInterval {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
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
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Emission proof for [`FormattedRepeatRule`](crate::FormattedRepeatRule) — the `format_repeat_rule` output proof(s), folded.
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
pub struct RepeatRuleFormatted {
    /// The re-issued `repeat_rule_valid` sub-claim.
    repeat_rule_valid: RepeatRuleValid,
    /// The re-issued `repeat_rule_uses_frequency_designator` sub-claim.
    repeat_rule_uses_frequency_designator: RepeatRuleUsesFrequencyDesignator,
    /// The re-issued `repeat_rule_declares_eligible_time_intervals` sub-claim.
    repeat_rule_declares_eligible_time_intervals: RepeatRuleDeclaresEligibleTimeIntervals,
    /// The re-issued `repeat_rule_selection_applies_within_eligible_intervals` sub-claim.
    repeat_rule_selection_applies_within_eligible_intervals:
        RepeatRuleSelectionAppliesWithinEligibleIntervals,
    /// The re-issued `repeat_rule_evaluation_inherits_initial_start_component_information` sub-claim.
    repeat_rule_evaluation_inherits_initial_start_component_information:
        RepeatRuleEvaluationInheritsInitialStartComponentInformation,
}

/// Lawful token: [`RepeatRuleFormatted`](crate::RepeatRuleFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RepeatRuleFormatted")]
#[amenable_derive::establish(
    credential = "crate::RepeatRuleProofToken",
    proposition = "crate::RepeatRuleFormatted"
)]
pub struct RepeatRuleFormattedToken(());

/// Output sidecar for the `format_repeat_rule` exchange: the emitted text + a [`RepeatRuleFormatted`](crate::RepeatRuleFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::RepeatRuleFormatted", constructor = "pub")]
pub struct FormattedRepeatRule {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: RepeatRuleFormattedToken,
}

impl FormattedRepeatRule {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Emission proof for [`FormattedRecurringIntervalWithRepeatRule`](crate::FormattedRecurringIntervalWithRepeatRule) — the `format_recurring_interval_with_repeat_rule` output proof(s), folded.
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
pub struct RecurringIntervalWithRepeatRuleFormatted {
    /// The re-issued `recurring_interval_with_repeat_rule_valid` sub-claim.
    recurring_interval_with_repeat_rule_valid: RecurringIntervalWithRepeatRuleValid,
    /// The re-issued `recurring_interval_with_repeat_rule_interval_proof_branch` sub-claim.
    recurring_interval_with_repeat_rule_interval_proof_branch:
        RecurringIntervalWithRepeatRuleIntervalProofBranch,
    /// The re-issued `repeat_rule_uses_frequency_designator` sub-claim.
    repeat_rule_uses_frequency_designator: RepeatRuleUsesFrequencyDesignator,
    /// The re-issued `repeat_rule_declares_eligible_time_intervals` sub-claim.
    repeat_rule_declares_eligible_time_intervals: RepeatRuleDeclaresEligibleTimeIntervals,
    /// The re-issued `repeat_rule_selection_applies_within_eligible_intervals` sub-claim.
    repeat_rule_selection_applies_within_eligible_intervals:
        RepeatRuleSelectionAppliesWithinEligibleIntervals,
    /// The re-issued `repeat_rule_evaluation_inherits_initial_start_component_information` sub-claim.
    repeat_rule_evaluation_inherits_initial_start_component_information:
        RepeatRuleEvaluationInheritsInitialStartComponentInformation,
}

/// Lawful token: [`RecurringIntervalWithRepeatRuleFormatted`](crate::RecurringIntervalWithRepeatRuleFormatted) was established by emitting a proven descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RecurringIntervalWithRepeatRuleFormatted")]
#[amenable_derive::establish(
    credential = "crate::RecurringIntervalWithRepeatRuleProofToken",
    proposition = "crate::RecurringIntervalWithRepeatRuleFormatted"
)]
pub struct RecurringIntervalWithRepeatRuleFormattedToken(());

/// Output sidecar for the `format_recurring_interval_with_repeat_rule` exchange: the emitted text + a [`RecurringIntervalWithRepeatRuleFormatted`](crate::RecurringIntervalWithRepeatRuleFormatted) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::RecurringIntervalWithRepeatRuleFormatted",
    constructor = "pub"
)]
pub struct FormattedRecurringIntervalWithRepeatRule {
    #[sidecar(primary)]
    text: FormattedTemporalText,
    #[sidecar(token)]
    token: RecurringIntervalWithRepeatRuleFormattedToken,
}

impl FormattedRecurringIntervalWithRepeatRule {
    /// Borrow the emitted text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}

/// Descriptors the `evaluate_date_time_formula` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct EvaluateDateTimeFormulaRequest {
    /// The `formula` descriptor.
    formula: DateTimeFormulaDescriptor,
}

/// Preconditions the `evaluate_date_time_formula` exchange requires, folded into one proposition.
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
pub struct EvaluateDateTimeFormulaPreconditions {
    /// The established `date_time_formula_valid` sub-claim.
    date_time_formula_valid: DateTimeFormulaValid,
    /// The established `date_time_formula_evaluation_semantics_valid` sub-claim.
    date_time_formula_evaluation_semantics_valid: DateTimeFormulaEvaluationSemanticsValid,
}

/// Proofs the `evaluate_date_time_formula` exchange re-issues, folded into one proposition.
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
pub struct EvaluateDateTimeFormulaEstablished {
    /// The re-issued `explicit_temporal_form_valid` sub-claim.
    explicit_temporal_form_valid: ExplicitTemporalFormValid,
    /// The re-issued `date_time_formula_evaluation_result_valid` sub-claim.
    date_time_formula_evaluation_result_valid: DateTimeFormulaEvaluationResultValid,
}

/// Lawful token: [`EvaluateDateTimeFormulaPreconditions`](crate::EvaluateDateTimeFormulaPreconditions).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::EvaluateDateTimeFormulaPreconditions")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::EvaluateDateTimeFormulaPreconditions"
)]
pub struct EvaluateDateTimeFormulaPreconditionsToken(());

/// Lawful token: [`EvaluateDateTimeFormulaEstablished`](crate::EvaluateDateTimeFormulaEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::EvaluateDateTimeFormulaEstablished")]
#[amenable_derive::establish(
    credential = "crate::EvaluateDateTimeFormulaPreconditionsToken",
    proposition = "crate::EvaluateDateTimeFormulaEstablished"
)]
pub struct EvaluateDateTimeFormulaEstablishedToken(());

/// Input sidecar for the `evaluate_date_time_formula` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::EvaluateDateTimeFormulaPreconditions",
    constructor = "pub"
)]
pub struct EvaluateDateTimeFormulaInput {
    #[sidecar(primary)]
    request: EvaluateDateTimeFormulaRequest,
    #[sidecar(token)]
    token: EvaluateDateTimeFormulaPreconditionsToken,
}

impl EvaluateDateTimeFormulaInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &EvaluateDateTimeFormulaRequest {
        &self.request
    }
}

/// Output sidecar for the `evaluate_date_time_formula` exchange: the emitted
/// [`ExplicitTemporalValueDescriptor`](crate::ExplicitTemporalValueDescriptor) + a
/// [`EvaluateDateTimeFormulaEstablished`](crate::EvaluateDateTimeFormulaEstablished) token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::EvaluateDateTimeFormulaEstablished",
    constructor = "pub"
)]
pub struct EvaluateDateTimeFormulaOutput {
    #[sidecar(primary)]
    descriptor: ExplicitTemporalValueDescriptor,
    #[sidecar(token)]
    token: EvaluateDateTimeFormulaEstablishedToken,
}

impl EvaluateDateTimeFormulaOutput {
    /// Borrow the evaluated descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &ExplicitTemporalValueDescriptor {
        &self.descriptor
    }
}
