//! Duration/interval/recurring-interval native-carrier trait families.
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
