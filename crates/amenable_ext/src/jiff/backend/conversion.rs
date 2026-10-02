use super::instant::{
    JiffOffsetDateTime, jiff_parts_to_offset_date_time_descriptor,
    offset_date_time_descriptor_to_jiff_parts,
};
use super::zone_conversions::zoned_date_time_descriptor_to_jiff_zoned;
use crate::{JiffTimeBackend, JiffVerifier, JiffZoned};
use amenable_core::{Establish, Exchange, Sidecar};
use amenable_time::{
    AdjustPrecisionLosslesslyEstablished, AdjustPrecisionLosslesslyInput,
    AdjustPrecisionLosslesslyNativeEstablished, AdjustPrecisionLosslesslyNativeInput,
    AdjustPrecisionLosslesslyNativeOutput, AdjustPrecisionLosslesslyOutput,
    AdjustPrecisionLosslesslyPreconditionsToken, InvalidDescriptorSource,
    NormalizeToUtcEstablished, NormalizeToUtcInput, NormalizeToUtcNativeEstablished,
    NormalizeToUtcNativeOutput, NormalizeToUtcOutput, NormalizeToUtcPreconditionsToken,
    OffsetDateTimeDescriptor, OffsetDateTimeProof, OffsetDateTimeProofToken,
    OffsetDateTimeSemanticBundle, PrecisionDescriptor, ProvenOffsetDateTimeCarrier,
    ProvenZonedDateTimeCarrier, RoundingModeDescriptor, StripNamedZoneEstablished,
    StripNamedZoneInput, StripNamedZoneOutput, StripNamedZonePreconditionsToken, TemporalComponent,
    TemporalError, TemporalErrorKind, TemporalInputToken, TruncateSubsecondsEstablished,
    TruncateSubsecondsInput, TruncateSubsecondsNativeEstablished, TruncateSubsecondsNativeInput,
    TruncateSubsecondsNativeOutput, TruncateSubsecondsOutput, TruncateSubsecondsPreconditionsToken,
    UnsupportedSource,
};

// ── Conversion factory (Phase 5) ────────────────────────────────────
//
// `TemporalConversionFactory<JiffVerifier>` plus its native counterpart:
// UTC normalization, named-zone stripping (reusing Phase 4b's own
// consistency-checked `zoned_date_time_descriptor_to_jiff_zoned`), and
// sub-second precision reduction. All four are real jiff arithmetic,
// not stand-ins.

/// Validate that `target` anchors precision at the second and return
/// its declared fractional-digit count (`0` when absent).
///
/// jiff's civil time is always second-plus-nanosecond in shape --
/// there is no jiff concept of a "smallest component" coarser than a
/// second for THIS pair of edges (that belongs to the civil/ordinal
/// round trip built in Phase 3), so any `smallest_component` other
/// than `Second` is a real, honest `Unsupported` case here, not a
/// silent no-op.
#[cfg_attr(not(kani), tracing::instrument(level = "debug", skip(target)))]
fn require_second_precision(target: &PrecisionDescriptor) -> Result<u8, TemporalError> {
    if target.smallest_component() != TemporalComponent::Second {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(format!(
                "jiff backend's precision-adjustment edges anchor at the second; \
             smallest_component = {} is not supported",
                target.smallest_component(),
            )),
        )));
    }
    let digits = target.fractional_digits().unwrap_or(0);
    if digits > 9 {
        return Err(TemporalError::new(TemporalErrorKind::Unsupported(
            UnsupportedSource::new(format!(
                "jiff's civil time is nanosecond-precision (9 fractional digits); \
             {digits} fractional digits exceeds its representable range"
            )),
        )));
    }
    Ok(digits)
}

/// Zero out every nanosecond digit beyond `digits` (0..=9).
#[cfg_attr(not(kani), tracing::instrument(level = "trace"))]
fn truncate_nanos_to_digits(nanos: i32, digits: u8) -> i32 {
    if digits >= 9 {
        return nanos;
    }
    let scale = 10i32.pow(9 - u32::from(digits));
    (nanos / scale) * scale
}

