//! `parse_recurring_interval_with_repeat_rule`/`format_recurring_interval_with_repeat_rule` sidecars (CalConnect extension family).

use crate::{
    FormattedTemporalText, RecurringIntervalWithRepeatRuleDescriptor,
    RecurringIntervalWithRepeatRuleIntervalProofBranch, RecurringIntervalWithRepeatRuleValid,
    RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
};

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
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
