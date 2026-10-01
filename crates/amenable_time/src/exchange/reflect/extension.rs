//! CalConnect/ISO 8601-2 extension-family `Reflected<X>` sidecars.
//!
//!
//! `Reflected<X>` sidecars -- a neutral descriptor paired with its
//! aggregate-semantics token (`proof_composition::semantic_bundles`).
//! This is the shape a `realize_*` bridge consumes and a `reflect_*`
//! bridge produces: the token rides straight through from / to the
//! `ProvenTemporalCarrier`, so no new `#[establish]` edge is minted
//! here. The `Exchange` impls live in the backend crate.

use crate::{
    DateTimeFormulaDescriptor, DateTimeFormulaSemanticBundleToken, ExplicitTemporalFormDescriptor,
    ExplicitTemporalFormSemanticBundleToken, GroupedTimeScaleUnitDescriptor,
    GroupedTimeScaleUnitSemanticBundleToken, QualifiedTemporalValueDescriptor,
    QualifiedTemporalValueSemanticBundleToken, TemporalSetDescriptor,
    TemporalSetSemanticBundleToken,
};

/// A neutral [`QualifiedTemporalValueDescriptor`](crate::QualifiedTemporalValueDescriptor) paired with a token for the
/// folded [`QualifiedTemporalValueSemanticBundle`](crate::QualifiedTemporalValueSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::QualifiedTemporalValueSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedQualifiedTemporalValue {
    #[sidecar(primary)]
    descriptor: QualifiedTemporalValueDescriptor,
    #[sidecar(token)]
    token: QualifiedTemporalValueSemanticBundleToken,
}
/// A neutral [`ExplicitTemporalFormDescriptor`](crate::ExplicitTemporalFormDescriptor) paired with a token for the
/// folded [`ExplicitTemporalFormSemanticBundle`](crate::ExplicitTemporalFormSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::ExplicitTemporalFormSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedExplicitTemporalForm {
    #[sidecar(primary)]
    descriptor: ExplicitTemporalFormDescriptor,
    #[sidecar(token)]
    token: ExplicitTemporalFormSemanticBundleToken,
}
/// A neutral [`GroupedTimeScaleUnitDescriptor`](crate::GroupedTimeScaleUnitDescriptor) paired with a token for the
/// folded [`GroupedTimeScaleUnitSemanticBundle`](crate::GroupedTimeScaleUnitSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::GroupedTimeScaleUnitSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedGroupedTimeScaleUnit {
    #[sidecar(primary)]
    descriptor: GroupedTimeScaleUnitDescriptor,
    #[sidecar(token)]
    token: GroupedTimeScaleUnitSemanticBundleToken,
}
/// A neutral [`TemporalSetDescriptor`](crate::TemporalSetDescriptor) paired with a token for the
/// folded [`TemporalSetSemanticBundle`](crate::TemporalSetSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TemporalSetSemanticBundle", constructor = "pub")]
pub struct ReflectedTemporalSet {
    #[sidecar(primary)]
    descriptor: TemporalSetDescriptor,
    #[sidecar(token)]
    token: TemporalSetSemanticBundleToken,
}
/// A neutral [`DateTimeFormulaDescriptor`](crate::DateTimeFormulaDescriptor) paired with a token for the
/// folded [`DateTimeFormulaSemanticBundle`](crate::DateTimeFormulaSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::DateTimeFormulaSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedDateTimeFormula {
    #[sidecar(primary)]
    descriptor: DateTimeFormulaDescriptor,
    #[sidecar(token)]
    token: DateTimeFormulaSemanticBundleToken,
}
