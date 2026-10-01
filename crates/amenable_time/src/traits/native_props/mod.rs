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
//!
//! Split by real domain: `primary` (civil/instant/zone), `span`
//! (duration/interval/recurring-interval), `extension` (the CalConnect/
//! ISO 8601-2 family), `umbrella` (`TemporalNativeProps`, the
//! fully-capable-backend composition point over all of the above) —
//! matching the domain split already used by `traits::native_bridge`.

mod extension;
mod primary;
mod span;
mod umbrella;

pub use extension::{
    TemporalDateTimeFormulaProps, TemporalExplicitDurationProps, TemporalExplicitTemporalFormProps,
    TemporalExplicitTimeIntervalProps, TemporalExtensionProps, TemporalGroupedTimeScaleUnitProps,
    TemporalQualifiedTemporalValueProps, TemporalSetProps,
};
pub use primary::{TemporalCivilProps, TemporalInstantProps, TemporalZoneProps};
pub use span::{
    TemporalDurationProps, TemporalRecurringIntervalProps, TemporalSpanProps,
    TemporalTimeIntervalProps,
};
pub use umbrella::TemporalNativeProps;
