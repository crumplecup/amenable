//! `TemporalIntervalFactory` exchange surface. Each transition
//! method's descriptors fold into a `*Request` primary, its
//! `Established<_>` preconditions into a `*Preconditions` proposition,
//! and its return-tuple proofs into a `*Established` proposition (the
//! `proof_composition` fold). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    IntervalEndpointsOrdered, OffsetDateTimeDescriptor, OrderOffsetEndpointsEstablishedToken,
    OrderOffsetEndpointsPreconditionsToken, TimestampRepresentsFixedInstant,
};

/// Descriptors the `order_offset_endpoints` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
)]
#[evidence(basis = "Self")]
pub struct OrderOffsetEndpointsRequest {
    /// The `start` descriptor.
    start: OffsetDateTimeDescriptor,
    /// The `end` descriptor.
    end: OffsetDateTimeDescriptor,
}

/// Preconditions the `order_offset_endpoints` exchange requires — the
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
pub struct OrderOffsetEndpointsPreconditions {
    /// The established `start` sub-claim.
    start: TimestampRepresentsFixedInstant,
    /// The established `end` sub-claim.
    end: TimestampRepresentsFixedInstant,
}

/// Proofs the `order_offset_endpoints` exchange re-issues, folded into one
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
pub struct OrderOffsetEndpointsEstablished {
    /// The re-issued `interval_endpoints_ordered` sub-claim.
    interval_endpoints_ordered: IntervalEndpointsOrdered,
}

/// Input sidecar for the `order_offset_endpoints` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::OrderOffsetEndpointsPreconditions",
    constructor = "pub"
)]
pub struct OrderOffsetEndpointsInput {
    #[sidecar(primary)]
    request: OrderOffsetEndpointsRequest,
    #[sidecar(token)]
    token: OrderOffsetEndpointsPreconditionsToken,
}

impl OrderOffsetEndpointsInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &OrderOffsetEndpointsRequest {
        &self.request
    }
}

/// Output sidecar for the `order_offset_endpoints` exchange — no descriptor,
/// so the folded [`OrderOffsetEndpointsEstablished`](crate::OrderOffsetEndpointsEstablished) proposition
/// is itself the primary payload.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(constructor = "pub")]
pub struct OrderOffsetEndpointsOutput {
    #[sidecar(primary)]
    established: OrderOffsetEndpointsEstablished,
    #[sidecar(token)]
    token: OrderOffsetEndpointsEstablishedToken,
}

impl OrderOffsetEndpointsOutput {
    /// Borrow the re-issued proof composite.
    #[must_use]
    pub fn established(&self) -> &OrderOffsetEndpointsEstablished {
        &self.established
    }
}
