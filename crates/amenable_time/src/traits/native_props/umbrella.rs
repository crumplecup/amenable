//! The `TemporalNativeProps` umbrella trait for a fully-capable backend.
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

use crate::{
    TemporalCivilProps, TemporalExtensionProps, TemporalInstantProps, TemporalSpanProps,
    TemporalZoneProps,
};

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
