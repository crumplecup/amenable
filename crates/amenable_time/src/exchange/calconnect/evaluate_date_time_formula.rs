//! `evaluate_date_time_formula` sidecars (CalConnect extension family).

use crate::{
    DateTimeFormulaDescriptor, DateTimeFormulaEvaluationResultValid,
    DateTimeFormulaEvaluationSemanticsValid, DateTimeFormulaValid, ExplicitTemporalFormValid,
    ExplicitTemporalValueDescriptor,
};

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
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
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

/// Output sidecar for the `evaluate_date_time_formula` exchange: the emitted
/// [`ExplicitTemporalValueDescriptor`](crate::ExplicitTemporalValueDescriptor) + a
/// [`EvaluateDateTimeFormulaEstablished`](crate::EvaluateDateTimeFormulaEstablished) token.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
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
