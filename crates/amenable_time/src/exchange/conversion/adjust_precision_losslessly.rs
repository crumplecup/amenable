//! The `adjust_precision_losslessly` exchange.
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
    AdjustPrecisionLosslesslyEstablishedToken, AdjustPrecisionLosslesslyPreconditionsToken,
    ConversionLossless, OffsetDateTimeDescriptor, OffsetDateTimeValid, PrecisionDescriptor,
    TimestampRepresentsFixedInstant,
};

/// Descriptors the `adjust_precision_losslessly` exchange consumes.
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
pub struct AdjustPrecisionLosslesslyRequest {
    /// The `timestamp` descriptor.
    timestamp: OffsetDateTimeDescriptor,
    /// The `target_precision` descriptor.
    target_precision: PrecisionDescriptor,
}
/// Preconditions the `adjust_precision_losslessly` exchange requires — the
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
pub struct AdjustPrecisionLosslesslyPreconditions {
    /// The established `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
}
/// Proofs the `adjust_precision_losslessly` exchange re-issues, folded into one
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
pub struct AdjustPrecisionLosslesslyEstablished {
    /// The re-issued `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The re-issued `conversion_lossless` sub-claim.
    conversion_lossless: ConversionLossless,
}
/// Input sidecar for the `adjust_precision_losslessly` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::AdjustPrecisionLosslesslyPreconditions",
    constructor = "pub"
)]
pub struct AdjustPrecisionLosslesslyInput {
    #[sidecar(primary)]
    request: AdjustPrecisionLosslesslyRequest,
    #[sidecar(token)]
    token: AdjustPrecisionLosslesslyPreconditionsToken,
}
/// Output sidecar for the `adjust_precision_losslessly` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`AdjustPrecisionLosslesslyEstablished`](crate::AdjustPrecisionLosslesslyEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::AdjustPrecisionLosslesslyEstablished",
    constructor = "pub"
)]
pub struct AdjustPrecisionLosslesslyOutput {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: AdjustPrecisionLosslesslyEstablishedToken,
}
