//! UTC-offset/named-zone `Reflected<X>` sidecars.
//!
//!
//! `Reflected<X>` sidecars -- a neutral descriptor paired with its
//! aggregate-semantics token (`proof_composition::semantic_bundles`).
//! This is the shape a `realize_*` bridge consumes and a `reflect_*`
//! bridge produces: the token rides straight through from / to the
//! `ProvenTemporalCarrier`, so no new `#[establish]` edge is minted
//! here. The `Exchange` impls live in the backend crate.

use crate::{
    NamedTimeZoneDescriptor, NamedTimeZoneSemanticBundleToken, ZonedDateTimeDescriptor,
    ZonedDateTimeSemanticBundleToken,
};

/// A neutral [`NamedTimeZoneDescriptor`](crate::NamedTimeZoneDescriptor) paired with a token for the
/// folded [`NamedTimeZoneSemanticBundle`](crate::NamedTimeZoneSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::NamedTimeZoneSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedNamedTimeZone {
    #[sidecar(primary)]
    descriptor: NamedTimeZoneDescriptor,
    #[sidecar(token)]
    token: NamedTimeZoneSemanticBundleToken,
}
/// A neutral [`ZonedDateTimeDescriptor`](crate::ZonedDateTimeDescriptor) paired with a token for the
/// folded [`ZonedDateTimeSemanticBundle`](crate::ZonedDateTimeSemanticBundle).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::ZonedDateTimeSemanticBundle",
    constructor = "pub"
)]
pub struct ReflectedZonedDateTime {
    #[sidecar(primary)]
    descriptor: ZonedDateTimeDescriptor,
    #[sidecar(token)]
    token: ZonedDateTimeSemanticBundleToken,
}
