//! Normalize-to-UTC and adjust-precision-losslessly native-carrier exchange types.
//!
//! Phase 5 Step 4b -- the `*_native` factory analogs' exchange types.
//! Carrier<->carrier: the native-carrier versions of the
//! descriptor-side factory transitions. Multi-input methods fold their
//! runtime values into a `<M>NativeRequest<B>` primary (basis = the
//! [`NativeCarrierRequest`] marker, so the compound needs no `Default`);
//! methods with an extra output proof fold it into a `<M>NativeEstablished`
//! composite behind a `<M>NativeToken`.

use derive_new::new;

use crate::{
    LosslessConversionBundle, OffsetDateTimeSemanticBundle, PrecisionDescriptor,
    TemporalInputToken, TemporalInstantProps, TemporalZoneProps,
};

/// Abstract basis for a folded native-carrier request compound — the
/// fact "a native-carrier request was assembled", not a root claim.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, amenable_derive::Evidence)]
#[evidence(basis = "Self")]
pub struct NativeCarrierRequest;
/// Proofs the `normalize_to_utc_native` exchange re-issues, folded into one proposition.
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
pub struct NormalizeToUtcNativeEstablished {
    /// The re-issued `offset_date_time_semantic_bundle` sub-claim.
    offset_date_time_semantic_bundle: OffsetDateTimeSemanticBundle,
    /// The re-issued `lossless_conversion_bundle` sub-claim.
    lossless_conversion_bundle: LosslessConversionBundle,
}
/// Lawful token for the folded [`NormalizeToUtcNativeEstablished`](crate::NormalizeToUtcNativeEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::NormalizeToUtcNativeEstablished")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::NormalizeToUtcNativeEstablished"
)]
pub struct NormalizeToUtcNativeToken(());
/// Output sidecar for the `normalize_to_utc_native` exchange: the emitted
/// native `OffsetDateTime` carrier + a [`NormalizeToUtcNativeEstablished`](crate::NormalizeToUtcNativeEstablished) token.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::NormalizeToUtcNativeEstablished",
    constructor = "pub"
)]
pub struct NormalizeToUtcNativeOutput<B: TemporalInstantProps + TemporalZoneProps> {
    #[sidecar(primary)]
    carrier: B::OffsetDateTime,
    #[sidecar(token)]
    token: NormalizeToUtcNativeToken,
}
/// Runtime values the `adjust_precision_losslessly_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters, new)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct AdjustPrecisionLosslesslyNativeRequest<B: TemporalInstantProps + TemporalZoneProps> {
    /// The `timestamp` input.
    timestamp: B::OffsetDateTime,
    /// The `target_precision` input.
    target_precision: PrecisionDescriptor,
}
/// Input sidecar for the `adjust_precision_losslessly_native` exchange.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct AdjustPrecisionLosslesslyNativeInput<B: TemporalInstantProps + TemporalZoneProps> {
    #[sidecar(primary)]
    request: AdjustPrecisionLosslesslyNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}
/// Proofs the `adjust_precision_losslessly_native` exchange re-issues, folded into one proposition.
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
pub struct AdjustPrecisionLosslesslyNativeEstablished {
    /// The re-issued `offset_date_time_semantic_bundle` sub-claim.
    offset_date_time_semantic_bundle: OffsetDateTimeSemanticBundle,
    /// The re-issued `lossless_conversion_bundle` sub-claim.
    lossless_conversion_bundle: LosslessConversionBundle,
}
/// Lawful token for the folded [`AdjustPrecisionLosslesslyNativeEstablished`](crate::AdjustPrecisionLosslesslyNativeEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::AdjustPrecisionLosslesslyNativeEstablished")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::AdjustPrecisionLosslesslyNativeEstablished"
)]
pub struct AdjustPrecisionLosslesslyNativeToken(());
/// Output sidecar for the `adjust_precision_losslessly_native` exchange: the emitted
/// native `OffsetDateTime` carrier + a [`AdjustPrecisionLosslesslyNativeEstablished`](crate::AdjustPrecisionLosslesslyNativeEstablished) token.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::AdjustPrecisionLosslesslyNativeEstablished",
    constructor = "pub"
)]
pub struct AdjustPrecisionLosslesslyNativeOutput<B: TemporalInstantProps + TemporalZoneProps> {
    #[sidecar(primary)]
    carrier: B::OffsetDateTime,
    #[sidecar(token)]
    token: AdjustPrecisionLosslesslyNativeToken,
}
