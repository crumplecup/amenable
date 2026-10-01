//! The `strip_named_zone` exchange.
//!
//!
//! `TemporalConversionFactory` exchange surface. Each transition
//! method's descriptors fold into a `*Request` primary, its
//! `Established<_>` preconditions into a `*Preconditions` proposition,
//! and its return-tuple proofs into a `*Established` proposition (the
//! `proof_composition` fold). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.
//!
//! Each `*Request` type gets a real `derive_new::new` constructor --
//! the same real, pre-existing gap already fixed in `exchange/zone.rs`
//! (private fields, `Getters`-only read access, no public constructor
//! of any kind).

use derive_new::new;

use crate::{
    ConversionDropsNamedZoneIdentity, ConversionPreservesRepresentedInstant,
    ConversionPreservesTemporalOrdering, OffsetConsistentWithNamedZone, OffsetDateTimeDescriptor,
    OffsetDateTimeValid, StripNamedZoneEstablishedToken, StripNamedZonePreconditionsToken,
    ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Descriptors the `strip_named_zone` exchange consumes.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    derive_getters::Getters,
    new,
)]
#[evidence(basis = "Self")]
pub struct StripNamedZoneRequest {
    /// The `timestamp` descriptor.
    timestamp: ZonedDateTimeDescriptor,
}
/// Preconditions the `strip_named_zone` exchange requires — the
/// caller's `Established<_>` sidecars, folded into one proposition.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct StripNamedZonePreconditions {
    /// The established `zoned_date_time_has_named_zone` sub-claim.
    zoned_date_time_has_named_zone: ZonedDateTimeHasNamedZone,
    /// The established `offset_consistent_with_named_zone` sub-claim.
    offset_consistent_with_named_zone: OffsetConsistentWithNamedZone,
}
/// Proofs the `strip_named_zone` exchange re-issues, folded into one
/// proposition (the `proof_composition` structural closure).
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Eq,
    Hash,
    amenable_derive::Evidence,
    amenable_derive::Witness,
    derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct StripNamedZoneEstablished {
    /// The re-issued `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The re-issued `conversion_drops_named_zone_identity` sub-claim.
    conversion_drops_named_zone_identity: ConversionDropsNamedZoneIdentity,
    /// The re-issued `conversion_preserves_represented_instant` sub-claim.
    conversion_preserves_represented_instant: ConversionPreservesRepresentedInstant,
    /// The re-issued `conversion_preserves_temporal_ordering` sub-claim.
    conversion_preserves_temporal_ordering: ConversionPreservesTemporalOrdering,
}
/// Input sidecar for the `strip_named_zone` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::StripNamedZonePreconditions",
    constructor = "pub"
)]
pub struct StripNamedZoneInput {
    #[sidecar(primary)]
    request: StripNamedZoneRequest,
    #[sidecar(token)]
    token: StripNamedZonePreconditionsToken,
}
/// Output sidecar for the `strip_named_zone` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`StripNamedZoneEstablished`](crate::StripNamedZoneEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::StripNamedZoneEstablished", constructor = "pub")]
pub struct StripNamedZoneOutput {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: StripNamedZoneEstablishedToken,
}
