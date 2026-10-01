//! Phase 5 carrier surface. Each aggregate proof bundle
//! (`proof_composition::semantic_bundles`) gets one `<Bundle>Token`,
//! `#[establish]`-minted from the token that produced it — the
//! bundle proposition is the *named aggregate*, the token *stands for
//! it*. [`ProvenTemporalCarrier`] pairs a backend-native carrier `T`
//! with one such token; keyed on the token so the token stays the
//! single source of truth (the proposition is
//! `<STok as ProofToken>::Proposition`).
//!
//! Split by the same real domains as `proof_composition::composites`:
//! `proven_carrier` (the generic wrapper itself), `datetime`, `zone`,
//! `interval`, `extension`, `conversion`.

mod conversion;
mod datetime;
mod extension;
mod interval;
mod proven_carrier;
mod zone;

pub use conversion::{
    BackendConversionSemanticBundleToken, LosslessConversionBundleToken,
    LossyConversionAuthorityBundleToken, SubsecondTruncationBundleToken,
};
pub use datetime::{
    LocalDateTimeSemanticBundleToken, OffsetDateTimeSemanticBundleToken,
    ProvenLocalDateTimeCarrier, ProvenOffsetDateTimeCarrier,
};
pub use extension::{
    DateTimeFormulaEvaluationResultBundleToken, DateTimeFormulaSemanticBundleToken,
    ExplicitTemporalFormSemanticBundleToken, GroupedTimeScaleUnitSemanticBundleToken,
    ProvenDateTimeFormulaCarrier, ProvenExplicitTemporalFormCarrier,
    ProvenGroupedTimeScaleUnitCarrier, ProvenQualifiedTemporalValueCarrier,
    ProvenTemporalSetCarrier, QualifiedTemporalValueSemanticBundleToken,
    TemporalSetSemanticBundleToken,
};
pub use interval::{
    DurationSemanticBundleToken, ExplicitDurationSemanticBundleToken,
    ExplicitTimeIntervalSemanticBundleToken, IntervalEndpointOrderingBundleToken,
    ProvenDurationCarrier, ProvenExplicitDurationCarrier, ProvenExplicitTimeIntervalCarrier,
    ProvenRecurringIntervalCarrier, ProvenTimeIntervalCarrier,
    RecurringIntervalSemanticBundleToken, TimeIntervalSemanticBundleToken,
};
pub use proven_carrier::ProvenTemporalCarrier;
pub use zone::{
    NamedTimeZoneRevisionBundleToken, NamedTimeZoneSemanticBundleToken, ProvenNamedTimeZoneCarrier,
    ProvenZonedDateTimeCarrier, ZoneTransitionResolutionAuthorityBundleToken,
    ZonedDateTimeSemanticBundleToken,
};
