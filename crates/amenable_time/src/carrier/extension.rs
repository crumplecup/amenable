//! CalConnect/ISO 8601-2 extension-family semantic-bundle tokens and carriers.
//!
//! Phase 5 carrier surface. Each aggregate proof bundle
//! (`proof_composition::semantic_bundles`) gets one `<Bundle>Token`,
//! `#[establish]`-minted from the token that produced it -- the
//! bundle proposition is the *named aggregate*, the token *stands for
//! it*. [`ProvenTemporalCarrier`] pairs a backend-native carrier `T`
//! with one such token; keyed on the token so the token stays the
//! single source of truth (the proposition is
//! `<STok as ProofToken>::Proposition`).
//!
//! The `#[proof_token]` / `#[establish]` / `#[sidecar]` attributes
//! re-parse their `crate::...` string arguments as paths, so no `use`
//! is needed for the token structs themselves.

use crate::ProvenTemporalCarrier;

/// Lawful token standing for the whole [`QualifiedTemporalValueSemanticBundle`](crate::QualifiedTemporalValueSemanticBundle),
/// minted from a proven [`QualifiedTemporalValueProofToken`](crate::QualifiedTemporalValueProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::QualifiedTemporalValueSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::QualifiedTemporalValueProofToken",
    proposition = "crate::QualifiedTemporalValueSemanticBundle"
)]
pub struct QualifiedTemporalValueSemanticBundleToken(());
/// Lawful token standing for the whole [`ExplicitTemporalFormSemanticBundle`](crate::ExplicitTemporalFormSemanticBundle),
/// minted from a proven [`ExplicitTemporalFormProofToken`](crate::ExplicitTemporalFormProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTemporalFormSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::ExplicitTemporalFormProofToken",
    proposition = "crate::ExplicitTemporalFormSemanticBundle"
)]
pub struct ExplicitTemporalFormSemanticBundleToken(());
/// Lawful token standing for the whole [`GroupedTimeScaleUnitSemanticBundle`](crate::GroupedTimeScaleUnitSemanticBundle),
/// minted from a proven [`GroupedTimeScaleUnitProofToken`](crate::GroupedTimeScaleUnitProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::GroupedTimeScaleUnitSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::GroupedTimeScaleUnitProofToken",
    proposition = "crate::GroupedTimeScaleUnitSemanticBundle"
)]
pub struct GroupedTimeScaleUnitSemanticBundleToken(());
/// Lawful token standing for the whole [`TemporalSetSemanticBundle`](crate::TemporalSetSemanticBundle),
/// minted from a proven [`TemporalSetProofToken`](crate::TemporalSetProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TemporalSetSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::TemporalSetProofToken",
    proposition = "crate::TemporalSetSemanticBundle"
)]
pub struct TemporalSetSemanticBundleToken(());
/// Lawful token standing for the whole [`DateTimeFormulaSemanticBundle`](crate::DateTimeFormulaSemanticBundle),
/// minted from a proven [`DateTimeFormulaProofToken`](crate::DateTimeFormulaProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateTimeFormulaSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::DateTimeFormulaProofToken",
    proposition = "crate::DateTimeFormulaSemanticBundle"
)]
pub struct DateTimeFormulaSemanticBundleToken(());
/// Lawful token standing for the whole [`DateTimeFormulaEvaluationResultBundle`](crate::DateTimeFormulaEvaluationResultBundle),
/// minted from a proven [`EvaluateDateTimeFormulaEstablishedToken`](crate::EvaluateDateTimeFormulaEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateTimeFormulaEvaluationResultBundle")]
#[amenable_derive::establish(
    credential = "crate::EvaluateDateTimeFormulaEstablishedToken",
    proposition = "crate::DateTimeFormulaEvaluationResultBundle"
)]
pub struct DateTimeFormulaEvaluationResultBundleToken(());
/// User-facing proven qualifiedtemporalvalue carrier: a backend-native
/// `QualifiedTemporalValue` value under a [`QualifiedTemporalValueSemanticBundle`](crate::QualifiedTemporalValueSemanticBundle) token.
pub type ProvenQualifiedTemporalValueCarrier<T> =
    ProvenTemporalCarrier<T, QualifiedTemporalValueSemanticBundleToken>;
/// User-facing proven explicittemporalform carrier: a backend-native
/// `ExplicitTemporalForm` value under a [`ExplicitTemporalFormSemanticBundle`](crate::ExplicitTemporalFormSemanticBundle) token.
pub type ProvenExplicitTemporalFormCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitTemporalFormSemanticBundleToken>;
/// User-facing proven groupedtimescaleunit carrier: a backend-native
/// `GroupedTimeScaleUnit` value under a [`GroupedTimeScaleUnitSemanticBundle`](crate::GroupedTimeScaleUnitSemanticBundle) token.
pub type ProvenGroupedTimeScaleUnitCarrier<T> =
    ProvenTemporalCarrier<T, GroupedTimeScaleUnitSemanticBundleToken>;
/// User-facing proven temporalset carrier: a backend-native
/// `TemporalSet` value under a [`TemporalSetSemanticBundle`](crate::TemporalSetSemanticBundle) token.
pub type ProvenTemporalSetCarrier<T> = ProvenTemporalCarrier<T, TemporalSetSemanticBundleToken>;
/// User-facing proven datetimeformula carrier: a backend-native
/// `DateTimeFormula` value under a [`DateTimeFormulaSemanticBundle`](crate::DateTimeFormulaSemanticBundle) token.
pub type ProvenDateTimeFormulaCarrier<T> =
    ProvenTemporalCarrier<T, DateTimeFormulaSemanticBundleToken>;
