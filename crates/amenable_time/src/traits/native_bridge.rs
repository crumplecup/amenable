//! Native descriptor/carrier bridge traits — ported from
//! `elicit_temporal::traits::native_bridge` / `native_span` /
//! `native_extension`. Each per-family bridge's supertrait bundle *is*
//! its `realize_x` / `reflect_x` exchange pair:
//!
//! - `realize_x` = `Exchange<Reflected<X>, Proven<X>Carrier<Self::X>, V>`
//! - `reflect_x`  = `Exchange<Proven<X>Carrier<Self::X>, Reflected<X>, V>`
//!
//! (`Reflected<X>` is `<X>Descriptor` + the semantic-bundle token; the
//! carrier holds the same token, so reflect is a genuine inverse.) The
//! aggregate bridges and their blanket impls mirror `elicit_temporal`.

use amenable_core::{Exchange, Verifier, Witness};

use crate::{
    DateTimeFormulaSemanticBundle, DurationSemanticBundle, ExplicitDurationSemanticBundle,
    ExplicitTemporalFormSemanticBundle, ExplicitTimeIntervalSemanticBundle,
    GroupedTimeScaleUnitSemanticBundle, LocalDateTimeSemanticBundle, NamedTimeZoneSemanticBundle,
    OffsetDateTimeSemanticBundle, ProvenDateTimeFormulaCarrier, ProvenDurationCarrier,
    ProvenExplicitDurationCarrier, ProvenExplicitTemporalFormCarrier,
    ProvenExplicitTimeIntervalCarrier, ProvenGroupedTimeScaleUnitCarrier,
    ProvenLocalDateTimeCarrier, ProvenNamedTimeZoneCarrier, ProvenOffsetDateTimeCarrier,
    ProvenQualifiedTemporalValueCarrier, ProvenRecurringIntervalCarrier, ProvenTemporalSetCarrier,
    ProvenTimeIntervalCarrier, ProvenZonedDateTimeCarrier, QualifiedTemporalValueSemanticBundle,
    RecurringIntervalSemanticBundle, ReflectedDateTimeFormula, ReflectedDuration,
    ReflectedExplicitDuration, ReflectedExplicitTemporalForm, ReflectedExplicitTimeInterval,
    ReflectedGroupedTimeScaleUnit, ReflectedLocalDateTime, ReflectedNamedTimeZone,
    ReflectedOffsetDateTime, ReflectedQualifiedTemporalValue, ReflectedRecurringInterval,
    ReflectedTemporalSet, ReflectedTimeInterval, ReflectedZonedDateTime, TemporalCivilProps,
    TemporalDateTimeFormulaProps, TemporalDurationProps, TemporalError,
    TemporalExplicitDurationProps, TemporalExplicitTemporalFormProps,
    TemporalExplicitTimeIntervalProps, TemporalGroupedTimeScaleUnitProps, TemporalInstantProps,
    TemporalQualifiedTemporalValueProps, TemporalRecurringIntervalProps, TemporalSetProps,
    TemporalSetSemanticBundle, TemporalTimeIntervalProps, TemporalZoneProps,
    TimeIntervalSemanticBundle, ZonedDateTimeSemanticBundle,
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

/// Realize and reflect native `qualified_temporal_value` carriers.
pub trait TemporalQualifiedTemporalValueNativeBridge<V: Verifier>:
    TemporalQualifiedTemporalValueProps
    + Send
    + Sync
    + Exchange<
        ReflectedQualifiedTemporalValue,
        ProvenQualifiedTemporalValueCarrier<Self::QualifiedTemporalValue>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenQualifiedTemporalValueCarrier<Self::QualifiedTemporalValue>,
        ReflectedQualifiedTemporalValue,
        V,
        Error = TemporalError,
    >
where
    QualifiedTemporalValueSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalQualifiedTemporalValueNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalQualifiedTemporalValueProps
        + Send
        + Sync
        + Exchange<
            ReflectedQualifiedTemporalValue,
            ProvenQualifiedTemporalValueCarrier<B::QualifiedTemporalValue>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenQualifiedTemporalValueCarrier<B::QualifiedTemporalValue>,
            ReflectedQualifiedTemporalValue,
            V,
            Error = TemporalError,
        >,
    QualifiedTemporalValueSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `explicit_temporal_form` carriers.
pub trait TemporalExplicitTemporalFormNativeBridge<V: Verifier>:
    TemporalExplicitTemporalFormProps
    + Send
    + Sync
    + Exchange<
        ReflectedExplicitTemporalForm,
        ProvenExplicitTemporalFormCarrier<Self::ExplicitTemporalForm>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenExplicitTemporalFormCarrier<Self::ExplicitTemporalForm>,
        ReflectedExplicitTemporalForm,
        V,
        Error = TemporalError,
    >
where
    ExplicitTemporalFormSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalExplicitTemporalFormNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalExplicitTemporalFormProps
        + Send
        + Sync
        + Exchange<
            ReflectedExplicitTemporalForm,
            ProvenExplicitTemporalFormCarrier<B::ExplicitTemporalForm>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenExplicitTemporalFormCarrier<B::ExplicitTemporalForm>,
            ReflectedExplicitTemporalForm,
            V,
            Error = TemporalError,
        >,
    ExplicitTemporalFormSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `explicit_duration` carriers.
pub trait TemporalExplicitDurationNativeBridge<V: Verifier>:
    TemporalExplicitDurationProps
    + Send
    + Sync
    + Exchange<
        ReflectedExplicitDuration,
        ProvenExplicitDurationCarrier<Self::ExplicitDuration>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenExplicitDurationCarrier<Self::ExplicitDuration>,
        ReflectedExplicitDuration,
        V,
        Error = TemporalError,
    >
where
    ExplicitDurationSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalExplicitDurationNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalExplicitDurationProps
        + Send
        + Sync
        + Exchange<
            ReflectedExplicitDuration,
            ProvenExplicitDurationCarrier<B::ExplicitDuration>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenExplicitDurationCarrier<B::ExplicitDuration>,
            ReflectedExplicitDuration,
            V,
            Error = TemporalError,
        >,
    ExplicitDurationSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `explicit_time_interval` carriers.
pub trait TemporalExplicitTimeIntervalNativeBridge<V: Verifier>:
    TemporalExplicitTimeIntervalProps
    + Send
    + Sync
    + Exchange<
        ReflectedExplicitTimeInterval,
        ProvenExplicitTimeIntervalCarrier<Self::ExplicitTimeInterval>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenExplicitTimeIntervalCarrier<Self::ExplicitTimeInterval>,
        ReflectedExplicitTimeInterval,
        V,
        Error = TemporalError,
    >
where
    ExplicitTimeIntervalSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalExplicitTimeIntervalNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalExplicitTimeIntervalProps
        + Send
        + Sync
        + Exchange<
            ReflectedExplicitTimeInterval,
            ProvenExplicitTimeIntervalCarrier<B::ExplicitTimeInterval>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenExplicitTimeIntervalCarrier<B::ExplicitTimeInterval>,
            ReflectedExplicitTimeInterval,
            V,
            Error = TemporalError,
        >,
    ExplicitTimeIntervalSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `grouped_time_scale_unit` carriers.
pub trait TemporalGroupedTimeScaleUnitNativeBridge<V: Verifier>:
    TemporalGroupedTimeScaleUnitProps
    + Send
    + Sync
    + Exchange<
        ReflectedGroupedTimeScaleUnit,
        ProvenGroupedTimeScaleUnitCarrier<Self::GroupedTimeScaleUnit>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenGroupedTimeScaleUnitCarrier<Self::GroupedTimeScaleUnit>,
        ReflectedGroupedTimeScaleUnit,
        V,
        Error = TemporalError,
    >
where
    GroupedTimeScaleUnitSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalGroupedTimeScaleUnitNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalGroupedTimeScaleUnitProps
        + Send
        + Sync
        + Exchange<
            ReflectedGroupedTimeScaleUnit,
            ProvenGroupedTimeScaleUnitCarrier<B::GroupedTimeScaleUnit>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenGroupedTimeScaleUnitCarrier<B::GroupedTimeScaleUnit>,
            ReflectedGroupedTimeScaleUnit,
            V,
            Error = TemporalError,
        >,
    GroupedTimeScaleUnitSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `temporal_set` carriers.
pub trait TemporalSetNativeBridge<V: Verifier>:
    TemporalSetProps
    + Send
    + Sync
    + Exchange<
        ReflectedTemporalSet,
        ProvenTemporalSetCarrier<Self::TemporalSet>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenTemporalSetCarrier<Self::TemporalSet>,
        ReflectedTemporalSet,
        V,
        Error = TemporalError,
    >
where
    TemporalSetSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalSetNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalSetProps
        + Send
        + Sync
        + Exchange<
            ReflectedTemporalSet,
            ProvenTemporalSetCarrier<B::TemporalSet>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenTemporalSetCarrier<B::TemporalSet>,
            ReflectedTemporalSet,
            V,
            Error = TemporalError,
        >,
    TemporalSetSemanticBundle: Witness<V>,
{
}

/// Realize and reflect native `date_time_formula` carriers.
pub trait TemporalDateTimeFormulaNativeBridge<V: Verifier>:
    TemporalDateTimeFormulaProps
    + Send
    + Sync
    + Exchange<
        ReflectedDateTimeFormula,
        ProvenDateTimeFormulaCarrier<Self::DateTimeFormula>,
        V,
        Error = TemporalError,
    > + Exchange<
        ProvenDateTimeFormulaCarrier<Self::DateTimeFormula>,
        ReflectedDateTimeFormula,
        V,
        Error = TemporalError,
    >
where
    DateTimeFormulaSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalDateTimeFormulaNativeBridge<V> for B
where
    V: Verifier,
    B: TemporalDateTimeFormulaProps
        + Send
        + Sync
        + Exchange<
            ReflectedDateTimeFormula,
            ProvenDateTimeFormulaCarrier<B::DateTimeFormula>,
            V,
            Error = TemporalError,
        > + Exchange<
            ProvenDateTimeFormulaCarrier<B::DateTimeFormula>,
            ReflectedDateTimeFormula,
            V,
            Error = TemporalError,
        >,
    DateTimeFormulaSemanticBundle: Witness<V>,
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

/// Aggregate native bridge: TemporalQualifiedTemporalValueNativeBridge, TemporalExplicitTemporalFormNativeBridge, TemporalExplicitDurationNativeBridge, TemporalExplicitTimeIntervalNativeBridge, TemporalGroupedTimeScaleUnitNativeBridge, TemporalSetNativeBridge, TemporalDateTimeFormulaNativeBridge.
pub trait TemporalNativeExtensionBridge<V: Verifier>:
    TemporalQualifiedTemporalValueNativeBridge<V>
    + TemporalExplicitTemporalFormNativeBridge<V>
    + TemporalExplicitDurationNativeBridge<V>
    + TemporalExplicitTimeIntervalNativeBridge<V>
    + TemporalGroupedTimeScaleUnitNativeBridge<V>
    + TemporalSetNativeBridge<V>
    + TemporalDateTimeFormulaNativeBridge<V>
where
    QualifiedTemporalValueSemanticBundle: Witness<V>,
    ExplicitTemporalFormSemanticBundle: Witness<V>,
    ExplicitDurationSemanticBundle: Witness<V>,
    ExplicitTimeIntervalSemanticBundle: Witness<V>,
    GroupedTimeScaleUnitSemanticBundle: Witness<V>,
    TemporalSetSemanticBundle: Witness<V>,
    DateTimeFormulaSemanticBundle: Witness<V>,
{
}

impl<B, V> TemporalNativeExtensionBridge<V> for B
where
    V: Verifier,
    B: TemporalQualifiedTemporalValueNativeBridge<V>
        + TemporalExplicitTemporalFormNativeBridge<V>
        + TemporalExplicitDurationNativeBridge<V>
        + TemporalExplicitTimeIntervalNativeBridge<V>
        + TemporalGroupedTimeScaleUnitNativeBridge<V>
        + TemporalSetNativeBridge<V>
        + TemporalDateTimeFormulaNativeBridge<V>,
    QualifiedTemporalValueSemanticBundle: Witness<V>,
    ExplicitTemporalFormSemanticBundle: Witness<V>,
    ExplicitDurationSemanticBundle: Witness<V>,
    ExplicitTimeIntervalSemanticBundle: Witness<V>,
    GroupedTimeScaleUnitSemanticBundle: Witness<V>,
    TemporalSetSemanticBundle: Witness<V>,
    DateTimeFormulaSemanticBundle: Witness<V>,
{
}
