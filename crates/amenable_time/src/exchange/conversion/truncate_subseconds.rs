//! The `truncate_subseconds` exchange.
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
    BackendConversionSemanticsValid, ConversionTruncatesSubseconds, LossyConversionAuthorityValid,
    OffsetDateTimeDescriptor, OffsetDateTimeValid, PrecisionDescriptor,
    TimestampRepresentsFixedInstant, TruncateSubsecondsEstablishedToken,
    TruncateSubsecondsPreconditionsToken,
};

/// Descriptors the `truncate_subseconds` exchange consumes.
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
pub struct TruncateSubsecondsRequest {
    /// The `timestamp` descriptor.
    timestamp: OffsetDateTimeDescriptor,
    /// The `target_precision` descriptor.
    target_precision: PrecisionDescriptor,
}
/// Preconditions the `truncate_subseconds` exchange requires — the
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
pub struct TruncateSubsecondsPreconditions {
    /// The established `timestamp_represents_fixed_instant` sub-claim.
    timestamp_represents_fixed_instant: TimestampRepresentsFixedInstant,
    /// The established `lossy_conversion_authority_valid` sub-claim.
    lossy_conversion_authority_valid: LossyConversionAuthorityValid,
}
/// Proofs the `truncate_subseconds` exchange re-issues, folded into one
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
pub struct TruncateSubsecondsEstablished {
    /// The re-issued `offset_date_time_valid` sub-claim.
    offset_date_time_valid: OffsetDateTimeValid,
    /// The re-issued `backend_conversion_semantics_valid` sub-claim.
    backend_conversion_semantics_valid: BackendConversionSemanticsValid,
    /// The re-issued `conversion_truncates_subseconds` sub-claim.
    conversion_truncates_subseconds: ConversionTruncatesSubseconds,
}
/// Input sidecar for the `truncate_subseconds` exchange.
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::TruncateSubsecondsPreconditions",
    constructor = "pub"
)]
pub struct TruncateSubsecondsInput {
    #[sidecar(primary)]
    request: TruncateSubsecondsRequest,
    #[sidecar(token)]
    token: TruncateSubsecondsPreconditionsToken,
}
/// Output sidecar for the `truncate_subseconds` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`TruncateSubsecondsEstablished`](crate::TruncateSubsecondsEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::TruncateSubsecondsEstablished",
    constructor = "pub"
)]
pub struct TruncateSubsecondsOutput {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: TruncateSubsecondsEstablishedToken,
}
