//! The 7 CalConnect/ISO 8601-2 extension-family native bridges plus their aggregate `TemporalNativeExtensionBridge`.
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
    DateTimeFormulaSemanticBundle, ExplicitDurationSemanticBundle,
    ExplicitTemporalFormSemanticBundle, ExplicitTimeIntervalSemanticBundle,
    GroupedTimeScaleUnitSemanticBundle, ProvenDateTimeFormulaCarrier,
    ProvenExplicitDurationCarrier, ProvenExplicitTemporalFormCarrier,
    ProvenExplicitTimeIntervalCarrier, ProvenGroupedTimeScaleUnitCarrier,
    ProvenQualifiedTemporalValueCarrier, ProvenTemporalSetCarrier,
    QualifiedTemporalValueSemanticBundle, ReflectedDateTimeFormula, ReflectedExplicitDuration,
    ReflectedExplicitTemporalForm, ReflectedExplicitTimeInterval, ReflectedGroupedTimeScaleUnit,
    ReflectedQualifiedTemporalValue, ReflectedTemporalSet, TemporalDateTimeFormulaProps,
    TemporalError, TemporalExplicitDurationProps, TemporalExplicitTemporalFormProps,
    TemporalExplicitTimeIntervalProps, TemporalGroupedTimeScaleUnitProps,
    TemporalQualifiedTemporalValueProps, TemporalSetProps, TemporalSetSemanticBundle,
};

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
