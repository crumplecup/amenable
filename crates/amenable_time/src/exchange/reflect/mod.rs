//! `Reflected<X>` sidecars — a neutral descriptor paired with its
//! aggregate-semantics token (`proof_composition::semantic_bundles`).
//! This is the shape a `realize_*` bridge consumes and a `reflect_*`
//! bridge produces: the token rides straight through from / to the
//! `ProvenTemporalCarrier`, so no new `#[establish]` edge is minted
//! here. The `Exchange` impls live in the backend crate.
//!
//! Split by the same real domains as `carrier`/`semantic_bundles`:
//! `datetime`, `zone`, `interval`, `extension`.

mod datetime;
mod extension;
mod interval;
mod zone;

pub use datetime::{ReflectedLocalDateTime, ReflectedOffsetDateTime};
pub use extension::{
    ReflectedDateTimeFormula, ReflectedExplicitTemporalForm, ReflectedGroupedTimeScaleUnit,
    ReflectedQualifiedTemporalValue, ReflectedTemporalSet,
};
pub use interval::{
    ReflectedDuration, ReflectedExplicitDuration, ReflectedExplicitTimeInterval,
    ReflectedRecurringInterval, ReflectedTimeInterval,
};
pub use zone::{ReflectedNamedTimeZone, ReflectedZonedDateTime};
