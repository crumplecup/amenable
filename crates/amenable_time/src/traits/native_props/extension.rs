//! CalConnect/ISO 8601-2 extension-family native-carrier trait families.
//!
//! Associated native-carrier trait families for temporal backends --
//! ported from `elicit_temporal::traits::native_props`, one-to-one.
//!
//! Each backend exposes its real upstream carrier types here instead of
//! the framework inventing replacement runtime values. The amenable-flavored
//! change: every associated type is bound `: Evidence`, so a native
//! carrier can ride as the [`Sidecar::Primary`](amenable_core::Sidecar)
//! of a [`ProvenTemporalCarrier`](crate::ProvenTemporalCarrier) -- a thin
//! `#[derive(Evidence)]` newtype over the upstream type, not a synthetic
//! replacement.

use amenable_core::Evidence;

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
