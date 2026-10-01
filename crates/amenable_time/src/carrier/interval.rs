//! Duration/interval/recurring-interval semantic-bundle tokens and carriers.
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

/// Lawful token standing for the whole [`DurationSemanticBundle`](crate::DurationSemanticBundle),
/// minted from a proven [`DurationFormValidToken`](crate::DurationFormValidToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DurationSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::DurationFormValidToken",
    proposition = "crate::DurationSemanticBundle"
)]
pub struct DurationSemanticBundleToken(());
/// Lawful token standing for the whole [`TimeIntervalSemanticBundle`](crate::TimeIntervalSemanticBundle),
/// minted from a proven [`TimeIntervalProofToken`](crate::TimeIntervalProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TimeIntervalSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::TimeIntervalProofToken",
    proposition = "crate::TimeIntervalSemanticBundle"
)]
pub struct TimeIntervalSemanticBundleToken(());
/// Lawful token standing for the whole [`RecurringIntervalSemanticBundle`](crate::RecurringIntervalSemanticBundle),
/// minted from a proven [`RecurringIntervalFormValidToken`](crate::RecurringIntervalFormValidToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::RecurringIntervalSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::RecurringIntervalFormValidToken",
    proposition = "crate::RecurringIntervalSemanticBundle"
)]
pub struct RecurringIntervalSemanticBundleToken(());
/// Lawful token standing for the whole [`ExplicitDurationSemanticBundle`](crate::ExplicitDurationSemanticBundle),
/// minted from a proven [`ExplicitDurationProofToken`](crate::ExplicitDurationProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitDurationSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::ExplicitDurationProofToken",
    proposition = "crate::ExplicitDurationSemanticBundle"
)]
pub struct ExplicitDurationSemanticBundleToken(());
/// Lawful token standing for the whole [`ExplicitTimeIntervalSemanticBundle`](crate::ExplicitTimeIntervalSemanticBundle),
/// minted from a proven [`ExplicitTimeIntervalProofToken`](crate::ExplicitTimeIntervalProofToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ExplicitTimeIntervalSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::ExplicitTimeIntervalProofToken",
    proposition = "crate::ExplicitTimeIntervalSemanticBundle"
)]
pub struct ExplicitTimeIntervalSemanticBundleToken(());
/// Lawful token standing for the whole [`IntervalEndpointOrderingBundle`](crate::IntervalEndpointOrderingBundle),
/// minted from a proven [`OrderOffsetEndpointsEstablishedToken`](crate::OrderOffsetEndpointsEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IntervalEndpointOrderingBundle")]
#[amenable_derive::establish(
    credential = "crate::OrderOffsetEndpointsEstablishedToken",
    proposition = "crate::IntervalEndpointOrderingBundle"
)]
pub struct IntervalEndpointOrderingBundleToken(());
/// User-facing proven duration carrier: a backend-native
/// `Duration` value under a [`DurationSemanticBundle`](crate::DurationSemanticBundle) token.
pub type ProvenDurationCarrier<T> = ProvenTemporalCarrier<T, DurationSemanticBundleToken>;
/// User-facing proven timeinterval carrier: a backend-native
/// `TimeInterval` value under a [`TimeIntervalSemanticBundle`](crate::TimeIntervalSemanticBundle) token.
pub type ProvenTimeIntervalCarrier<T> = ProvenTemporalCarrier<T, TimeIntervalSemanticBundleToken>;
/// User-facing proven recurringinterval carrier: a backend-native
/// `RecurringInterval` value under a [`RecurringIntervalSemanticBundle`](crate::RecurringIntervalSemanticBundle) token.
pub type ProvenRecurringIntervalCarrier<T> =
    ProvenTemporalCarrier<T, RecurringIntervalSemanticBundleToken>;
/// User-facing proven explicitduration carrier: a backend-native
/// `ExplicitDuration` value under a [`ExplicitDurationSemanticBundle`](crate::ExplicitDurationSemanticBundle) token.
pub type ProvenExplicitDurationCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitDurationSemanticBundleToken>;
/// User-facing proven explicittimeinterval carrier: a backend-native
/// `ExplicitTimeInterval` value under a [`ExplicitTimeIntervalSemanticBundle`](crate::ExplicitTimeIntervalSemanticBundle) token.
pub type ProvenExplicitTimeIntervalCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitTimeIntervalSemanticBundleToken>;
