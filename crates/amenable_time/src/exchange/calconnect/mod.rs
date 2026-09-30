//! `TemporalCalConnectFactory` exchange surface (Phase 4 Step 5).
//! Self-contained: the 6 CalConnect-only parse methods + their 6
//! emit counterparts + `evaluate_date_time_formula`, each folded the
//! `proof_composition` way (`*Proof` for a parse, `*Formatted` for an
//! emit, `*Preconditions`/`*Established` for the transition). The 6
//! shared families (qualified value, grouped unit, date-time formula)
//! reuse the parser / formatter sidecars. `Exchange` impls: backend.
//!
//! Split one file per family: each holds both its parse-side
//! (`*Proof`/`*ProofToken`/`Parsed*`) and emit-side
//! (`*Formatted`/`*FormattedToken`/`Formatted*`) sidecars together,
//! since they're the same concern's two directions.

mod evaluate_date_time_formula;
mod explicit_duration;
mod explicit_temporal_form;
mod explicit_time_interval;
mod recurring_interval_with_repeat_rule;
mod repeat_rule;
mod selection_expression;

pub use evaluate_date_time_formula::{
    EvaluateDateTimeFormulaEstablished, EvaluateDateTimeFormulaEstablishedToken,
    EvaluateDateTimeFormulaInput, EvaluateDateTimeFormulaOutput,
    EvaluateDateTimeFormulaPreconditions, EvaluateDateTimeFormulaPreconditionsToken,
    EvaluateDateTimeFormulaRequest,
};
pub use explicit_duration::{
    ExplicitDurationFormatted, ExplicitDurationFormattedToken, ExplicitDurationProof,
    ExplicitDurationProofToken, FormattedExplicitDuration, ParsedExplicitDuration,
};
pub use explicit_temporal_form::{
    ExplicitTemporalFormFormatted, ExplicitTemporalFormFormattedToken, ExplicitTemporalFormProof,
    ExplicitTemporalFormProofToken, FormattedExplicitTemporalForm, ParsedExplicitTemporalForm,
};
pub use explicit_time_interval::{
    ExplicitTimeIntervalFormatted, ExplicitTimeIntervalFormattedToken, ExplicitTimeIntervalProof,
    ExplicitTimeIntervalProofToken, FormattedExplicitTimeInterval, ParsedExplicitTimeInterval,
};
pub use recurring_interval_with_repeat_rule::{
    FormattedRecurringIntervalWithRepeatRule, ParsedRecurringIntervalWithRepeatRule,
    RecurringIntervalWithRepeatRuleFormatted, RecurringIntervalWithRepeatRuleFormattedToken,
    RecurringIntervalWithRepeatRuleProof, RecurringIntervalWithRepeatRuleProofToken,
};
pub use repeat_rule::{
    FormattedRepeatRule, ParsedRepeatRule, RepeatRuleFormatted, RepeatRuleFormattedToken,
    RepeatRuleProof, RepeatRuleProofToken,
};
pub use selection_expression::{
    FormattedSelectionExpression, ParsedSelectionExpression, SelectionExpressionFormatted,
    SelectionExpressionFormattedToken, SelectionExpressionProof, SelectionExpressionProofToken,
};
