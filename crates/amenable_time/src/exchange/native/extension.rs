//! Date-time-formula-evaluation native-carrier exchange types.
//!
//! Phase 5 Step 4b -- the `*_native` factory analogs' exchange types.
//! Carrier<->carrier: the native-carrier versions of the
//! descriptor-side factory transitions. Multi-input methods fold their
//! runtime values into a `<M>NativeRequest<B>` primary (basis = the
//! [`NativeCarrierRequest`] marker, so the compound needs no `Default`);
//! methods with an extra output proof fold it into a `<M>NativeEstablished`
//! composite behind a `<M>NativeToken`.

use crate::{
    DateTimeFormulaEvaluationResultBundle, ExplicitTemporalFormSemanticBundle,
    TemporalDateTimeFormulaProps, TemporalExplicitTemporalFormProps,
};

/// Proofs the `evaluate_date_time_formula_native` exchange re-issues, folded into one proposition.
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
pub struct EvaluateDateTimeFormulaNativeEstablished {
    /// The re-issued `explicit_temporal_form_semantic_bundle` sub-claim.
    explicit_temporal_form_semantic_bundle: ExplicitTemporalFormSemanticBundle,
    /// The re-issued `date_time_formula_evaluation_result_bundle` sub-claim.
    date_time_formula_evaluation_result_bundle: DateTimeFormulaEvaluationResultBundle,
}
/// Lawful token for the folded [`EvaluateDateTimeFormulaNativeEstablished`](crate::EvaluateDateTimeFormulaNativeEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::EvaluateDateTimeFormulaNativeEstablished")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::EvaluateDateTimeFormulaNativeEstablished"
)]
pub struct EvaluateDateTimeFormulaNativeToken(());
/// Output sidecar for the `evaluate_date_time_formula_native` exchange: the emitted
/// native `ExplicitTemporalForm` carrier + a [`EvaluateDateTimeFormulaNativeEstablished`](crate::EvaluateDateTimeFormulaNativeEstablished) token.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::EvaluateDateTimeFormulaNativeEstablished",
    constructor = "pub"
)]
pub struct EvaluateDateTimeFormulaNativeOutput<
    B: TemporalDateTimeFormulaProps + TemporalExplicitTemporalFormProps,
> {
    #[sidecar(primary)]
    carrier: B::ExplicitTemporalForm,
    #[sidecar(token)]
    token: EvaluateDateTimeFormulaNativeToken,
}
