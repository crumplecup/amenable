//! `TemporalCivilNativeBridge`/`TemporalInstantNativeBridge`/`TemporalZoneNativeBridge` plus their aggregate `TemporalNativeBridge`.
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
    LocalDateTimeSemanticBundle, NamedTimeZoneSemanticBundle, OffsetDateTimeSemanticBundle,
    ProvenLocalDateTimeCarrier, ProvenNamedTimeZoneCarrier, ProvenOffsetDateTimeCarrier,
    ProvenZonedDateTimeCarrier, ReflectedLocalDateTime, ReflectedNamedTimeZone,
    ReflectedOffsetDateTime, ReflectedZonedDateTime, TemporalCivilProps, TemporalError,
    TemporalInstantProps, TemporalZoneProps, ZonedDateTimeSemanticBundle,
};

/// Realize and reflect native `local_date_time` carriers.
pub trait TemporalCivilNativeBridge<V: Verifier>:
    TemporalCivilProps
    + Send
    + Sync
    + Exchange<
        ReflectedLocalDateTime,
        ProvenLocalDateTimeCarrier<Self::LocalDateTime>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenLocalDateTimeCarrier<Self::LocalDateTime>,
        ReflectedLocalDateTime,
        V,
        Error = TemporalError,
    >
where
    LocalDateTimeSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalCivilNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalCivilProps
        + Send
        + Sync
        + Exchange<
            ReflectedLocalDateTime,
            ProvenLocalDateTimeCarrier<B::LocalDateTime>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenLocalDateTimeCarrier<B::LocalDateTime>,
            ReflectedLocalDateTime,
            V,
            Error = TemporalError,
        >,
    LocalDateTimeSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `offset_date_time` carriers.
pub trait TemporalInstantNativeBridge<V: Verifier>:
    TemporalInstantProps
    + Send
    + Sync
    + Exchange<
        ReflectedOffsetDateTime,
        ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenOffsetDateTimeCarrier<Self::OffsetDateTime>,
        ReflectedOffsetDateTime,
        V,
        Error = TemporalError,
    >
where
    OffsetDateTimeSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalInstantNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalInstantProps
        + Send
        + Sync
        + Exchange<
            ReflectedOffsetDateTime,
            ProvenOffsetDateTimeCarrier<B::OffsetDateTime>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenOffsetDateTimeCarrier<B::OffsetDateTime>,
            ReflectedOffsetDateTime,
            V,
            Error = TemporalError,
        >,
    OffsetDateTimeSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `named_time_zone`, `zoned_date_time` carriers.
pub trait TemporalZoneNativeBridge<V: Verifier>:
    TemporalInstantProps
    + TemporalZoneProps
    + Send
    + Sync
    + Exchange<
        ReflectedNamedTimeZone,
        ProvenNamedTimeZoneCarrier<Self::NamedTimeZone>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenNamedTimeZoneCarrier<Self::NamedTimeZone>,
        ReflectedNamedTimeZone,
        V,
        Error = TemporalError,
    > + Exchange<
        ReflectedZonedDateTime,
        ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenZonedDateTimeCarrier<Self::ZonedDateTime>,
        ReflectedZonedDateTime,
        V,
        Error = TemporalError,
    >
where
    NamedTimeZoneSemanticBundle: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalZoneNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalInstantProps
        + TemporalZoneProps
        + Send
        + Sync
        + Exchange<
            ReflectedNamedTimeZone,
            ProvenNamedTimeZoneCarrier<B::NamedTimeZone>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenNamedTimeZoneCarrier<B::NamedTimeZone>,
            ReflectedNamedTimeZone,
            V,
            Error = TemporalError,
        > + Exchange<
            ReflectedZonedDateTime,
            ProvenZonedDateTimeCarrier<B::ZonedDateTime>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenZonedDateTimeCarrier<B::ZonedDateTime>,
            ReflectedZonedDateTime,
            V,
            Error = TemporalError,
        >,
    NamedTimeZoneSemanticBundle: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}
/// Aggregate native bridge: TemporalCivilNativeBridge, TemporalInstantNativeBridge, TemporalZoneNativeBridge.
pub trait TemporalNativeBridge<V: Verifier>:
    TemporalCivilNativeBridge<V> + TemporalInstantNativeBridge<V> + TemporalZoneNativeBridge<V>
where
    LocalDateTimeSemanticBundle: Witness<V>,
    OffsetDateTimeSemanticBundle: Witness<V>,
    NamedTimeZoneSemanticBundle: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalCivilNativeBridge<V> + TemporalInstantNativeBridge<V> + TemporalZoneNativeBridge<V>,
    LocalDateTimeSemanticBundle: Witness<V>,
    OffsetDateTimeSemanticBundle: Witness<V>,
    NamedTimeZoneSemanticBundle: Witness<V>,
    ZonedDateTimeSemanticBundle: Witness<V>,
{
}
