//! The `elicit_temporal` `*Bundle` aggregate proof bundles, ported as
//! folded `#[derive(Evidence, Witness)]` composites (the same
//! structural closure as the rest of `proof_composition`). Each
//! `Established<X>` field collapses to `X`; a `<Foo>Evidence` field
//! already merged into its `*Valid` sibling is dropped; `*ProofBranch`
//! and standalone `*Evidence` fields are kept. These pair with the
//! Phase 5 `ProvenTemporalCarrier` carrier wrapper.
//!
//! Split by the same real domains as `composites`: `conversion`,
//! `datetime`, `zone`, `interval`, `extension`.

mod conversion;
mod datetime;
mod extension;
mod interval;
mod zone;

pub use conversion::{
    BackendConversionSemanticBundle, LosslessConversionBundle, LossyConversionAuthorityBundle,
    QualifiedTemporalValueSemanticBundle, SubsecondTruncationBundle,
};
pub use datetime::{LocalDateTimeSemanticBundle, OffsetDateTimeSemanticBundle};
pub use extension::{
    DateTimeFormulaEvaluationResultBundle, DateTimeFormulaSemanticBundle,
    ExplicitTemporalFormSemanticBundle, GroupedTimeScaleUnitSemanticBundle,
    TemporalSetSemanticBundle,
};
pub use interval::{
    DurationSemanticBundle, ExplicitDurationSemanticBundle, ExplicitTimeIntervalSemanticBundle,
    IntervalEndpointOrderingBundle, RecurringIntervalSemanticBundle, TimeIntervalSemanticBundle,
};
pub use zone::{
    NamedTimeZoneRevisionBundle, NamedTimeZoneSemanticBundle,
    ZoneTransitionResolutionAuthorityBundle, ZonedDateTimeSemanticBundle,
};
