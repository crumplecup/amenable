//! Temporal sets and date-time formula evaluation (derived/composed expressions).
//!
//!
//! Ported from `elicit_temporal::contracts::proof_composition`. Each
//! aggregate is folded with its `*Evidence` bundle: the aggregate's
//! fields ARE its sub-claims, and `#[derive(Witness)]` makes the
//! composite proof the structural product of its members' proofs.
//! See [`super`] for the design; leaf `Witness<V>` impls land in the
//! backend crates.

use crate::{
    DateTimeFormulaCombinesTemporalValueWithDuration, DateTimeFormulaEvaluationModeDeclared,
    DateTimeFormulaTruncatesAtComponentBoundaries, DateTimeFormulaUsesCarryOverSemantics,
    ExplicitTemporalFormValid, TemporalSetCarriesMultipleMembers,
    TemporalSetForbidsInternalWhitespace, TemporalSetMemberSeparatorDeclared,
    TemporalSetOpenRangeUsesBoundaryDoubleDot, TemporalSetRangeNeighborhoodSharesPrecision,
    TemporalSetRangeUsesInclusiveDoubleDotSemantics,
};

/// Aggregate proof that a temporal set expression is structurally valid.
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
pub struct TemporalSetExpressionValid {
    /// Member expressions are explicitly separated.
    separator: TemporalSetMemberSeparatorDeclared,
    /// The set carries multiple temporal members.
    members: TemporalSetCarriesMultipleMembers,
}

/// Aggregate proof that refined temporal-set range semantics are structurally valid.
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
pub struct TemporalSetRangeSemanticsValid {
    /// The underlying temporal-set expression is structurally valid.
    set: TemporalSetExpressionValid,
    /// No internal whitespace appears within the expression.
    no_whitespace: TemporalSetForbidsInternalWhitespace,
    /// `..` denotes the inclusive values between bounded range endpoints.
    inclusive_range: TemporalSetRangeUsesInclusiveDoubleDotSemantics,
    /// Leading or trailing `..` denotes an open-ended boundary.
    open_range: TemporalSetOpenRangeUsesBoundaryDoubleDot,
    /// Values adjacent to a range share the same precision as the range expansion.
    precision: TemporalSetRangeNeighborhoodSharesPrecision,
}

/// Aggregate proof that a date-time formula is structurally valid.
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
pub struct DateTimeFormulaValid {
    /// The formula combines an explicit temporal value with a duration.
    combination: DateTimeFormulaCombinesTemporalValueWithDuration,
    /// Overflow is carried across component boundaries.
    carry_over: DateTimeFormulaUsesCarryOverSemantics,
    /// Evaluation truncates at component boundaries when required.
    truncation: DateTimeFormulaTruncatesAtComponentBoundaries,
}

/// Aggregate proof that date-time formula evaluation semantics are fully declared.
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
pub struct DateTimeFormulaEvaluationSemanticsValid {
    /// The underlying formula is structurally valid.
    formula: DateTimeFormulaValid,
    /// The formula declares its evaluation family.
    evaluation_mode: DateTimeFormulaEvaluationModeDeclared,
}

/// Aggregate proof that an explicit temporal result was lawfully produced by date-time formula evaluation.
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
pub struct DateTimeFormulaEvaluationResultValid {
    /// The originating formula is structurally valid.
    formula: DateTimeFormulaValid,
    /// The originating formula declares a lawful evaluation family.
    semantics: DateTimeFormulaEvaluationSemanticsValid,
    /// The produced explicit temporal value is structurally valid.
    result: ExplicitTemporalFormValid,
}
