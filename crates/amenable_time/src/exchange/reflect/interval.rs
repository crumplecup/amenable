//! Duration/interval/recurring-interval `Reflected<X>` sidecars.
//!
//!
//! `Reflected<X>` sidecars -- a neutral descriptor paired with its
//! aggregate-semantics token (`proof_composition::semantic_bundles`).
//! This is the shape a `realize_*` bridge consumes and a `reflect_*`
//! bridge produces: the token rides straight through from / to the
//! `ProvenTemporalCarrier`, so no new `#[establish]` edge is minted
//! here. The `Exchange` impls live in the backend crate.

use crate::{
    DurationDescriptor, DurationSemanticBundleToken, ExplicitDurationDescriptor,
    ExplicitDurationSemanticBundleToken, ExplicitTimeIntervalDescriptor,
    ExplicitTimeIntervalSemanticBundleToken, RecurringIntervalDescriptor,
    RecurringIntervalSemanticBundleToken, TimeIntervalDescriptor, TimeIntervalSemanticBundleToken,
};

/// A neutral [`DurationDescriptor`](crate::DurationDescriptor) paired with a token for the
/// folded [`DurationSemanticBundle`](crate::DurationSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::DurationSemanticBundle", constructor = "pub")]
pub struct ReflectedDuration {
    #[sidecar(primary)]
    descriptor: DurationDescriptor,
    #[sidecar(token)]
    token: DurationSemanticBundleToken,
}
/// A neutral [`TimeIntervalDescriptor`](crate::TimeIntervalDescriptor) paired with a token for the
/// folded [`TimeIntervalSemanticBundle`](crate::TimeIntervalSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TimeIntervalSemanticBundle", constructor = "pub")]
pub struct ReflectedTimeInterval {
    #[sidecar(primary)]
    descriptor: TimeIntervalDescriptor,
    #[sidecar(token)]
    token: TimeIntervalSemanticBundleToken,
}
/// A neutral [`RecurringIntervalDescriptor`](crate::RecurringIntervalDescriptor) paired with a token for the
/// folded [`RecurringIntervalSemanticBundle`](crate::RecurringIntervalSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::RecurringIntervalSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedRecurringInterval {
    #[sidecar(primary)]
    descriptor: RecurringIntervalDescriptor,
    #[sidecar(token)]
    token: RecurringIntervalSemanticBundleToken,
}
/// A neutral [`ExplicitDurationDescriptor`](crate::ExplicitDurationDescriptor) paired with a token for the
/// folded [`ExplicitDurationSemanticBundle`](crate::ExplicitDurationSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::ExplicitDurationSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedExplicitDuration {
    #[sidecar(primary)]
    descriptor: ExplicitDurationDescriptor,
    #[sidecar(token)]
    token: ExplicitDurationSemanticBundleToken,
}
/// A neutral [`ExplicitTimeIntervalDescriptor`](crate::ExplicitTimeIntervalDescriptor) paired with a token for the
/// folded [`ExplicitTimeIntervalSemanticBundle`](crate::ExplicitTimeIntervalSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::ExplicitTimeIntervalSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedExplicitTimeInterval {
    #[sidecar(primary)]
    descriptor: ExplicitTimeIntervalDescriptor,
    #[sidecar(token)]
    token: ExplicitTimeIntervalSemanticBundleToken,
}