/// Adjust `local`'s sub-second precision to `target_precision`,
/// honoring `lossless` (reject any real information loss) vs. lossy
/// (require the declared rounding mode to be `Truncate`, since that is
/// the only rounding arithmetic this backend implements -- a non-
/// `Truncate` rounding mode is a real, honest `Unsupported` case, not
/// silently ignored or approximated).
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(local, target_precision))
)]
fn adjust_jiff_local_precision(
    local: jiff::civil::DateTime,
    target_precision: &PrecisionDescriptor,
    lossless: bool,
) -> Result<jiff::civil::DateTime, TemporalError> {
    let digits = require_second_precision(target_precision)?;
    let original_nanos = local.subsec_nanosecond();
    let truncated_nanos = truncate_nanos_to_digits(original_nanos, digits);
    if truncated_nanos != original_nanos {
        if lossless {
            return Err(TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "adjusting to {digits} fractional digits would discard non-zero \
                 sub-second precision ({original_nanos} nanoseconds); not lossless"
                )),
            )));
        }
        match target_precision.rounding_mode() {
            None | Some(RoundingModeDescriptor::Truncate) => {}
            Some(other) => {
                return Err(TemporalError::new(TemporalErrorKind::Unsupported(
                    UnsupportedSource::new(format!(
                        "jiff backend only implements truncation for sub-second \
                     precision reduction, not rounding mode {other}"
                    )),
                )));
            }
        }
    }
    local
        .with()
        .subsec_nanosecond(truncated_nanos)
        .build()
        .map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not rebuild the precision-adjusted local time: {err}"
                )),
            ))
        })
}

/// Descriptor-level wrapper over [`adjust_jiff_local_precision`].
#[cfg_attr(
    not(kani),
    tracing::instrument(level = "debug", skip(descriptor, target_precision))
)]
fn adjust_offset_date_time_precision(
    descriptor: &OffsetDateTimeDescriptor,
    target_precision: &PrecisionDescriptor,
    lossless: bool,
) -> Result<OffsetDateTimeDescriptor, TemporalError> {
    let (local, offset) = offset_date_time_descriptor_to_jiff_parts(descriptor)?;
    let adjusted_local = adjust_jiff_local_precision(local, target_precision, lossless)?;
    jiff_parts_to_offset_date_time_descriptor(adjusted_local, offset)
}

impl Exchange<NormalizeToUtcInput, NormalizeToUtcOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: NormalizeToUtcInput) -> Result<NormalizeToUtcOutput, TemporalError> {
        let (local, offset) =
            offset_date_time_descriptor_to_jiff_parts(input.request().timestamp())?;
        let timestamp = offset.to_timestamp(local).map_err(|err| {
            TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                InvalidDescriptorSource::new(format!(
                    "could not pin the offset date-time to a fixed instant: {err}"
                )),
            ))
        })?;
        let utc = timestamp.to_zoned(jiff::tz::TimeZone::UTC);
        let descriptor = jiff_parts_to_offset_date_time_descriptor(utc.datetime(), utc.offset())?;
        let input_token = <NormalizeToUtcInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <NormalizeToUtcEstablished as Establish<
            NormalizeToUtcPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(NormalizeToUtcOutput::new(descriptor, token))
    }
}

impl Exchange<StripNamedZoneInput, StripNamedZoneOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(&self, input: StripNamedZoneInput) -> Result<StripNamedZoneOutput, TemporalError> {
        let zoned = zoned_date_time_descriptor_to_jiff_zoned(input.request().timestamp())?;
        let descriptor =
            jiff_parts_to_offset_date_time_descriptor(zoned.datetime(), zoned.offset())?;
        let input_token = <StripNamedZoneInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <StripNamedZoneEstablished as Establish<
            StripNamedZonePreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(StripNamedZoneOutput::new(descriptor, token))
    }
}

impl Exchange<AdjustPrecisionLosslesslyInput, AdjustPrecisionLosslesslyOutput, JiffVerifier>
    for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AdjustPrecisionLosslesslyInput,
    ) -> Result<AdjustPrecisionLosslesslyOutput, TemporalError> {
        let request = input.request();
        let descriptor = adjust_offset_date_time_precision(
            request.timestamp(),
            request.target_precision(),
            true,
        )?;
        let input_token =
            <AdjustPrecisionLosslesslyInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <AdjustPrecisionLosslesslyEstablished as Establish<
            AdjustPrecisionLosslesslyPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(AdjustPrecisionLosslesslyOutput::new(descriptor, token))
    }
}

impl Exchange<TruncateSubsecondsInput, TruncateSubsecondsOutput, JiffVerifier> for JiffTimeBackend {
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: TruncateSubsecondsInput,
    ) -> Result<TruncateSubsecondsOutput, TemporalError> {
        let request = input.request();
        let descriptor = adjust_offset_date_time_precision(
            request.timestamp(),
            request.target_precision(),
            false,
        )?;
        let input_token = <TruncateSubsecondsInput as Sidecar<JiffVerifier>>::sidecar(&input);
        let token = <TruncateSubsecondsEstablished as Establish<
            TruncateSubsecondsPreconditionsToken,
            JiffVerifier,
        >>::establish(input_token);
        Ok(TruncateSubsecondsOutput::new(descriptor, token))
    }
}

