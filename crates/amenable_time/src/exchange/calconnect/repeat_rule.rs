//! `parse_repeat_rule`/`format_repeat_rule` sidecars (CalConnect extension family).

use crate::{
    FormattedTemporalText, RepeatRuleDeclaresEligibleTimeIntervals, RepeatRuleDescriptor,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    RepeatRuleValid,
};

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
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::RepeatRuleProof", constructor = "pub")]
pub struct ParsedRepeatRule {
    #[sidecar(primary)]
    descriptor: RepeatRuleDescriptor,
    #[sidecar(token)]
    token: RepeatRuleProofToken,
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
    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self)))]
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.value()
    }
}
