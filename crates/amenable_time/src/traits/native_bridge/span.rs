//! `TemporalDurationNativeBridge`/`TemporalTimeIntervalNativeBridge`/`TemporalRecurringIntervalNativeBridge` plus their aggregate `TemporalNativeSpanBridge`.
//!
//! Ported from `elicit_temporal::traits::native_bridge` / `native_span`
//! / `native_extension`. Each per-family bridge's supertrait bundle *is*
//! its `realize_x` / `reflect_x` exchange pair:
//!
//! - `realize_x` = `Exchange<Reflected<X>, Proven<X>Carrier<Self::X>, V>`
//! - `reflect_x`  = `Exchange<Proven<X>Carrier<Self::X>, Reflected<X>, V>`
//!
//! (`Reflected<X>` is `<X>Descriptor` + the semantic-bundle token; the
//! carrier holds the same token, so reflect is a genuine inverse.) The
//! aggregate bridge and its blanket impl mirror `elicit_temporal`.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    DurationSemanticBundle, ProvenDurationCarrier, ProvenRecurringIntervalCarrier,
    ProvenTimeIntervalCarrier, RecurringIntervalSemanticBundle, ReflectedDuration,
    ReflectedRecurringInterval, ReflectedTimeInterval, TemporalDurationProps, TemporalError,
    TemporalRecurringIntervalProps, TemporalTimeIntervalProps, TimeIntervalSemanticBundle,
};

/// Realize and reflect native `duration` carriers.
pub trait TemporalDurationNativeBridge<V: Verifier>:
    TemporalDurationProps
    + Send
    + Sync
    + Exchange<ReflectedDuration, ProvenDurationCarrier<Self::Duration>, V, Error = TemporalError>
    + Exchange<ProvenDurationCarrier<Self::Duration>, ReflectedDuration, V, Error = TemporalError>
where
    DurationSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `time_interval` carriers.
pub trait TemporalTimeIntervalNativeBridge<V: Verifier>:
    TemporalTimeIntervalProps
    + Send
    + Sync
    + Exchange<
        ReflectedTimeInterval,
        ProvenTimeIntervalCarrier<Self::TimeInterval>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenTimeIntervalCarrier<Self::TimeInterval>,
        ReflectedTimeInterval,
        V,
        Error = TemporalError,
    >
where
    TimeIntervalSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `recurring_interval` carriers.
pub trait TemporalRecurringIntervalNativeBridge<V: Verifier>:
    TemporalRecurringIntervalProps
    + Send
    + Sync
    + Exchange<
        ReflectedRecurringInterval,
        ProvenRecurringIntervalCarrier<Self::RecurringInterval>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenRecurringIntervalCarrier<Self::RecurringInterval>,
        ReflectedRecurringInterval,
        V,
        Error = TemporalError,
    >
where
    RecurringIntervalSemanticBundle: Witness<V>,
{
}

/// Aggregate native bridge: TemporalDurationNativeBridge, TemporalTimeIntervalNativeBridge, TemporalRecurringIntervalNativeBridge.
pub trait TemporalNativeSpanBridge<V: Verifier>:
    TemporalDurationNativeBridge<V>
    + TemporalTimeIntervalNativeBridge<V>
    + TemporalRecurringIntervalNativeBridge<V>
where
    DurationSemanticBundle: Witness<V>,
    TimeIntervalSemanticBundle: Witness<V>,
    RecurringIntervalSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalDurationNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalDurationProps
        + Send
        + Sync
        + Exchange<ReflectedDuration, ProvenDurationCarrier<B::Duration>, V, Error = TemporalError>
        + Exchange<ProvenDurationCarrier<B::Duration>, ReflectedDuration, V, Error = TemporalError>,
    DurationSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalTimeIntervalNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalTimeIntervalProps
        + Send
        + Sync
        + Exchange<
            ReflectedTimeInterval,
            ProvenTimeIntervalCarrier<B::TimeInterval>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenTimeIntervalCarrier<B::TimeInterval>,
            ReflectedTimeInterval,
            V,
            Error = TemporalError,
        >,
    TimeIntervalSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalRecurringIntervalNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalRecurringIntervalProps
        + Send
        + Sync
        + Exchange<
            ReflectedRecurringInterval,
            ProvenRecurringIntervalCarrier<B::RecurringInterval>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenRecurringIntervalCarrier<B::RecurringInterval>,
            ReflectedRecurringInterval,
            V,
            Error = TemporalError,
        >,
    RecurringIntervalSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalNativeSpanBridge<V> for B
where
    V: Verifier,
    B: TemporalDurationNativeBridge<V>
        + TemporalTimeIntervalNativeBridge<V>
        + TemporalRecurringIntervalNativeBridge<V>,
    DurationSemanticBundle: Witness<V>,
    TimeIntervalSemanticBundle: Witness<V>,
    RecurringIntervalSemanticBundle: Witness<V>,
{
}
