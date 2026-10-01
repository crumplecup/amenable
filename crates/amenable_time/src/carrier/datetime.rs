//! Local/offset date-time semantic-bundle tokens and carriers.
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

/// Lawful token standing for the whole [`LocalDateTimeSemanticBundle`](crate::LocalDateTimeSemanticBundle),
/// minted from a proven [`LocalDateTimeProofToken`](crate::LocalDateTimeProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LocalDateTimeSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::LocalDateTimeProofToken",
    proposition = "crate::LocalDateTimeSemanticBundle"
)]
pub struct LocalDateTimeSemanticBundleToken(());
/// Lawful token standing for the whole [`OffsetDateTimeSemanticBundle`](crate::OffsetDateTimeSemanticBundle),
/// minted from a proven [`OffsetDateTimeProofToken`](crate::OffsetDateTimeProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::OffsetDateTimeSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::OffsetDateTimeProofToken",
    proposition = "crate::OffsetDateTimeSemanticBundle"
)]
pub struct OffsetDateTimeSemanticBundleToken(());
/// User-facing proven localdatetime carrier: a backend-native
/// `LocalDateTime` value under a [`LocalDateTimeSemanticBundle`](crate::LocalDateTimeSemanticBundle) token.
pub type ProvenLocalDateTimeCarrier<T> = ProvenTemporalCarrier<T, LocalDateTimeSemanticBundleToken>;
/// User-facing proven offsetdatetime carrier: a backend-native
/// `OffsetDateTime` value under a [`OffsetDateTimeSemanticBundle`](crate::OffsetDateTimeSemanticBundle) token.
pub type ProvenOffsetDateTimeCarrier<T> =
    ProvenTemporalCarrier<T, OffsetDateTimeSemanticBundleToken>;
