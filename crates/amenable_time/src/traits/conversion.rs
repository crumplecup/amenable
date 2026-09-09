//! [`TemporalConversionFactory`] — the conversion half of the temporal seam,
//! ported from `elicit_temporal::traits::TemporalConversionFactory`. Same shape
//! as [`TemporalParser`](crate::TemporalParser): the supertrait bundle
//! *is* the conversion exchanges. The `where <Prop>: Witness<V>` bounds —
//! one per distinct precondition *and* re-issued proposition — are the
//! honest precondition, backend-provided.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    AdjustPrecisionLosslesslyEstablished, AdjustPrecisionLosslesslyInput,
    AdjustPrecisionLosslesslyOutput, AdjustPrecisionLosslesslyPreconditions,
    NormalizeToUtcEstablished, NormalizeToUtcInput, NormalizeToUtcOutput,
    NormalizeToUtcPreconditions, StripNamedZoneEstablished, StripNamedZoneInput,
    StripNamedZoneOutput, StripNamedZonePreconditions, TemporalError,
    TruncateSubsecondsEstablished, TruncateSubsecondsInput, TruncateSubsecondsOutput,
    TruncateSubsecondsPreconditions,
};

/// A backend that performs every conversion transition as an
/// [`Exchange`], for verifier `V`.
pub trait TemporalConversionFactory<V: Verifier>:
    Send
    + Sync
    + Exchange<NormalizeToUtcInput, NormalizeToUtcOutput, V, Error = TemporalError>
    + Exchange<StripNamedZoneInput, StripNamedZoneOutput, V, Error = TemporalError>
    + Exchange<
        AdjustPrecisionLosslesslyInput,
        AdjustPrecisionLosslesslyOutput,
        V,
        Error = TemporalError,
    > + Exchange<TruncateSubsecondsInput, TruncateSubsecondsOutput, V, Error = TemporalError>
where
    NormalizeToUtcPreconditions: Witness<V>,
    NormalizeToUtcEstablished: Witness<V>,
    StripNamedZonePreconditions: Witness<V>,
    StripNamedZoneEstablished: Witness<V>,
    AdjustPrecisionLosslesslyPreconditions: Witness<V>,
    AdjustPrecisionLosslesslyEstablished: Witness<V>,
    TruncateSubsecondsPreconditions: Witness<V>,
    TruncateSubsecondsEstablished: Witness<V>,
{
}

impl<T, V> TemporalConversionFactory<V> for T
where
    V: Verifier,
    NormalizeToUtcPreconditions: Witness<V>,
    NormalizeToUtcEstablished: Witness<V>,
    StripNamedZonePreconditions: Witness<V>,
    StripNamedZoneEstablished: Witness<V>,
    AdjustPrecisionLosslesslyPreconditions: Witness<V>,
    AdjustPrecisionLosslesslyEstablished: Witness<V>,
    TruncateSubsecondsPreconditions: Witness<V>,
    TruncateSubsecondsEstablished: Witness<V>,
    T: Send
        + Sync
        + Exchange<NormalizeToUtcInput, NormalizeToUtcOutput, V, Error = TemporalError>
        + Exchange<StripNamedZoneInput, StripNamedZoneOutput, V, Error = TemporalError>
        + Exchange<
            AdjustPrecisionLosslesslyInput,
            AdjustPrecisionLosslesslyOutput,
            V,
            Error = TemporalError,
        > + Exchange<TruncateSubsecondsInput, TruncateSubsecondsOutput, V, Error = TemporalError>,
{
}
