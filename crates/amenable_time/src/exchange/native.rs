//! Phase 5 Step 4b — the `*_native` factory analogs' exchange types.
//! Carrier<->carrier: the native-carrier versions of the
//! descriptor-side factory transitions. Multi-input methods fold their
//! runtime values into a `<M>NativeRequest<B>` primary (basis = the
//! [`NativeCarrierRequest`] marker, so the compound needs no `Default`);
//! methods with an extra output proof fold it into a `<M>NativeEstablished`
//! composite behind a `<M>NativeToken`.

use crate::{
    DateTimeFormulaEvaluationResultBundle, ExplicitTemporalFormSemanticBundle,
    IntervalEndpointOrderingBundle, IntervalEndpointOrderingBundleToken,
    LocalTimeZoneResolutionAuthorityDescriptor, LocalTimeZoneResolutionProofBranch,
    LosslessConversionBundle, LossyConversionAuthorityBundle, NamedTimeZoneRevisionBundle,
    NamedTimeZoneRevisionBundleToken, OffsetDateTimeSemanticBundle, PrecisionDescriptor,
    SubsecondTruncationBundle, TemporalCivilProps, TemporalDateTimeFormulaProps,
    TemporalExplicitTemporalFormProps, TemporalInputToken, TemporalInstantProps, TemporalZoneProps,
    ZoneTransitionResolutionAuthorityBundle, ZonedDateTimeSemanticBundle,
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
#[derive(amenable_derive::Sidecar)]
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

impl<B: TemporalInstantProps + TemporalZoneProps> NormalizeToUtcNativeOutput<B> {
    /// Borrow the emitted native carrier.
    #[must_use]
    pub fn carrier(&self) -> &B::OffsetDateTime {
        &self.carrier
    }
}

/// Runtime values the `adjust_precision_losslessly_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct AdjustPrecisionLosslesslyNativeRequest<B: TemporalInstantProps + TemporalZoneProps> {
    /// The `timestamp` input.
    timestamp: B::OffsetDateTime,
    /// The `target_precision` input.
    target_precision: PrecisionDescriptor,
}

/// Input sidecar for the `adjust_precision_losslessly_native` exchange.
#[derive(amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct AdjustPrecisionLosslesslyNativeInput<B: TemporalInstantProps + TemporalZoneProps> {
    #[sidecar(primary)]
    request: AdjustPrecisionLosslesslyNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}

impl<B: TemporalInstantProps + TemporalZoneProps> AdjustPrecisionLosslesslyNativeInput<B> {
    /// Borrow the request values.
    #[must_use]
    pub fn request(&self) -> &AdjustPrecisionLosslesslyNativeRequest<B> {
        &self.request
    }
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
#[derive(amenable_derive::Sidecar)]
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

impl<B: TemporalInstantProps + TemporalZoneProps> AdjustPrecisionLosslesslyNativeOutput<B> {
    /// Borrow the emitted native carrier.
    #[must_use]
    pub fn carrier(&self) -> &B::OffsetDateTime {
        &self.carrier
    }
}

/// Runtime values the `truncate_subseconds_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters)]
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
#[derive(amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct TruncateSubsecondsNativeInput<B: TemporalInstantProps + TemporalZoneProps> {
    #[sidecar(primary)]
    request: TruncateSubsecondsNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}

impl<B: TemporalInstantProps + TemporalZoneProps> TruncateSubsecondsNativeInput<B> {
    /// Borrow the request values.
    #[must_use]
    pub fn request(&self) -> &TruncateSubsecondsNativeRequest<B> {
        &self.request
    }
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
#[derive(amenable_derive::Sidecar)]
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

impl<B: TemporalInstantProps + TemporalZoneProps> TruncateSubsecondsNativeOutput<B> {
    /// Borrow the emitted native carrier.
    #[must_use]
    pub fn carrier(&self) -> &B::OffsetDateTime {
        &self.carrier
    }
}

/// Runtime values the `resolve_local_date_time_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct ResolveLocalDateTimeNativeRequest<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    /// The `timestamp` input.
    timestamp: B::LocalDateTime,
    /// The `zone` input.
    zone: B::NamedTimeZone,
    /// The `resolution_authority` input.
    resolution_authority: LocalTimeZoneResolutionAuthorityDescriptor,
    /// The `authority` input.
    authority: ZoneTransitionResolutionAuthorityBundle,
}

