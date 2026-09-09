//! [`TemporalIntervalFactory`] — the interval half of the temporal seam,
//! ported from `elicit_temporal::traits::TemporalIntervalFactory`. Same shape
//! as [`TemporalParser`](crate::TemporalParser): the supertrait bundle
//! *is* the interval exchanges. The `where <Prop>: Witness<V>` bounds —
//! one per distinct precondition *and* re-issued proposition — are the
//! honest precondition, backend-provided.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    DurationFormValid, OrderOffsetEndpointsEstablished, OrderOffsetEndpointsInput,
    OrderOffsetEndpointsOutput, OrderOffsetEndpointsPreconditions, ParsedDuration,
    ParsedRecurringInterval, ParsedTimeInterval, RawInput, RecurringIntervalFormValid,
    TemporalError, TemporalInputReceived, TimeIntervalProof,
};

/// A backend that performs every interval transition as an
/// [`Exchange`], for verifier `V`.
pub trait TemporalIntervalFactory<V: Verifier>:
    Send
    + Sync
    + Exchange<RawInput, ParsedDuration, V, Error = TemporalError>
    + Exchange<RawInput, ParsedRecurringInterval, V, Error = TemporalError>
    + Exchange<RawInput, ParsedTimeInterval, V, Error = TemporalError>
    + Exchange<OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput, V, Error = TemporalError>
where
    TemporalInputReceived: Witness<V>,
    DurationFormValid: Witness<V>,
    RecurringIntervalFormValid: Witness<V>,
    TimeIntervalProof: Witness<V>,
    OrderOffsetEndpointsPreconditions: Witness<V>,
    OrderOffsetEndpointsEstablished: Witness<V>,
{
}

impl<T, V> TemporalIntervalFactory<V> for T
where
    V: Verifier,
    TemporalInputReceived: Witness<V>,
    DurationFormValid: Witness<V>,
    RecurringIntervalFormValid: Witness<V>,
    TimeIntervalProof: Witness<V>,
    OrderOffsetEndpointsPreconditions: Witness<V>,
    OrderOffsetEndpointsEstablished: Witness<V>,
    T: Send
        + Sync
        + Exchange<RawInput, ParsedDuration, V, Error = TemporalError>
        + Exchange<RawInput, ParsedRecurringInterval, V, Error = TemporalError>
        + Exchange<RawInput, ParsedTimeInterval, V, Error = TemporalError>
        + Exchange<OrderOffsetEndpointsInput, OrderOffsetEndpointsOutput, V, Error = TemporalError>,
{
}
