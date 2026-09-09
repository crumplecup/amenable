//! Phase 5 carrier surface. Each aggregate proof bundle
//! (`proof_composition::semantic_bundles`) gets one `<Bundle>Token`,
//! `#[establish]`-minted from the token that produced it — the
//! bundle proposition is the *named aggregate*, the token *stands for
//! it*. [`ProvenTemporalCarrier`] pairs a backend-native carrier `T`
//! with one such token; keyed on the token so the token stays the
//! single source of truth (the proposition is
//! `<STok as ProofToken>::Proposition`).

// The `#[proof_token]` / `#[establish]` / `#[sidecar]` attributes re-parse
// their `crate::…` string arguments as paths, so no `use` is needed here.

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

/// Lawful token standing for the whole [`NamedTimeZoneSemanticBundle`](crate::NamedTimeZoneSemanticBundle),
/// minted from a proven [`NamedTimeZoneIdentityValidToken`](crate::NamedTimeZoneIdentityValidToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NamedTimeZoneSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::NamedTimeZoneIdentityValidToken",
    proposition = "crate::NamedTimeZoneSemanticBundle"
)]
pub struct NamedTimeZoneSemanticBundleToken(());

/// Lawful token standing for the whole [`ZonedDateTimeSemanticBundle`](crate::ZonedDateTimeSemanticBundle),
/// minted from a proven [`AttachNamedZoneEstablishedToken`](crate::AttachNamedZoneEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ZonedDateTimeSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::AttachNamedZoneEstablishedToken",
    proposition = "crate::ZonedDateTimeSemanticBundle"
)]
pub struct ZonedDateTimeSemanticBundleToken(());

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

/// Lawful token standing for the whole [`BackendConversionSemanticBundle`](crate::BackendConversionSemanticBundle),
/// minted from a proven [`TemporalInputToken`](crate::TemporalInputToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::BackendConversionSemanticBundle")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::BackendConversionSemanticBundle"
)]
pub struct BackendConversionSemanticBundleToken(());

/// Lawful token standing for the whole [`ZoneTransitionResolutionAuthorityBundle`](crate::ZoneTransitionResolutionAuthorityBundle),
/// minted from a proven [`ConfirmZoneAuthorityEstablishedToken`](crate::ConfirmZoneAuthorityEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ZoneTransitionResolutionAuthorityBundle")]
#[amenable_derive::establish(
    credential = "crate::ConfirmZoneAuthorityEstablishedToken",
    proposition = "crate::ZoneTransitionResolutionAuthorityBundle"
)]
pub struct ZoneTransitionResolutionAuthorityBundleToken(());

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

/// Lawful token standing for the whole [`NamedTimeZoneRevisionBundle`](crate::NamedTimeZoneRevisionBundle),
/// minted from a proven [`ConfirmNamedZoneRevisionEstablishedToken`](crate::ConfirmNamedZoneRevisionEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NamedTimeZoneRevisionBundle")]
#[amenable_derive::establish(
    credential = "crate::ConfirmNamedZoneRevisionEstablishedToken",
    proposition = "crate::NamedTimeZoneRevisionBundle"
)]
pub struct NamedTimeZoneRevisionBundleToken(());

/// Lawful token standing for the whole [`IntervalEndpointOrderingBundle`](crate::IntervalEndpointOrderingBundle),
/// minted from a proven [`OrderOffsetEndpointsEstablishedToken`](crate::OrderOffsetEndpointsEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::IntervalEndpointOrderingBundle")]
#[amenable_derive::establish(
    credential = "crate::OrderOffsetEndpointsEstablishedToken",
    proposition = "crate::IntervalEndpointOrderingBundle"
)]
pub struct IntervalEndpointOrderingBundleToken(());

/// Lawful token standing for the whole [`DateTimeFormulaEvaluationResultBundle`](crate::DateTimeFormulaEvaluationResultBundle),
/// minted from a proven [`EvaluateDateTimeFormulaEstablishedToken`](crate::EvaluateDateTimeFormulaEstablishedToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::DateTimeFormulaEvaluationResultBundle")]
#[amenable_derive::establish(
    credential = "crate::EvaluateDateTimeFormulaEstablishedToken",
    proposition = "crate::DateTimeFormulaEvaluationResultBundle"
)]
pub struct DateTimeFormulaEvaluationResultBundleToken(());

