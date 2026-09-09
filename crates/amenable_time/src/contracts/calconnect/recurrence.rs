//! CalConnect CC 18012:2018 (*General recurrence representation*)
//! selection expressions (§5) and repeat rules (§6). See
//! [`super::explicit`] for the tier-B conventions.

use crate::NormativeQuotation;

temporal_standard! {
    /// A selection expression is delimited by `L...N`.
    SelectionExpressionUsesSelectionDelimiters => (
        document: "CalConnect CC 18012:2018", section: "5.1", body: CalConnect, status: Normative,
        summary: "a selection expression is enclosed by the L ... N selection markers",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// A selection expression uses the recognized rule vocabulary.
    SelectionExpressionUsesRecognizedSelectionRuleVocabulary => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a selection expression draws its rules only from the standard selection-rule vocabulary",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Month selection uses the `[monthE]` family.
    SelectionRuleMonthUsesMonthExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting the calendar month of the year uses the [monthE] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Week selection uses the `[weekE]` family.
    SelectionRuleWeekUsesWeekExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting the calendar week of the year uses the [weekE] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Day-of-month selection uses the `[dayE]` family.
    SelectionRuleDayOfMonthUsesDayExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting the calendar day of the month uses the [dayE] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Weekday selection uses the `[daykE]` family.
    SelectionRuleWeekDayUsesDayOfWeekExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting a day of the week uses the [daykE] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Ordinal-day selection uses the `[dayoE(m)]` family.
    SelectionRuleOrdinalDayOfYearUsesOrdinalDayExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting an ordinal day of the calendar year uses the [dayoE(m)] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Hour selection uses the `[hourE]` family.
    SelectionRuleHourUsesHourExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting an hour uses the [hourE] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Minute selection uses the `[minE]` family.
    SelectionRuleMinuteUsesMinuteExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting a minute uses the [minE] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Second selection uses the `[secE]` family.
    SelectionRuleSecondUsesSecondExpression => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a rule selecting a second uses the [secE] family",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Position selection uses an integer plus the instance designator.
    SelectionRulePositionUsesInstanceDesignatorSuffix => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a position-selection rule is an integer followed by the instance designator",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Selection rules apply within prior results.
    SelectionRulesApplyWithinSelectedResults => (
        document: "CalConnect CC 18012:2018", section: "5.3", body: CalConnect, status: Normative,
        summary: "each selection rule narrows the set already selected by the rules before it",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// A selection expression may pick a single instance.
    SelectionExpressionMaySelectSingleInstance => (
        document: "CalConnect CC 18012:2018", section: "5.1", body: CalConnect, status: Normative,
        summary: "a selection expression may resolve to exactly one instance by using the instance designator",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Position selection is applied last.
    SelectionRulePositionAppliesLast => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a position-based selection rule runs after all the preceding selection rules",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Selection-with-duration appends a duration suffix.
    SelectionWithDurationUsesDurationSuffix => (
        document: "CalConnect CC 18012:2018", section: "5.2", body: CalConnect, status: Normative,
        summary: "a selection component may be extended by appending an explicit duration suffix",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// A repeat rule uses the frequency designator `F`.
    RepeatRuleUsesFrequencyDesignator => (
        document: "CalConnect CC 18012:2018", section: "6.3.1", body: CalConnect, status: Normative,
        summary: "a repeat rule is introduced by the frequency designator F",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// A repeat rule declares its eligible time intervals.
    RepeatRuleDeclaresEligibleTimeIntervals => (
        document: "CalConnect CC 18012:2018", section: "6.3.2", body: CalConnect, status: Normative,
        summary: "a repeat rule states explicitly which time intervals its recurrence is eligible to fall within",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Repeat-rule selection applies within the eligible intervals.
    RepeatRuleSelectionAppliesWithinEligibleIntervals => (
        document: "CalConnect CC 18012:2018", section: "6.3.3", body: CalConnect, status: Normative,
        summary: "a repeat rule's selection part chooses instances from within the eligible intervals of each cycle",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// Repeat-rule evaluation inherits the start's components.
    RepeatRuleEvaluationInheritsInitialStartComponentInformation => (
        document: "CalConnect CC 18012:2018", section: "6.5", body: CalConnect, status: Normative,
        summary: "evaluating a repeat rule inherits time-scale component values from the initial start date",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
    /// A recurring representation with a repeat rule uses the complete form.
    RecurringIntervalWithRepeatRuleUsesCompleteRepresentation => (
        document: "CalConnect CC 18012:2018", section: "6.4", body: CalConnect, status: Normative,
        summary: "a recurring representation that carries a repeat rule uses the complete R.../.../... form",
        quotation: NormativeQuotation::ParaphraseOnly, url: "https://standards.calconnect.org/csd/cc-18012.html",
    );
}
