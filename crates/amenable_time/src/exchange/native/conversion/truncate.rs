//! Truncate-subseconds native-carrier exchange types.
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
    LossyConversionAuthorityBundle, OffsetDateTimeSemanticBundle, PrecisionDescriptor,
    SubsecondTruncationBundle, TemporalInputToken, TemporalInstantProps, TemporalZoneProps,
};

/// Runtime values the `truncate_subseconds_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters, new)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct TruncateSubsecondsNativeRequest<B: TemporalInstantProps + TemporalZoneProps> {
    /// The `timestamp` input.
    timestamp: B::OffsetDateTime,
    /// The `target_precision` input.
    target_precision: PrecisionDescriptor,
    /// The `lossy_authority` input.
    lossy_authority: LossyConversionAuthorityBundle,
}
/// Input sidecar for the `truncate_subseconds_native` exchange.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct TruncateSubsecondsNativeInput<B: TemporalInstantProps + TemporalZoneProps> {
    #[sidecar(primary)]
    request: TruncateSubsecondsNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}
/// Proofs the `truncate_subseconds_native` exchange re-issues, folded into one proposition.
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
pub struct TruncateSubsecondsNativeEstablished {
    /// The re-issued `offset_date_time_semantic_bundle` sub-claim.
    offset_date_time_semantic_bundle: OffsetDateTimeSemanticBundle,
    /// The re-issued `subsecond_truncation_bundle` sub-claim.
    subsecond_truncation_bundle: SubsecondTruncationBundle,
}
/// Lawful token for the folded [`TruncateSubsecondsNativeEstablished`](crate::TruncateSubsecondsNativeEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::TruncateSubsecondsNativeEstablished")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::TruncateSubsecondsNativeEstablished"
)]
pub struct TruncateSubsecondsNativeToken(());
/// Output sidecar for the `truncate_subseconds_native` exchange: the emitted
/// native `OffsetDateTime` carrier + a [`TruncateSubsecondsNativeEstablished`](crate::TruncateSubsecondsNativeEstablished) token.
#[derive(amenable_derive::Sidecar, derive_getters::Getters)]
#[sidecar(
    proposition = "crate::TruncateSubsecondsNativeEstablished",
    constructor = "pub"
)]
pub struct TruncateSubsecondsNativeOutput<B: TemporalInstantProps + TemporalZoneProps> {
    #[sidecar(primary)]
    carrier: B::OffsetDateTime,
    #[sidecar(token)]
    token: TruncateSubsecondsNativeToken,
}
