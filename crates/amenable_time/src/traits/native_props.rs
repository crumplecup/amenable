//! Associated native-carrier trait families for temporal backends —
//! ported from `elicit_temporal::traits::native_props`, one-to-one.
//!
//! Each backend exposes its real upstream carrier types here instead of
//! the framework inventing replacement runtime values. The amenable-flavored
//! change: every associated type is bound `: Evidence`, so a native
//! carrier can ride as the [`Sidecar::Primary`](amenable_core::Sidecar)
//! of a [`ProvenTemporalCarrier`](crate::ProvenTemporalCarrier) — a thin
//! `#[derive(Evidence)]` newtype over the upstream type, not a synthetic
//! replacement.

use amenable_core::Evidence;

/// Civil temporal carriers that do not by themselves identify a fixed instant.
///
/// These associated types cover date and local-time families whose semantics
/// remain civil or local until additional zone or offset law is applied.
pub trait TemporalCivilProps {
    /// Backend-native calendar date carrier.
    type CalendarDate: Evidence;

    /// Backend-native reduced-precision calendar date carrier.
    type ReducedCalendarDate: Evidence;

    /// Backend-native ordinal date carrier.
    type OrdinalDate: Evidence;

    /// Backend-native week date carrier.
    type WeekDate: Evidence;

    /// Backend-native local time-of-day carrier.
    type LocalTime: Evidence;

    /// Backend-native reduced local time-of-day carrier.
    type ReducedLocalTime: Evidence;

    /// Backend-native combined local date-time carrier.
    type LocalDateTime: Evidence;
}

/// Instant-bearing temporal carriers.
///
/// `Instant` is kept distinct from `OffsetDateTime` because some upstream
/// libraries expose an absolute timestamp type that is not identical to an
/// offset-bearing civil representation.
pub trait TemporalInstantProps {
    /// Backend-native UTC offset carrier.
    type UtcOffset: Evidence;

    /// Backend-native offset date-time carrier.
    type OffsetDateTime: Evidence;

    /// Backend-native fixed-instant carrier.
    type Instant: Evidence;
}

/// Named-zone and zone-attached temporal carriers.
///
/// This family remains separate from the civil and instant families because
/// some backends lawfully support offsets but do not have an upstream named-
/// zone carrier.
pub trait TemporalZoneProps {
    /// Backend-native named time zone carrier.
    type NamedTimeZone: Evidence;

    /// Backend-native named-zone-attached date-time carrier.
    type ZonedDateTime: Evidence;
}

/// Native duration carriers.
pub trait TemporalDurationProps {
    /// Backend-native duration carrier.
    type Duration: Evidence;
}

/// Native interval carriers.
pub trait TemporalTimeIntervalProps {
    /// Backend-native interval carrier.
    type TimeInterval: Evidence;
}

/// Native recurring-interval carriers.
///
/// A lawful recurring-interval carrier necessarily presupposes a lawful
/// interval carrier, since recurrence wraps an interval payload.
pub trait TemporalRecurringIntervalProps: TemporalTimeIntervalProps {
    /// Backend-native recurring interval carrier.
    type RecurringInterval: Evidence;
}

/// Aggregate span-carrier family for backends that support all span forms.
pub trait TemporalSpanProps:
    TemporalDurationProps + TemporalTimeIntervalProps + TemporalRecurringIntervalProps
{
}

impl<T> TemporalSpanProps for T where
    T: TemporalDurationProps + TemporalTimeIntervalProps + TemporalRecurringIntervalProps
{
}

/// Backend-native qualified temporal value carriers.
pub trait TemporalQualifiedTemporalValueProps {
    /// Backend-native qualified temporal value carrier.
    type QualifiedTemporalValue: Evidence;
}

/// Backend-native explicit temporal form carriers.
pub trait TemporalExplicitTemporalFormProps {
    /// Backend-native explicit temporal form carrier.
    type ExplicitTemporalForm: Evidence;
}

/// Backend-native explicit duration carriers.
pub trait TemporalExplicitDurationProps {
    /// Backend-native explicit duration carrier.
    type ExplicitDuration: Evidence;
}

/// Backend-native explicit time-interval carriers.
pub trait TemporalExplicitTimeIntervalProps {
    /// Backend-native explicit time interval carrier.
    type ExplicitTimeInterval: Evidence;
}

/// Backend-native grouped time-scale-unit carriers.
pub trait TemporalGroupedTimeScaleUnitProps {
    /// Backend-native grouped time scale unit carrier.
    type GroupedTimeScaleUnit: Evidence;
}

/// Backend-native temporal-set carriers.
pub trait TemporalSetProps {
    /// Backend-native temporal set carrier.
    type TemporalSet: Evidence;
}

/// Backend-native date-time formula carriers.
pub trait TemporalDateTimeFormulaProps {
    /// Backend-native date-time formula carrier.
    type DateTimeFormula: Evidence;
}

/// Aggregate higher-order ISO 8601-2 and CalConnect extension carriers.
///
/// A backend should expose a native carrier here only when it has a lawful,
/// meaningful upstream representation. Descriptor-only forms should remain in
/// the neutral accord rather than forcing synthetic runtime wrappers.
pub trait TemporalExtensionProps:
    TemporalQualifiedTemporalValueProps
    + TemporalExplicitTemporalFormProps
    + TemporalExplicitDurationProps
    + TemporalExplicitTimeIntervalProps
    + TemporalGroupedTimeScaleUnitProps
    + TemporalSetProps
    + TemporalDateTimeFormulaProps
{
}

impl<T> TemporalExtensionProps for T where
    T: TemporalQualifiedTemporalValueProps
        + TemporalExplicitTemporalFormProps
        + TemporalExplicitDurationProps
        + TemporalExplicitTimeIntervalProps
        + TemporalGroupedTimeScaleUnitProps
        + TemporalSetProps
        + TemporalDateTimeFormulaProps
{
}

/// Umbrella associated-type family for a fully-capable temporal backend.
///
/// Narrower backend capabilities should usually implement the relevant
/// subfamilies directly. This umbrella trait is the composition point for
/// backends that provide the full native temporal surface.
pub trait TemporalNativeProps:
    TemporalCivilProps
    + TemporalInstantProps
    + TemporalZoneProps
    + TemporalSpanProps
    + TemporalExtensionProps
{
}

impl<T> TemporalNativeProps for T where
    T: TemporalCivilProps
        + TemporalInstantProps
        + TemporalZoneProps
        + TemporalSpanProps
        + TemporalExtensionProps
{
}
