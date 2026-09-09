//! CalConnect CC 18011:2018 (*Date and time — Explicit representation*)
//! and CC 18012:2018 (*Date and time — General recurrence
//! representation*) propositions (61 in `elicit_temporal`).
//!
//! Tier B: CalConnect specs are publicly available but copyrighted.
//! Primary quotation is `ParaphraseOnly` for now (`SemanticSummary` +
//! clause pointer + CSD URL carry the citation); a verbatim single-clause
//! upgrade is a follow-on.

mod explicit;
pub use explicit::{
    ExplicitDateTimeTimePortionMayBeReducedPrecision,
    ExplicitDateTimeUsesDateThenTimeConcatenation,
    ExplicitDateTimeWithShiftUsesDateTimeThenShiftConcatenation,
    ExplicitDateWithShiftUsesDateThenShiftConcatenation,
    ExplicitTemporalFormMayOmitZeroValuedComponents, ExplicitTemporalFormUsesDesignatorSymbols,
    ExplicitTemporalPrecisionUsesLowestDenotedComponent,
    ExplicitTimeIntervalDurationSubstitutionInfersMissingBoundary,
    ExplicitTimeIntervalLeadingShiftAppliesToTrailingComponentUnlessOverridden,
    ExplicitTimeIntervalTrailingEndMayInheritHigherOrderComponents,
    ExplicitTimeIntervalUsesDateTimeEndpointFamily, ExplicitTimeOfDayForbidsEndOfDayRepresentation,
    ExplicitTimeOfDayUsesHourMinuteSecondUnitDesignators, ExplicitTimeOfDayUsesTimeDesignator,
    ExplicitTimeOfDayWithShiftUsesTimeThenShiftConcatenation,
    ExplicitTimeShiftBareZuluRepresentsUtcZero, ExplicitTimeShiftPayloadUsesExplicitTimeOfDay,
    ExplicitTimeShiftUsesLeadingMinusOnlyWhenBehindUtc, ExplicitTimeShiftUsesZuluDesignator,
    ExplicitUtcRelationshipUsesZuluOrSignedShift,
};
mod grouped_and_duration;
pub use grouped_and_duration::{
    ContextDependentDurationSemanticsDeclared, DateTimeFormulaCombinesTemporalValueWithDuration,
    DateTimeFormulaEvaluationModeDeclared, DateTimeFormulaTruncatesAtComponentBoundaries,
    DateTimeFormulaUsesCarryOverSemantics, ExactDurationSemanticsDeclared,
    ExplicitDurationCompositeRepresentationDeclared, ExplicitDurationMayBeNegative,
    ExplicitDurationMayUseFractionalLowestOrderUnit,
    ExplicitDurationPrecedenceRepresentationCarriesEvaluationOrder,
    ExplicitDurationRepresentationKindDeclared, ExplicitDurationUsesDurationalUnitDesignators,
    GroupedTimeScaleUnitCarriesOneOrMoreDurationUnits, GroupedTimeScaleUnitConvertsToTimeInterval,
    GroupedTimeScaleUnitDateTimeMayCarryExplicitTimeShift,
    GroupedTimeScaleUnitDefinitionIsContinuous,
    GroupedTimeScaleUnitLowerOrderUnitsRemainWithinGroupBounds,
    GroupedTimeScaleUnitTruncatesOutOfBoundsRemainder, GroupedTimeScaleUnitUsesGroupingDesignators,
    GroupedTimeScaleUnitValueCarriesExplicitCoefficient, SpeculativeDurationSemanticsDeclared,
};
mod recurrence;
pub use recurrence::{
    RecurringIntervalWithRepeatRuleUsesCompleteRepresentation,
    RepeatRuleDeclaresEligibleTimeIntervals,
    RepeatRuleEvaluationInheritsInitialStartComponentInformation,
    RepeatRuleSelectionAppliesWithinEligibleIntervals, RepeatRuleUsesFrequencyDesignator,
    SelectionExpressionMaySelectSingleInstance,
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary,
    SelectionExpressionUsesSelectionDelimiters, SelectionRuleDayOfMonthUsesDayExpression,
    SelectionRuleHourUsesHourExpression, SelectionRuleMinuteUsesMinuteExpression,
    SelectionRuleMonthUsesMonthExpression, SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression,
    SelectionRulePositionAppliesLast, SelectionRulePositionUsesInstanceDesignatorSuffix,
    SelectionRuleSecondUsesSecondExpression, SelectionRuleWeekDayUsesDayOfWeekExpression,
    SelectionRuleWeekUsesWeekExpression, SelectionRulesApplyWithinSelectedResults,
    SelectionWithDurationUsesDurationSuffix,
};
