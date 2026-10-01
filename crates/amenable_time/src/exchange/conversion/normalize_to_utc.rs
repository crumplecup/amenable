//! The `normalize_to_utc` exchange.
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
    ConversionPreservesRepresentedInstant, ConversionPreservesTemporalOrdering,
    NormalizeToUtcEstablishedToken, NormalizeToUtcPreconditionsToken, OffsetDateTimeDescriptor,
    OffsetDateTimeValid, TimestampRepresentsFixedInstant,
};

/// Descriptors the `normalize_to_utc` exchange consumes.
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
pub struct NormalizeToUtcRequest {
    /// The `timestamp` descriptor.
    timestamp: OffsetDateTimeDescriptor,
}
/// Preconditions the `normalize_to_utc` exchange requires — the
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
pub struct NormalizeToUtcPreconditions {
    /// The established `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
}
/// Proofs the `normalize_to_utc` exchange re-issues, folded into one
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
pub struct NormalizeToUtcEstablished {
    /// The re-issued `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The re-issued `conversion_preserves_represented_instant` sub-claim.
    conversion_preserves_represented_instant: ConversionPreservesRepresentedInstant,
    /// The re-issued `conversion_preserves_temporal_ordering` sub-claim.
    conversion_preserves_temporal_ordering: ConversionPreservesTemporalOrdering,
}
/// Input sidecar for the `normalize_to_utc` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::NormalizeToUtcPreconditions",
    constructor = "pub"
)]
pub struct NormalizeToUtcInput {
    #[sidecar(primary)]
    request: NormalizeToUtcRequest,
    #[sidecar(token)]
    token: NormalizeToUtcPreconditionsToken,
}
/// Output sidecar for the `normalize_to_utc` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`NormalizeToUtcEstablished`](crate::NormalizeToUtcEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::NormalizeToUtcEstablished", constructor = "pub")]
pub struct NormalizeToUtcOutput {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: NormalizeToUtcEstablishedToken,
}
