//! Recurring-interval and repeat-rule propositions.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    RecurringIntervalCarriesIntervalComponent, RecurringIntervalCountIsNonNegativeWhenBounded,
    RecurringIntervalOmittedCountDenotesUnboundedOccurrences,
    RecurringIntervalUsesRepeatDesignator, RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
};

/// Aggregate proof that complete recurring-interval representation semantics are established.
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
pub struct CompleteRecurringIntervalRepresentationSemanticsValid {
    /// The recurring-interval wrapper is structurally valid.
    recurring_interval: RecurringIntervalFormValid,
}

/// Aggregate proof that other-than-complete recurring-interval semantics are established.
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
pub struct OtherThanCompleteRecurringIntervalRepresentationSemanticsValid {
    /// The recurring-interval wrapper is structurally valid.
    recurring_interval: RecurringIntervalFormValid,
}

/// Aggregate proof that a recurring interval representation is structurally valid.
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
pub struct RecurringIntervalFormValid {
    /// The representation begins with the repetition designator.
    repeat_designator: RecurringIntervalUsesRepeatDesignator,
    /// Any bounded recurrence count is non-negative.
    repeat_count: RecurringIntervalCountIsNonNegativeWhenBounded,
    /// Omitting the recurrence count denotes an unbounded occurrence stream.
    omitted_count: RecurringIntervalOmittedCountDenotesUnboundedOccurrences,
    /// The repetition prefix is followed by an interval component.
    interval_component: RecurringIntervalCarriesIntervalComponent,
}

/// Aggregate proof that a recurring interval with repeat-rule refinement is structurally valid.
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
pub struct RecurringIntervalWithRepeatRuleValid {
    /// The recurring-interval prefix and interval component are structurally valid.
    recurring_interval: RecurringIntervalFormValid,
    /// The attached repeat rule carries its full repeat-law sidecars.
    repeat_rule: RepeatRuleValid,
}

/// Aggregate proof that a repeat rule is structurally valid.
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
pub struct RepeatRuleValid {
    /// Repeat rules use the frequency designator.
    frequency: RepeatRuleUsesFrequencyDesignator,
    /// Eligible time intervals are explicitly declared.
    eligible_intervals: RepeatRuleDeclaresEligibleTimeIntervals,
    /// Selection rules apply within the eligible intervals.
    selection: RepeatRuleSelectionAppliesWithinEligibleIntervals,
    /// Evaluation inherits component information from the initial start date.
    inheritance: RepeatRuleEvaluationInheritsInitialStartComponentInformation,
}