/// Lawful token standing for the whole [`LossyConversionAuthorityBundle`](crate::LossyConversionAuthorityBundle),
/// minted from a proven [`TemporalInputToken`](crate::TemporalInputToken).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::LossyConversionAuthorityBundle")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::LossyConversionAuthorityBundle"
)]
pub struct LossyConversionAuthorityBundleToken(());

/// Generic user-facing wrapper: a backend-native carrier `T` paired
/// with one proof token `STok` standing for its whole semantic
/// bundle. A [`Sidecar`](amenable_core::Sidecar) — `carrier` is the
/// primary payload, `semantics` the sidecar token, and the
/// proposition is recovered from the token as
/// `<STok as ProofToken>::Proposition`. A `T` only ever exists
/// because a native factory produced it under proof, so this pairing
/// is lawful by construction.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition_from_token, constructor = "pub")]
pub struct ProvenTemporalCarrier<T, STok>
where
    T: ::amenable_core::Evidence,
    STok: ::amenable_core::ProofToken,
{
    #[sidecar(primary)]
    carrier: T,
    #[sidecar(token)]
    semantics: STok,
}

impl<T, STok> ProvenTemporalCarrier<T, STok>
where
    T: ::amenable_core::Evidence,
    STok: ::amenable_core::ProofToken,
{
    /// Borrow the backend-native carrier.
    #[must_use]
    pub fn carrier(&self) -> &T {
        &self.carrier
    }
}

/// User-facing proven localdatetime carrier: a backend-native
/// `LocalDateTime` value under a [`LocalDateTimeSemanticBundle`](crate::LocalDateTimeSemanticBundle) token.
pub type ProvenLocalDateTimeCarrier<T> = ProvenTemporalCarrier<T, LocalDateTimeSemanticBundleToken>;

/// User-facing proven offsetdatetime carrier: a backend-native
/// `OffsetDateTime` value under a [`OffsetDateTimeSemanticBundle`](crate::OffsetDateTimeSemanticBundle) token.
pub type ProvenOffsetDateTimeCarrier<T> =
    ProvenTemporalCarrier<T, OffsetDateTimeSemanticBundleToken>;

/// User-facing proven namedtimezone carrier: a backend-native
/// `NamedTimeZone` value under a [`NamedTimeZoneSemanticBundle`](crate::NamedTimeZoneSemanticBundle) token.
pub type ProvenNamedTimeZoneCarrier<T> = ProvenTemporalCarrier<T, NamedTimeZoneSemanticBundleToken>;

/// User-facing proven zoneddatetime carrier: a backend-native
/// `ZonedDateTime` value under a [`ZonedDateTimeSemanticBundle`](crate::ZonedDateTimeSemanticBundle) token.
pub type ProvenZonedDateTimeCarrier<T> = ProvenTemporalCarrier<T, ZonedDateTimeSemanticBundleToken>;

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

/// User-facing proven qualifiedtemporalvalue carrier: a backend-native
/// `QualifiedTemporalValue` value under a [`QualifiedTemporalValueSemanticBundle`](crate::QualifiedTemporalValueSemanticBundle) token.
pub type ProvenQualifiedTemporalValueCarrier<T> =
    ProvenTemporalCarrier<T, QualifiedTemporalValueSemanticBundleToken>;

/// User-facing proven explicittemporalform carrier: a backend-native
/// `ExplicitTemporalForm` value under a [`ExplicitTemporalFormSemanticBundle`](crate::ExplicitTemporalFormSemanticBundle) token.
pub type ProvenExplicitTemporalFormCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitTemporalFormSemanticBundleToken>;

/// User-facing proven explicitduration carrier: a backend-native
/// `ExplicitDuration` value under a [`ExplicitDurationSemanticBundle`](crate::ExplicitDurationSemanticBundle) token.
pub type ProvenExplicitDurationCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitDurationSemanticBundleToken>;

/// User-facing proven explicittimeinterval carrier: a backend-native
/// `ExplicitTimeInterval` value under a [`ExplicitTimeIntervalSemanticBundle`](crate::ExplicitTimeIntervalSemanticBundle) token.
pub type ProvenExplicitTimeIntervalCarrier<T> =
    ProvenTemporalCarrier<T, ExplicitTimeIntervalSemanticBundleToken>;

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
