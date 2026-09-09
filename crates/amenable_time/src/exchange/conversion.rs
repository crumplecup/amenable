//! `TemporalConversionFactory` exchange surface. Each transition
//! method's descriptors fold into a `*Request` primary, its
//! `Established<_>` preconditions into a `*Preconditions` proposition,
//! and its return-tuple proofs into a `*Established` proposition (the
//! `proof_composition` fold). The `Exchange` impls live in the backend
//! crate (`#[capture_exchange_body]`); `amenable_time` ships the shape.

use crate::{
    AdjustPrecisionLosslesslyEstablishedToken, AdjustPrecisionLosslesslyPreconditionsToken,
    BackendConversionSemanticsValid, ConversionDropsNamedZoneIdentity, ConversionLossless,
    ConversionPreservesRepresentedInstant, ConversionPreservesTemporalOrdering,
    ConversionTruncatesSubseconds, LossyConversionAuthorityValid, NormalizeToUtcEstablishedToken,
    NormalizeToUtcPreconditionsToken, OffsetConsistentWithNamedZone, OffsetDateTimeDescriptor,
    OffsetDateTimeValid, PrecisionDescriptor, StripNamedZoneEstablishedToken,
    StripNamedZonePreconditionsToken, TimestampRepresentsFixedInstant,
    TruncateSubsecondsEstablishedToken, TruncateSubsecondsPreconditionsToken,
    ZonedDateTimeDescriptor, ZonedDateTimeHasNamedZone,
};

/// Descriptors the `normalize_to_utc` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
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
#[derive(Debug, Clone, amenable_derive::Sidecar)]
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

impl NormalizeToUtcInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &NormalizeToUtcRequest {
        &self.request
    }
}

/// Output sidecar for the `normalize_to_utc` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`NormalizeToUtcEstablished`](crate::NormalizeToUtcEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::NormalizeToUtcEstablished", constructor = "pub")]
pub struct NormalizeToUtcOutput {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: NormalizeToUtcEstablishedToken,
}

impl NormalizeToUtcOutput {
    /// Borrow the emitted descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OffsetDateTimeDescriptor {
        &self.descriptor
    }
}

/// Descriptors the `strip_named_zone` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
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
#[derive(Debug, Clone, amenable_derive::Sidecar)]
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

impl StripNamedZoneInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &StripNamedZoneRequest {
        &self.request
    }
}

/// Output sidecar for the `strip_named_zone` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`StripNamedZoneEstablished`](crate::StripNamedZoneEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::StripNamedZoneEstablished", constructor = "pub")]
pub struct StripNamedZoneOutput {
    #[sidecar(primary)]
    descriptor: OffsetDateTimeDescriptor,
    #[sidecar(token)]
    token: StripNamedZoneEstablishedToken,
}

impl StripNamedZoneOutput {
    /// Borrow the emitted descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OffsetDateTimeDescriptor {
        &self.descriptor
    }
}

/// Descriptors the `adjust_precision_losslessly` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
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
#[derive(Debug, Clone, amenable_derive::Sidecar)]
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

impl AdjustPrecisionLosslesslyInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &AdjustPrecisionLosslesslyRequest {
        &self.request
    }
}

/// Output sidecar for the `adjust_precision_losslessly` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`AdjustPrecisionLosslesslyEstablished`](crate::AdjustPrecisionLosslesslyEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
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

impl AdjustPrecisionLosslesslyOutput {
    /// Borrow the emitted descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OffsetDateTimeDescriptor {
        &self.descriptor
    }
}

/// Descriptors the `truncate_subseconds` exchange consumes.
#[derive(
    Debug, Clone, Default, PartialEq, Eq, Hash, amenable_derive::Evidence, derive_getters::Getters,
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
#[derive(Debug, Clone, amenable_derive::Sidecar)]
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

impl TruncateSubsecondsInput {
    /// Borrow the request descriptors.
    #[must_use]
    pub fn request(&self) -> &TruncateSubsecondsRequest {
        &self.request
    }
}

/// Output sidecar for the `truncate_subseconds` exchange: the
/// [`OffsetDateTimeDescriptor`](crate::OffsetDateTimeDescriptor) it emits plus a
/// token for the folded [`TruncateSubsecondsEstablished`](crate::TruncateSubsecondsEstablished).
#[derive(Debug, Clone, amenable_derive::Sidecar)]
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

impl TruncateSubsecondsOutput {
    /// Borrow the emitted descriptor.
    #[must_use]
    pub fn descriptor(&self) -> &OffsetDateTimeDescriptor {
        &self.descriptor
    }
}
