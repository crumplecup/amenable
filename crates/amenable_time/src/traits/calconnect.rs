//! [`TemporalCalConnectFactory`] — the CalConnect extension seam,
//! ported from `elicit_temporal::traits::TemporalCalConnectFactory`.
//! Same shape as [`TemporalParser`](crate::TemporalParser): the
//! supertrait bundle *is* the 19 CalConnect exchanges (6 parse, 6
//! emit, `evaluate_date_time_formula`, + the 6 shared families that
//! reuse the parser / formatter sidecars).

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    DateTimeFormulaFormatted, DateTimeFormulaProof, EvaluateDateTimeFormulaEstablished,
    EvaluateDateTimeFormulaInput, EvaluateDateTimeFormulaOutput,
    EvaluateDateTimeFormulaPreconditions, ExplicitDurationFormatted, ExplicitDurationProof,
    ExplicitTemporalFormFormatted, ExplicitTemporalFormProof, ExplicitTimeIntervalFormatted,
    ExplicitTimeIntervalProof, FormattedDateTimeFormula, FormattedExplicitDuration,
    FormattedExplicitTemporalForm, FormattedExplicitTimeInterval, FormattedGroupedTimeScaleUnit,
    FormattedQualifiedTemporalValue, FormattedRecurringIntervalWithRepeatRule, FormattedRepeatRule,
    FormattedSelectionExpression, GroupedTimeScaleUnitFormatted, GroupedTimeScaleUnitProof,
    ParsedDateTimeFormula, ParsedExplicitDuration, ParsedExplicitTemporalForm,
    ParsedExplicitTimeInterval, ParsedGroupedTimeScaleUnit, ParsedQualifiedTemporalValue,
    ParsedRecurringIntervalWithRepeatRule, ParsedRepeatRule, ParsedSelectionExpression,
    QualifiedTemporalValueFormatted, QualifiedTemporalValueProof, RawInput,
    RecurringIntervalWithRepeatRuleFormatted, RecurringIntervalWithRepeatRuleProof,
    RepeatRuleFormatted, RepeatRuleProof, SelectionExpressionFormatted, SelectionExpressionProof,
    TemporalError, TemporalInputReceived,
};

/// A backend that parses, emits, and evaluates the CalConnect
/// extensions as [`Exchange`]s, for verifier `V`.
pub trait TemporalCalConnectFactory<V: Verifier>: Send
    + Sync
    + Exchange<RawInput, ParsedQualifiedTemporalValue, V, Error = TemporalError>
    + Exchange<RawInput, ParsedGroupedTimeScaleUnit, V, Error = TemporalError>
    + Exchange<RawInput, ParsedDateTimeFormula, V, Error = TemporalError>
    + Exchange<
        ParsedQualifiedTemporalValue,
        FormattedQualifiedTemporalValue,
        V,
        Error = TemporalError,
    > + Exchange<ParsedGroupedTimeScaleUnit, FormattedGroupedTimeScaleUnit, V, Error = TemporalError>
    + Exchange<ParsedDateTimeFormula, FormattedDateTimeFormula, V, Error = TemporalError>
    + Exchange<RawInput, ParsedExplicitTemporalForm, V, Error = TemporalError>
    + Exchange<ParsedExplicitTemporalForm, FormattedExplicitTemporalForm, V, Error = TemporalError>
    + Exchange<RawInput, ParsedExplicitDuration, V, Error = TemporalError>
    + Exchange<ParsedExplicitDuration, FormattedExplicitDuration, V, Error = TemporalError>
    + Exchange<RawInput, ParsedExplicitTimeInterval, V, Error = TemporalError>
    + Exchange<ParsedExplicitTimeInterval, FormattedExplicitTimeInterval, V, Error = TemporalError>
    + Exchange<RawInput, ParsedSelectionExpression, V, Error = TemporalError>
    + Exchange<ParsedSelectionExpression, FormattedSelectionExpression, V, Error = TemporalError>
    + Exchange<RawInput, ParsedRepeatRule, V, Error = TemporalError>
    + Exchange<ParsedRepeatRule, FormattedRepeatRule, V, Error = TemporalError>
    + Exchange<RawInput, ParsedRecurringIntervalWithRepeatRule, V, Error = TemporalError>
    + Exchange<
        ParsedRecurringIntervalWithRepeatRule,
        FormattedRecurringIntervalWithRepeatRule,
        V,
        Error = TemporalError,
    > + Exchange<EvaluateDateTimeFormulaInput, EvaluateDateTimeFormulaOutput, V, Error = TemporalError>