/// Input sidecar for the `resolve_local_date_time_native` exchange.
#[derive(amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct ResolveLocalDateTimeNativeInput<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    #[sidecar(primary)]
    request: ResolveLocalDateTimeNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}

impl<B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps>
    ResolveLocalDateTimeNativeInput<B>
{
    /// Borrow the request values.
    #[must_use]
    pub fn request(&self) -> &ResolveLocalDateTimeNativeRequest<B> {
        &self.request
    }
}

/// Proofs the `resolve_local_date_time_native` exchange re-issues, folded into one proposition.
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
pub struct ResolveLocalDateTimeNativeEstablished {
    /// The re-issued `zoned_date_time_semantic_bundle` sub-claim.
    zoned_date_time_semantic_bundle: ZonedDateTimeSemanticBundle,
    /// The re-issued `zone_transition_resolution_authority_bundle` sub-claim.
    zone_transition_resolution_authority_bundle: ZoneTransitionResolutionAuthorityBundle,
    /// The re-issued `local_time_zone_resolution_proof_branch` sub-claim.
    local_time_zone_resolution_proof_branch: LocalTimeZoneResolutionProofBranch,
}

/// Lawful token for the folded [`ResolveLocalDateTimeNativeEstablished`](crate::ResolveLocalDateTimeNativeEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::ResolveLocalDateTimeNativeEstablished")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::ResolveLocalDateTimeNativeEstablished"
)]
pub struct ResolveLocalDateTimeNativeToken(());

/// Output sidecar for the `resolve_local_date_time_native` exchange: the emitted
/// native `ZonedDateTime` carrier + a [`ResolveLocalDateTimeNativeEstablished`](crate::ResolveLocalDateTimeNativeEstablished) token.
#[derive(amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::ResolveLocalDateTimeNativeEstablished",
    constructor = "pub"
)]
pub struct ResolveLocalDateTimeNativeOutput<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    #[sidecar(primary)]
    carrier: B::ZonedDateTime,
    #[sidecar(token)]
    token: ResolveLocalDateTimeNativeToken,
}

impl<B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps>
    ResolveLocalDateTimeNativeOutput<B>
{
    /// Borrow the emitted native carrier.
    #[must_use]
    pub fn carrier(&self) -> &B::ZonedDateTime {
        &self.carrier
    }
}

/// Runtime values the `attach_named_zone_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct AttachNamedZoneNativeRequest<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    /// The `timestamp` input.
    timestamp: B::OffsetDateTime,
    /// The `zone` input.
    zone: B::NamedTimeZone,
}

/// Input sidecar for the `attach_named_zone_native` exchange.
#[derive(amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct AttachNamedZoneNativeInput<
    B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps,
> {
    #[sidecar(primary)]
    request: AttachNamedZoneNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}

impl<B: TemporalCivilProps + TemporalInstantProps + TemporalZoneProps>
    AttachNamedZoneNativeInput<B>
{
    /// Borrow the request values.
    #[must_use]
    pub fn request(&self) -> &AttachNamedZoneNativeRequest<B> {
        &self.request
    }
}

/// Output sidecar for the `confirm_named_zone_revision_native` exchange — pure proof,
/// the [`NamedTimeZoneRevisionBundle`](crate::NamedTimeZoneRevisionBundle) it re-issues + its token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::NamedTimeZoneRevisionBundle",
    constructor = "pub"
)]
pub struct ConfirmNamedZoneRevisionNativeOutput {
    #[sidecar(primary)]
    bundle: NamedTimeZoneRevisionBundle,
    #[sidecar(token)]
    token: NamedTimeZoneRevisionBundleToken,
}

impl ConfirmNamedZoneRevisionNativeOutput {
    /// Borrow the re-issued proof bundle.
    #[must_use]
    pub fn bundle(&self) -> &NamedTimeZoneRevisionBundle {
        &self.bundle
    }
}

