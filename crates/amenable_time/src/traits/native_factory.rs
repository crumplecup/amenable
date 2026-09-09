//! Native higher-order factory bridge traits — ported from
//! `elicit_temporal::traits::native_conversion` / `native_zone` /
//! `native_interval` / (the formula factory in) `native_extension`.
//! Each per-family bridge's supertrait bundle *is* its `*_native`
//! exchange(s): a proven native carrier in, a proven native carrier
//! (or folded result) out. `Exchange` impls: backend.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    AdjustPrecisionLosslesslyNativeEstablished, AdjustPrecisionLosslesslyNativeInput,
    AdjustPrecisionLosslesslyNativeOutput, AttachNamedZoneNativeInput,
    ConfirmNamedZoneRevisionNativeOutput, DateTimeFormulaSemanticBundle,
    EvaluateDateTimeFormulaNativeEstablished, EvaluateDateTimeFormulaNativeOutput,
    IntervalEndpointOrderingBundle, NamedTimeZoneRevisionBundle, NormalizeToUtcNativeEstablished,
    NormalizeToUtcNativeOutput, OffsetDateTimeSemanticBundle, OrderOffsetEndpointsNativeInput,
    OrderOffsetEndpointsNativeOutput, ProvenDateTimeFormulaCarrier, ProvenOffsetDateTimeCarrier,
    ProvenZonedDateTimeCarrier, ResolveLocalDateTimeNativeEstablished,
    ResolveLocalDateTimeNativeInput, ResolveLocalDateTimeNativeOutput, TemporalCivilProps,
    TemporalDateTimeFormulaProps, TemporalError, TemporalExplicitTemporalFormProps,
    TemporalInputReceived, TemporalInstantProps, TemporalZoneProps,
    TruncateSubsecondsNativeEstablished, TruncateSubsecondsNativeInput,
    TruncateSubsecondsNativeOutput, ZonedDateTimeSemanticBundle,
};

/// Native `normalize_to_utc_native`, `strip_named_zone_native`, `adjust_precision_losslessly_native`, `truncate_subseconds_native` exchange(s).
pub trait TemporalNativeConversionFactory<V: Verifier>:
    Sized
    + TemporalInstantProps
    + TemporalZoneProps
    + Send
    + Sync
    + Exchange<
        ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        NormalizeToUtcNativeOutput<Self>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
        ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        V,
        Error = TemporalError,
    > + Exchange<
        AdjustPrecisionLosslesslyNativeInput<Self>,
        AdjustPrecisionLosslesslyNativeOutput<Self>,
        V,
        Error = TemporalError,
    > + Exchange<
        TruncateSubsecondsNativeInput<Self>,
        TruncateSubsecondsNativeOutput<Self>,
        V,
        Error = TemporalError,
    >
where
    AdjustPrecisionLosslesslyNativeEstablished: Witness<V>,
    NormalizeToUtcNativeEstablished: Witness<V>,
    OffsetDateTimeSemanticBundle: Witness<V>,
    TemporalInputReceived: Witness<V>,
    TruncateSubsecondsNativeEstablished: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalNativeConversionFactory<V> for B
where
    V: Verifier,
    B: TemporalInstantProps
        + TemporalZoneProps
        + Send
        + Sync
        + Exchange<
            ProvenOffsetDateTimeCarrier<B::OffsetDateTime>,
            NormalizeToUtcNativeOutput<B>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenZonedDateTimeCarrier<B::ZonedDateTime>,
            ProvenOffsetDateTimeCarrier<B::OffsetDateTime>,
            V,
            Error = TemporalError,
        > + Exchange<
            AdjustPrecisionLosslesslyNativeInput<B>,
            AdjustPrecisionLosslesslyNativeOutput<B>,
            V,
            Error = TemporalError,
        > + Exchange<
            TruncateSubsecondsNativeInput<B>,
            TruncateSubsecondsNativeOutput<B>,
            V,
            Error = TemporalError,
        >,
    AdjustPrecisionLosslesslyNativeEstablished: Witness<V>,
    NormalizeToUtcNativeEstablished: Witness<V>,
    OffsetDateTimeSemanticBundle: Witness<V>,
    TemporalInputReceived: Witness<V>,
    TruncateSubsecondsNativeEstablished: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}

