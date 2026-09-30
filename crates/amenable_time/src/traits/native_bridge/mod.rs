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
//!
//! Split one file per aggregate: `primary` (civil/instant/zone +
//! `TemporalNativeBridge`), `span` (duration/time-interval/recurring-
//! interval + `TemporalNativeSpanBridge`), and `extension` (the 7
//! CalConnect/ISO 8601-2 family bridges + `TemporalNativeExtensionBridge`)
//! — each aggregate lives with its own constituent bridges, so no
//! cross-file supertrait references are needed. (Not named `core`:
//! that would shadow the `core` crate for any sibling module reaching
//! for `core::fmt`/etc.)

mod extension;
mod primary;
mod span;

pub use extension::{
    TemporalDateTimeFormulaNativeBridge, TemporalExplicitDurationNativeBridge,
    TemporalExplicitTemporalFormNativeBridge, TemporalExplicitTimeIntervalNativeBridge,
    TemporalGroupedTimeScaleUnitNativeBridge, TemporalNativeExtensionBridge,
    TemporalQualifiedTemporalValueNativeBridge, TemporalSetNativeBridge,
};
pub use primary::{
    TemporalCivilNativeBridge, TemporalInstantNativeBridge, TemporalNativeBridge,
    TemporalZoneNativeBridge,
};
pub use span::{
    TemporalDurationNativeBridge, TemporalNativeSpanBridge, TemporalRecurringIntervalNativeBridge,
    TemporalTimeIntervalNativeBridge,
};