// ── Native conversion factory (Phase 5) ─────────────────────────────
//
// The native mirror of the four edges above, operating directly on
// `JiffOffsetDateTime`/`JiffZoned` carriers instead of descriptors.
// `strip_named_zone_native` has no separate output wrapper -- its own
// trait bound (`TemporalNativeConversionFactory`) returns
// `ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>` directly, since
// the emitted carrier's own `OffsetDateTimeSemanticBundleToken` is
// already the right shape.

impl
    Exchange<
        ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
        NormalizeToUtcNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
    ) -> Result<NormalizeToUtcNativeOutput<JiffTimeBackend>, TemporalError> {
        let carrier = input.carrier();
        let timestamp = carrier
            .offset()
            .to_timestamp(*carrier.local())
            .map_err(|err| {
                TemporalError::new(TemporalErrorKind::InvalidDescriptor(
                    InvalidDescriptorSource::new(format!(
                        "could not pin the offset date-time to a fixed instant: {err}"
                    )),
                ))
            })?;
        let utc = timestamp.to_zoned(jiff::tz::TimeZone::UTC);
        let native = JiffOffsetDateTime::new(utc.datetime(), utc.offset());
        // This edge's own input carries an `OffsetDateTimeSemanticBundleToken`,
        // a different proposition than `NormalizeToUtcNativeEstablished`'s own
        // single-hop `TemporalInputToken` credential -- so a fresh root
        // credential is synthesized here, the same way Phase 4b's own
        // `ConfirmNamedZoneRevisionNativeOutput` edge already does.
        let token = <NormalizeToUtcNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        Ok(NormalizeToUtcNativeOutput::<JiffTimeBackend>::new(
            native, token,
        ))
    }
}

impl
    Exchange<
        ProvenZonedDateTimeCarrier<JiffZoned>,
        ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: ProvenZonedDateTimeCarrier<JiffZoned>,
    ) -> Result<ProvenOffsetDateTimeCarrier<JiffOffsetDateTime>, TemporalError> {
        let zoned = input.carrier();
        let native = JiffOffsetDateTime::new(zoned.datetime(), zoned.offset());
        // Fresh two-hop chain (`TemporalInputToken` -> `OffsetDateTimeProof`
        // -> `OffsetDateTimeSemanticBundle`): the input's own
        // `ZonedDateTimeSemanticBundleToken` proves a different proposition
        // than the output's `OffsetDateTimeSemanticBundleToken`.
        let proof_token =
            <OffsetDateTimeProof as Establish<TemporalInputToken, JiffVerifier>>::establish(
                TemporalInputToken::new(),
            );
        let bundle_token = <OffsetDateTimeSemanticBundle as Establish<
            OffsetDateTimeProofToken,
            JiffVerifier,
        >>::establish(proof_token);
        Ok(ProvenOffsetDateTimeCarrier::<JiffOffsetDateTime>::new(
            native,
            bundle_token,
        ))
    }
}

impl
    Exchange<
        AdjustPrecisionLosslesslyNativeInput<JiffTimeBackend>,
        AdjustPrecisionLosslesslyNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: AdjustPrecisionLosslesslyNativeInput<JiffTimeBackend>,
    ) -> Result<AdjustPrecisionLosslesslyNativeOutput<JiffTimeBackend>, TemporalError> {
        let request = input.request();
        let adjusted_local = adjust_jiff_local_precision(
            *request.timestamp().local(),
            request.target_precision(),
            true,
        )?;
        let native = JiffOffsetDateTime::new(adjusted_local, *request.timestamp().offset());
        let token = <AdjustPrecisionLosslesslyNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        Ok(AdjustPrecisionLosslesslyNativeOutput::<JiffTimeBackend>::new(native, token))
    }
}

impl
    Exchange<
        TruncateSubsecondsNativeInput<JiffTimeBackend>,
        TruncateSubsecondsNativeOutput<JiffTimeBackend>,
        JiffVerifier,
    > for JiffTimeBackend
{
    type Error = TemporalError;

    #[cfg_attr(not(kani), tracing::instrument(level = "trace", skip(self, input)))]
    fn exchange(
        &self,
        input: TruncateSubsecondsNativeInput<JiffTimeBackend>,
    ) -> Result<TruncateSubsecondsNativeOutput<JiffTimeBackend>, TemporalError> {
        let request = input.request();
        let adjusted_local = adjust_jiff_local_precision(
            *request.timestamp().local(),
            request.target_precision(),
            false,
        )?;
        let native = JiffOffsetDateTime::new(adjusted_local, *request.timestamp().offset());
        let token = <TruncateSubsecondsNativeEstablished as Establish<
            TemporalInputToken,
            JiffVerifier,
        >>::establish(TemporalInputToken::new());
        Ok(TruncateSubsecondsNativeOutput::<JiffTimeBackend>::new(
            native, token,
        ))
    }
}