/// Runtime values the `order_offset_endpoints_native` exchange consumes.
#[derive(amenable_derive::Evidence, derive_getters::Getters)]
#[evidence(basis = "crate::NativeCarrierRequest")]
pub struct OrderOffsetEndpointsNativeRequest<B: TemporalInstantProps> {
    /// The `start` input.
    start: B::OffsetDateTime,
    /// The `end` input.
    end: B::OffsetDateTime,
}

/// Input sidecar for the `order_offset_endpoints_native` exchange.
#[derive(amenable_derive::Sidecar)]
#[sidecar(proposition = "crate::TemporalInputReceived", constructor = "pub")]
pub struct OrderOffsetEndpointsNativeInput<B: TemporalInstantProps> {
    #[sidecar(primary)]
    request: OrderOffsetEndpointsNativeRequest<B>,
    #[sidecar(token)]
    token: TemporalInputToken,
}

impl<B: TemporalInstantProps> OrderOffsetEndpointsNativeInput<B> {
    /// Borrow the request values.
    #[must_use]
    pub fn request(&self) -> &OrderOffsetEndpointsNativeRequest<B> {
        &self.request
    }
}

/// Output sidecar for the `order_offset_endpoints_native` exchange — pure proof,
/// the [`IntervalEndpointOrderingBundle`](crate::IntervalEndpointOrderingBundle) it re-issues + its token.
#[derive(Debug, Clone, amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::IntervalEndpointOrderingBundle",
    constructor = "pub"
)]
pub struct OrderOffsetEndpointsNativeOutput {
    #[sidecar(primary)]
    bundle: IntervalEndpointOrderingBundle,
    #[sidecar(token)]
    token: IntervalEndpointOrderingBundleToken,
}

impl OrderOffsetEndpointsNativeOutput {
    /// Borrow the re-issued proof bundle.
    #[must_use]
    pub fn bundle(&self) -> &IntervalEndpointOrderingBundle {
        &self.bundle
    }
}

/// Proofs the `evaluate_date_time_formula_native` exchange re-issues, folded into one proposition.
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
pub struct EvaluateDateTimeFormulaNativeEstablished {
    /// The re-issued `explicit_temporal_form_semantic_bundle` sub-claim.
    explicit_temporal_form_semantic_bundle: ExplicitTemporalFormSemanticBundle,
    /// The re-issued `date_time_formula_evaluation_result_bundle` sub-claim.
    date_time_formula_evaluation_result_bundle: DateTimeFormulaEvaluationResultBundle,
}

/// Lawful token for the folded [`EvaluateDateTimeFormulaNativeEstablished`](crate::EvaluateDateTimeFormulaNativeEstablished).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, amenable_derive::ProofToken)]
#[proof_token(proposition = "crate::EvaluateDateTimeFormulaNativeEstablished")]
#[amenable_derive::establish(
    credential = "crate::TemporalInputToken",
    proposition = "crate::EvaluateDateTimeFormulaNativeEstablished"
)]
pub struct EvaluateDateTimeFormulaNativeToken(());

/// Output sidecar for the `evaluate_date_time_formula_native` exchange: the emitted
/// native `ExplicitTemporalForm` carrier + a [`EvaluateDateTimeFormulaNativeEstablished`](crate::EvaluateDateTimeFormulaNativeEstablished) token.
#[derive(amenable_derive::Sidecar)]
#[sidecar(
    proposition = "crate::EvaluateDateTimeFormulaNativeEstablished",
    constructor = "pub"
)]
pub struct EvaluateDateTimeFormulaNativeOutput<
    B: TemporalDateTimeFormulaProps + TemporalExplicitTemporalFormProps,
> {
    #[sidecar(primary)]
    carrier: B::ExplicitTemporalForm,
    #[sidecar(token)]
    token: EvaluateDateTimeFormulaNativeToken,
}

impl<B: TemporalDateTimeFormulaProps + TemporalExplicitTemporalFormProps>
    EvaluateDateTimeFormulaNativeOutput<B>
{
    /// Borrow the emitted native carrier.
    #[must_use]
    pub fn carrier(&self) -> &B::ExplicitTemporalForm {
        &self.carrier
    }
}