where
    TemporalInputReceived: Witness<V>,
    QualifiedTemporalValueProof: Witness<V>,
    GroupedTimeScaleUnitProof: Witness<V>,
    DateTimeFormulaProof: Witness<V>,
    QualifiedTemporalValueFormatted: Witness<V>,
    GroupedTimeScaleUnitFormatted: Witness<V>,
    DateTimeFormulaFormatted: Witness<V>,
    ExplicitTemporalFormProof: Witness<V>,
    ExplicitTemporalFormFormatted: Witness<V>,
    ExplicitDurationProof: Witness<V>,
    ExplicitDurationFormatted: Witness<V>,
    ExplicitTimeIntervalProof: Witness<V>,
    ExplicitTimeIntervalFormatted: Witness<V>,
    SelectionExpressionProof: Witness<V>,
    SelectionExpressionFormatted: Witness<V>,
    RepeatRuleProof: Witness<V>,
    RepeatRuleFormatted: Witness<V>,
    RecurringIntervalWithRepeatRuleProof: Witness<V>,
    RecurringIntervalWithRepeatRuleFormatted: Witness<V>,
    EvaluateDateTimeFormulaPreconditions: Witness<V>,
    EvaluateDateTimeFormulaEstablished: Witness<V>,
{
}

impl<T, V> TemporalCalConnectFactory<V> for T
where
    V: Verifier,
    TemporalInputReceived: Witness<V>,
    QualifiedTemporalValueProof: Witness<V>,
    GroupedTimeScaleUnitProof: Witness<V>,
    DateTimeFormulaProof: Witness<V>,
    QualifiedTemporalValueFormatted: Witness<V>,
    GroupedTimeScaleUnitFormatted: Witness<V>,
    DateTimeFormulaFormatted: Witness<V>,
    ExplicitTemporalFormProof: Witness<V>,
    ExplicitTemporalFormFormatted: Witness<V>,
    ExplicitDurationProof: Witness<V>,
    ExplicitDurationFormatted: Witness<V>,
    ExplicitTimeIntervalProof: Witness<V>,
    ExplicitTimeIntervalFormatted: Witness<V>,
    SelectionExpressionProof: Witness<V>,
    SelectionExpressionFormatted: Witness<V>,
    RepeatRuleProof: Witness<V>,
    RepeatRuleFormatted: Witness<V>,
    RecurringIntervalWithRepeatRuleProof: Witness<V>,
    RecurringIntervalWithRepeatRuleFormatted: Witness<V>,
    EvaluateDateTimeFormulaPreconditions: Witness<V>,
    EvaluateDateTimeFormulaEstablished: Witness<V>,
    T: Send
        + Sync
        + Exchange<RawInput, ParsedQualifiedTemporalValue, V, Error = TemporalError>
        + Exchange<RawInput, ParsedGroupedTimeScaleUnit, V, Error = TemporalError>
        + Exchange<RawInput, ParsedDateTimeFormula, V, Error = TemporalError>
        + Exchange<
            ParsedQualifiedTemporalValue,
            FormattedQualifiedTemporalValue,
            V,
            Error = TemporalError,
        > + Exchange<
            ParsedGroupedTimeScaleUnit,
            FormattedGroupedTimeScaleUnit,
            V,
            Error = TemporalError,
        > + Exchange<ParsedDateTimeFormula, FormattedDateTimeFormula, V, Error = TemporalError>
        + Exchange<RawInput, ParsedExplicitTemporalForm, V, Error = TemporalError>
        + Exchange<
            ParsedExplicitTemporalForm,
            FormattedExplicitTemporalForm,
            V,
            Error = TemporalError,
        > + Exchange<RawInput, ParsedExplicitDuration, V, Error = TemporalError>
        + Exchange<ParsedExplicitDuration, FormattedExplicitDuration, V, Error = TemporalError>
        + Exchange<RawInput, ParsedExplicitTimeInterval, V, Error = TemporalError>
        + Exchange<
            ParsedExplicitTimeInterval,
            FormattedExplicitTimeInterval,
            V,
            Error = TemporalError,
        > + Exchange<RawInput, ParsedSelectionExpression, V, Error = TemporalError>
        + Exchange<ParsedSelectionExpression, FormattedSelectionExpression, V, Error = TemporalError>
        + Exchange<RawInput, ParsedRepeatRule, V, Error = TemporalError>
        + Exchange<ParsedRepeatRule, FormattedRepeatRule, V, Error = TemporalError>
        + Exchange<RawInput, ParsedRecurringIntervalWithRepeatRule, V, Error = TemporalError>
        + Exchange<
            ParsedRecurringIntervalWithRepeatRule,
            FormattedRecurringIntervalWithRepeatRule,
            V,
            Error = TemporalError,
        > + Exchange<
            EvaluateDateTimeFormulaInput,
            EvaluateDateTimeFormulaOutput,
            V,
            Error = TemporalError,
        >,
{
}
