//! The CalConnect explicit temporal form and its selection-expression refinement.
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    ExplicitTemporalFormMayOmitZeroValuedComponents, ExplicitTemporalFormUsesDesignatorSymbols,
    ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitUtcRelationshipUsesZuluOrSignedShift, SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionRuleDayOfMonthUsesDayExpression,
    SelectionRuleHourUsesHourExpression, SelectionRuleMinuteUsesMinuteExpression,
    SelectionRuleMonthUsesMonthExpression, SelectionRulePositionAppliesLast,
    SelectionRulePositionUsesInstanceDesignatorSuffix, SelectionRuleSecondUsesSecondExpression,
    SelectionRuleWeekDayUsesDayOfWeekExpression, SelectionRuleWeekUsesWeekExpression,
    SelectionRulesApplyWithinSelectedResults, SelectionWithDurationUsesDurationSuffix,
};

/// Aggregate proof that a CalConnect explicit form is structurally valid.
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
pub struct ExplicitTemporalFormValid {
    /// Explicit forms use designator symbols.
    designators: ExplicitTemporalFormUsesDesignatorSymbols,
    /// Zero-valued components may be omitted when the result remains valid.
    zero_omission: ExplicitTemporalFormMayOmitZeroValuedComponents,
    /// The lowest denoted component declares precision.
    precision: ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    /// UTC relationship syntax uses `Z` or a signed shift.
    utc_relationship: ExplicitUtcRelationshipUsesZuluOrSignedShift,
}

/// Aggregate proof that a selection expression is structurally valid.
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
pub struct SelectionExpressionValid {
    /// Selection expressions use the `L...N` delimiters.
    delimiters: SelectionExpressionUsesSelectionDelimiters,
    /// Selection expressions use the standard rule vocabulary.
    vocabulary: SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    /// Month-selection rules use the month expression family.
    month_rule: SelectionRuleMonthUsesMonthExpression,
    /// Week-selection rules use the week expression family.
    week_rule: SelectionRuleWeekUsesWeekExpression,
    /// Day-of-month selection rules use the day expression family.
    day_of_month_rule: SelectionRuleDayOfMonthUsesDayExpression,
    /// Weekday selection rules use the day-of-week expression family.
    weekday_rule: SelectionRuleWeekDayUsesDayOfWeekExpression,
    /// Hour-selection rules use the hour expression family.
    hour_rule: SelectionRuleHourUsesHourExpression,
    /// Minute-selection rules use the minute expression family.
    minute_rule: SelectionRuleMinuteUsesMinuteExpression,
    /// Second-selection rules use the second expression family.
    second_rule: SelectionRuleSecondUsesSecondExpression,
    /// Subsequent components apply within previously selected results.
    nesting: SelectionRulesApplyWithinSelectedResults,
    /// Single-instance selection is explicitly modeled.
    single_instance: SelectionExpressionMaySelectSingleInstance,
    /// Position rules use an integer followed by the instance designator.
    position_syntax: SelectionRulePositionUsesInstanceDesignatorSuffix,
    /// Position rules apply after earlier selection rules.
    position: SelectionRulePositionAppliesLast,
    /// Selection-with-duration uses an explicit duration suffix.
    duration_window: SelectionWithDurationUsesDurationSuffix,
}