/// Native `resolve_local_date_time_native`, `attach_named_zone_native`, `confirm_named_zone_revision_native` exchange(s).
pub trait TemporalNativeZoneFactory<V: Verifier>:
    Sized
    + TemporalCivilProps
    + TemporalInstantProps
    + TemporalZoneProps
    + Send
    + Sync
    + Exchange<
        ResolveLocalDateTimeNativeInput<Self>,
        ResolveLocalDateTimeNativeOutput<Self>,
        V,
        Error = TemporalError,
    > + Exchange<
        AttachNamedZoneNativeInput<Self>,
        ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
        ConfirmNamedZoneRevisionNativeOutput,
        V,
        Error = TemporalError,
    >
where
    NamedTimeZoneRevisionBundle: Witness<V>,
    ResolveLocalDateTimeNativeEstablished: Witness<V>,
    TemporalInputReceived: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalNativeZoneFactory<V> for B
where
    V: Verifier,
    B: TemporalCivilProps
        + TemporalInstantProps
        + TemporalZoneProps
        + Send
        + Sync
        + Exchange<
            ResolveLocalDateTimeNativeInput<B>,
            ResolveLocalDateTimeNativeOutput<B>,
            V,
            Error = TemporalError,
        > + Exchange<
            AttachNamedZoneNativeInput<B>,
            ProvenZonedDateTimeCarrier<B::ZonedDateTime>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenZonedDateTimeCarrier<B::ZonedDateTime>,
            ConfirmNamedZoneRevisionNativeOutput,
            V,
            Error = TemporalError,
        >,
    NamedTimeZoneRevisionBundle: Witness<V>,
    ResolveLocalDateTimeNativeEstablished: Witness<V>,
    TemporalInputReceived: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}

/// Native `order_offset_endpoints_native` exchange(s).
pub trait TemporalNativeIntervalFactory<V: Verifier>:
    Sized
    + TemporalInstantProps
    + Send
    + Sync
    + Exchange<
        OrderOffsetEndpointsNativeInput<Self>,
        OrderOffsetEndpointsNativeOutput,
        V,
        Error = TemporalError,
    >
where
    IntervalEndpointOrderingBundle: Witness<V>,
    TemporalInputReceived: Witness<V>,
{
}

impl<B, V> TemporalNativeIntervalFactory<V> for B
where
    V: Verifier,
    B: TemporalInstantProps
        + Send
        + Sync
        + Exchange<
            OrderOffsetEndpointsNativeInput<B>,
            OrderOffsetEndpointsNativeOutput,
            V,
            Error = TemporalError,
        >,
    IntervalEndpointOrderingBundle: Witness<V>,
    TemporalInputReceived: Witness<V>,
{
}

/// Native `evaluate_date_time_formula_native` exchange(s).
pub trait TemporalNativeDateTimeFormulaFactory<V: Verifier>:
    Sized
    + TemporalDateTimeFormulaProps
    + TemporalExplicitTemporalFormProps
    + Send
    + Sync
    + Exchange<
        ProvenDateTimeFormulaCarrier<Self::DateTimeFormula>,
        EvaluateDateTimeFormulaNativeOutput<Self>,
        V,
        Error = TemporalError,
    >
where
    DateTimeFormulaSemanticBundle: Witness<V>,
    EvaluateDateTimeFormulaNativeEstablished: Witness<V>,
{
}

impl<B, V> TemporalNativeDateTimeFormulaFactory<V> for B
where
    V: Verifier,
    B: TemporalDateTimeFormulaProps
        + TemporalExplicitTemporalFormProps
        + Send
        + Sync
        + Exchange<
            ProvenDateTimeFormulaCarrier<B::DateTimeFormula>,
            EvaluateDateTimeFormulaNativeOutput<B>,
            V,
            Error = TemporalError,
        >,
    DateTimeFormulaSemanticBundle: Witness<V>,
    EvaluateDateTimeFormulaNativeEstablished: Witness<V>,
{
}
