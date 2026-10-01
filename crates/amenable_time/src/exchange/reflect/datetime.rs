//! Local/offset date-time `Reflected<X>` sidecars.
//!
//!
//! `Reflected<X>` sidecars -- a neutral descriptor paired with its
//! aggregate-semantics token (`proof_composition::semantic_bundles`).
//! This is the shape a `realize_*` bridge consumes and a `reflect_*`
//! bridge produces: the token rides straight through from / to the
//! `ProvenTemporalCarrier`, so no new `#[establish]` edge is minted
//! here. The `Exchange` impls live in the backend crate.

use crate::{
    LocalDateTimeDescriptor, LocalDateTimeSemanticBundleToken, OffsetDateTimeDescriptor,
    OffsetDateTimeSemanticBundleToken,
};

/// A neutral [`LocalDateTimeDescriptor`](crate::LocalDateTimeDescriptor) paired with a token for the
/// folded [`LocalDateTimeSemanticBundle`](crate::LocalDateTimeSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::LocalDateTimeSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedLocalDateTime {
    #[sidecar(primary)]
    descriptor: LocalDateTimeDescriptor,
    #[sidecar(token)]
    token: LocalDateTimeSemanticBundleToken,
}
/// A neutral [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) paired with a token for the
/// folded [`OffsetDateTimeSemanticBundle`](crate::OffsetDateTimeSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::OffsetDateTimeSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedOffsetDateTime {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: OffsetDateTimeSemanticBundleToken,
}
