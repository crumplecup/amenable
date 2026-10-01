//! Backend-conversion/precision semantic-bundle tokens (no native carrier).
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

/// Lawful token standing for the whole [`BackendConversionSemanticBundle`](crate::BackendConversionSemanticBundle),
/// minted from a proven [`TemporalInputToken`](crate::TemporalInputToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::BackendConversionSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::BackendConversionSemanticBundle"
)]
pub struct BackendConversionSemanticBundleToken(());
/// Lawful token standing for the whole [`LosslessConversionBundle`](crate::LosslessConversionBundle),
/// minted from a proven [`AdjustPrecisionLosslesslyEstablishedToken`](crate::AdjustPrecisionLosslesslyEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LosslessConversionBundle")]
#[amenable_derive::establish(
    credential = "crate::AdjustPrecisionLosslesslyEstablishedToken",
    proposition = "crate::LosslessConversionBundle"
)]
pub struct LosslessConversionBundleToken(());
/// Lawful token standing for the whole [`SubsecondTruncationBundle`](crate::SubsecondTruncationBundle),
/// minted from a proven [`TruncateSubsecondsEstablishedToken`](crate::TruncateSubsecondsEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::SubsecondTruncationBundle")]
#[amenable_derive::establish(
    credential = "crate::TruncateSubsecondsEstablishedToken",
    proposition = "crate::SubsecondTruncationBundle"
)]
pub struct SubsecondTruncationBundleToken(());
/// Lawful token standing for the whole [`LossyConversionAuthorityBundle`](crate::LossyConversionAuthorityBundle),
/// minted from a proven [`TemporalInputToken`](crate::TemporalInputToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LossyConversionAuthorityBundle")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::LossyConversionAuthorityBundle"
)]
pub struct LossyConversionAuthorityBundleToken(());
